//! Servidor HTTP REST para Avila Diffusion

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::AvilaDiffusion;
use image::ImageOutputFormat;
use std::io::Cursor;
use base64::Engine;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Text2ImgRequest {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    #[serde(default = "default_width")]
    pub width: usize,
    #[serde(default = "default_height")]
    pub height: usize,
    #[serde(default = "default_steps")]
    pub steps: usize,
    #[serde(default = "default_guidance", alias = "cfg_scale")]
    pub guidance: f32,
    pub seed: Option<i64>,
    #[serde(default = "default_lang")]
    pub lang: String,
}

fn default_lang() -> String { "pt".to_string() }

fn default_width() -> usize { 512 }
fn default_height() -> usize { 512 }
fn default_steps() -> usize { 25 }
fn default_guidance() -> f32 { 7.5 }

#[derive(Serialize)]
pub struct ImageResponse {
    pub images: Vec<String>,
    pub info: GenerationInfo,
}

#[derive(Serialize)]
pub struct GenerationInfo {
    pub prompt: String,
    pub width: usize,
    pub height: usize,
    pub steps: usize,
    pub guidance: f32,
    pub seed: Option<i64>,
    pub time_taken: f32,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub model: String,
}

pub async fn run_server(port: u16) -> anyhow::Result<()> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║         🎨 AVILA DIFFUSION - Gerador de Imagens IA       ║");
    println!("║                  100% Soberania Tecnológica              ║");
    println!("║              🇧🇷 🇫🇷 🇩🇪  Multilíngue (PT/FR/DE)             ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("🚀 Inicializando servidor...");

    let diffusion = Arc::new(Mutex::new(AvilaDiffusion::new()?));
    println!("✅ Modelo carregado");
    println!();
    println!("🌐 Servidor rodando em http://0.0.0.0:{}", port);
    println!("📖 Documentação: http://localhost:{}/", port);
    println!("🗣️  Idiomas: Português (pt), Français (fr), Deutsch (de)");
    println!();
    println!("Pressione Ctrl+C para parar");
    println!();    // Rota raiz - documentação HTML com suporte a idiomas
    let index = warp::path::end()
        .and(warp::query::<HashMap<String, String>>())
        .map(|params: HashMap<String, String>| {
            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("pt");
            warp::reply::html(get_html_for_lang(lang))
        });

    // POST /txt2img
    let txt2img = warp::path("txt2img")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_diffusion(diffusion.clone()))
        .and_then(handle_txt2img);

    // GET /health com suporte a idiomas
    let health = warp::path("health")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .map(|params: HashMap<String, String>| {
            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("pt");
            let model_name = match lang {
                "fr" => "Avila Diffusion v1.0 - Générateur d'images IA",
                "de" => "Avila Diffusion v1.0 - KI-Bildgenerator",
                _ => "Avila Diffusion v1.0 - Gerador de Imagens IA",
            };

            warp::reply::json(&HealthResponse {
                status: "healthy".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                model: model_name.to_string(),
            })
        });

    // Servir interface profissional unificada
    let current_dir = std::env::current_dir().unwrap_or_default();
    let html_path = if std::path::Path::new("./frontend/index.html").exists() {
        "./frontend/index.html"
    } else {
        "d:/stable-diffusion-webui/avila-diffusion/frontend/index.html"
    };

    let app_html = std::fs::read_to_string(html_path)
        .unwrap_or_else(|e| {
            println!("⚠️  Frontend não encontrado!");
            println!("   Procurando em: {:?}", current_dir);
            println!("   Erro: {}", e);
            "<h1>Frontend não encontrado</h1><p>Arquivo nao encontrado</p>".to_string()
        });

    let app = warp::path("app")
        .and(warp::path::end())
        .map(move || warp::reply::html(app_html.clone()));

    let routes = index
        .or(app)
        .or(txt2img)
        .or(health);

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║               🚀 AVILA DIFFUSION SERVER                   ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("🎨 Interface Principal: http://localhost:{}/app", port);
    println!("📖 Documentação API:    http://localhost:{}/", port);
    println!("❤️  Health Check:       http://localhost:{}/health", port);
    println!("✨ Endpoint txt2img:    POST http://localhost:{}/txt2img", port);
    println!();

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;

    Ok(())
}

fn with_diffusion(
    diffusion: Arc<Mutex<AvilaDiffusion>>,
) -> impl Filter<Extract = (Arc<Mutex<AvilaDiffusion>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || diffusion.clone())
}

async fn handle_txt2img(
    body: Text2ImgRequest,
    diffusion: Arc<Mutex<AvilaDiffusion>>,
) -> Result<impl Reply, Rejection> {
    let start = std::time::Instant::now();

    let mut diff = diffusion.lock().unwrap();
    let seed_opt = body.seed.filter(|value| *value >= 0).map(|value| value as u64);

    match diff.text_to_image(
        &body.prompt,
        body.negative_prompt.as_deref(),
        body.width,
        body.height,
        body.steps,
        body.guidance,
        seed_opt,
    ) {
        Ok(image) => {
            // Converter imagem para PNG base64
            let mut buf = Vec::new();
            {
                let mut cursor = Cursor::new(&mut buf);
                image.write_to(&mut cursor, ImageOutputFormat::Png)
                    .map_err(|e| {
                        eprintln!("Erro ao codificar PNG: {}", e);
                        warp::reject()
                    })?;
            }
            let base64 = base64::engine::general_purpose::STANDARD.encode(&buf);

            let time_taken = start.elapsed().as_secs_f32();

            let response = ImageResponse {
                images: vec![base64],
                info: GenerationInfo {
                    prompt: body.prompt,
                    width: body.width,
                    height: body.height,
                    steps: body.steps,
                    guidance: body.guidance,
                    seed: body.seed,
                    time_taken,
                },
            };

            Ok(warp::reply::json(&response))
        }
        Err(e) => {
            eprintln!("Erro ao gerar imagem: {}", e);
            Err(warp::reject())
        }
    }
}

fn get_html_for_lang(lang: &str) -> String {
    match lang {
        "fr" => INDEX_HTML_FR.to_string(),
        "de" => INDEX_HTML_DE.to_string(),
        _ => INDEX_HTML_PT.to_string(),
    }
}

const INDEX_HTML_PT: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>Avila Diffusion API</title>
    <style>
        body { font-family: Arial; max-width: 900px; margin: 50px auto; padding: 20px; background: #f5f5f5; }
        h1 { color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }
        .endpoint { background: white; padding: 20px; margin: 20px 0; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        code { background: #34495e; color: #ecf0f1; padding: 2px 8px; border-radius: 4px; font-family: 'Courier New', monospace; }
        pre { background: #2c3e50; color: #ecf0f1; padding: 15px; border-radius: 8px; overflow-x: auto; }
        .tag { background: #3498db; color: white; padding: 4px 12px; border-radius: 12px; font-size: 12px; font-weight: bold; }
        ul { line-height: 1.8; }
    </style>
</head>
<body>
    <h1>🎨 Avila Diffusion API</h1>
    <p><strong>Gerador de imagens IA 100% proprietário</strong></p>
    <p>Soberania tecnológica total - Sistema próprio de IA</p>

    <h2>Endpoints Disponíveis</h2>

    <div class="endpoint">
        <h3><span class="tag">POST</span> /txt2img</h3>
        <p><strong>Gera imagem a partir de texto (prompt)</strong></p>
        <p>Request body:</p>
        <pre><code>{
  "prompt": "a beautiful landscape, mountains, sunset, 4k",
  "negative_prompt": "blurry, low quality",
  "width": 512,
  "height": 512,
  "steps": 25
}</code></pre>
        <p>Response:</p>
        <pre><code>{
  "images": ["base64_encoded_png_image"],
  "info": {
    "prompt": "a beautiful landscape...",
    "width": 512,
    "height": 512,
    "time_taken": 0.123
  }
}</code></pre>
    </div>

    <div class="endpoint">
        <h3><span class="tag">GET</span> /health</h3>
        <p><strong>Verifica status do servidor</strong></p>
        <p>Response:</p>
        <pre><code>{
  "status": "healthy",
  "version": "1.0.0",
  "model": "Avila Diffusion v1.0"
}</code></pre>
    </div>

    <h2>Características</h2>
    <ul>
        <li>✅ Sistema 100% proprietário</li>
        <li>✅ Geração rápida de imagens</li>
        <li>✅ API REST simples</li>
        <li>✅ Formato PNG com base64</li>
        <li>✅ Zero dependências externas de IA</li>
    </ul>

    <h2>Exemplo de Uso (curl)</h2>
    <pre><code>curl -X POST http://localhost:7860/txt2img \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "a beautiful sunset over mountains",
    "width": 512,
    "height": 512
  }'</code></pre>

    <footer style="margin-top: 50px; padding-top: 20px; border-top: 1px solid #ddd; color: #7f8c8d; text-align: center;">
        <p><strong>Avila Diffusion v1.0</strong> - Soberania Tecnológica 🇧🇷</p>
        <p style="font-size: 12px;">
            <a href="?lang=pt">🇧🇷 Português</a> |
            <a href="?lang=fr">🇫🇷 Français</a> |
            <a href="?lang=de">🇩🇪 Deutsch</a>
        </p>
    </footer>
</body>
</html>"#;

const INDEX_HTML_FR: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>API Avila Diffusion</title>
    <style>
        body { font-family: Arial; max-width: 900px; margin: 50px auto; padding: 20px; background: #f5f5f5; }
        h1 { color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }
        .endpoint { background: white; padding: 20px; margin: 20px 0; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        code { background: #34495e; color: #ecf0f1; padding: 2px 8px; border-radius: 4px; font-family: 'Courier New', monospace; }
        pre { background: #2c3e50; color: #ecf0f1; padding: 15px; border-radius: 8px; overflow-x: auto; }
        .tag { background: #3498db; color: white; padding: 4px 12px; border-radius: 12px; font-size: 12px; font-weight: bold; }
        ul { line-height: 1.8; }
    </style>
</head>
<body>
    <h1>🎨 API Avila Diffusion</h1>
    <p><strong>Générateur d'images IA 100% propriétaire</strong></p>
    <p>Souveraineté technologique totale - Système IA propriétaire</p>

    <h2>Points de terminaison disponibles</h2>

    <div class="endpoint">
        <h3><span class="tag">POST</span> /txt2img</h3>
        <p><strong>Génère une image à partir d'un texte (prompt)</strong></p>
        <p>Corps de la requête :</p>
        <pre><code>{
  "prompt": "un beau paysage, montagnes, coucher de soleil, 4k",
  "negative_prompt": "flou, basse qualité",
  "width": 512,
  "height": 512,
  "steps": 25,
  "lang": "fr"
}</code></pre>
        <p>Réponse :</p>
        <pre><code>{
  "images": ["image_png_encodée_base64"],
  "info": {
    "prompt": "un beau paysage...",
    "width": 512,
    "height": 512,
    "time_taken": 0.123
  }
}</code></pre>
    </div>

    <div class="endpoint">
        <h3><span class="tag">GET</span> /health</h3>
        <p><strong>Vérifie l'état du serveur</strong></p>
        <p>Réponse :</p>
        <pre><code>{
  "status": "healthy",
  "version": "1.0.0",
  "model": "Avila Diffusion v1.0"
}</code></pre>
    </div>

    <h2>Caractéristiques</h2>
    <ul>
        <li>✅ Système 100% propriétaire</li>
        <li>✅ Génération rapide d'images</li>
        <li>✅ API REST simple</li>
        <li>✅ Format PNG avec base64</li>
        <li>✅ Zéro dépendance IA externe</li>
        <li>✅ Support multilingue (PT/FR/DE)</li>
    </ul>

    <h2>Exemple d'utilisation (curl)</h2>
    <pre><code>curl -X POST http://localhost:8080/txt2img \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "un magnifique coucher de soleil sur les montagnes",
    "width": 512,
    "height": 512,
    "lang": "fr"
  }'</code></pre>

    <footer style="margin-top: 50px; padding-top: 20px; border-top: 1px solid #ddd; color: #7f8c8d; text-align: center;">
        <p><strong>Avila Diffusion v1.0</strong> - Souveraineté Technologique 🇫🇷</p>
        <p style="font-size: 12px;">
            <a href="?lang=pt">🇧🇷 Português</a> |
            <a href="?lang=fr">🇫🇷 Français</a> |
            <a href="?lang=de">🇩🇪 Deutsch</a>
        </p>
    </footer>
</body>
</html>"#;

const INDEX_HTML_DE: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>Avila Diffusion API</title>
    <style>
        body { font-family: Arial; max-width: 900px; margin: 50px auto; padding: 20px; background: #f5f5f5; }
        h1 { color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }
        .endpoint { background: white; padding: 20px; margin: 20px 0; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        code { background: #34495e; color: #ecf0f1; padding: 2px 8px; border-radius: 4px; font-family: 'Courier New', monospace; }
        pre { background: #2c3e50; color: #ecf0f1; padding: 15px; border-radius: 8px; overflow-x: auto; }
        .tag { background: #3498db; color: white; padding: 4px 12px; border-radius: 12px; font-size: 12px; font-weight: bold; }
        ul { line-height: 1.8; }
    </style>
</head>
<body>
    <h1>🎨 Avila Diffusion API</h1>
    <p><strong>100% proprietärer KI-Bildgenerator</strong></p>
    <p>Vollständige technologische Souveränität - Eigenes KI-System</p>

    <h2>Verfügbare Endpunkte</h2>

    <div class="endpoint">
        <h3><span class="tag">POST</span> /txt2img</h3>
        <p><strong>Generiert ein Bild aus Text (Prompt)</strong></p>
        <p>Anfrage-Body:</p>
        <pre><code>{
  "prompt": "eine schöne Landschaft, Berge, Sonnenuntergang, 4k",
  "negative_prompt": "verschwommen, niedrige Qualität",
  "width": 512,
  "height": 512,
  "steps": 25,
  "lang": "de"
}</code></pre>
        <p>Antwort:</p>
        <pre><code>{
  "images": ["base64_kodiertes_png_bild"],
  "info": {
    "prompt": "eine schöne Landschaft...",
    "width": 512,
    "height": 512,
    "time_taken": 0.123
  }
}</code></pre>
    </div>

    <div class="endpoint">
        <h3><span class="tag">GET</span> /health</h3>
        <p><strong>Überprüft den Serverstatus</strong></p>
        <p>Antwort:</p>
        <pre><code>{
  "status": "healthy",
  "version": "1.0.0",
  "model": "Avila Diffusion v1.0"
}</code></pre>
    </div>

    <h2>Eigenschaften</h2>
    <ul>
        <li>✅ 100% proprietäres System</li>
        <li>✅ Schnelle Bildgenerierung</li>
        <li>✅ Einfache REST-API</li>
        <li>✅ PNG-Format mit base64</li>
        <li>✅ Keine externen KI-Abhängigkeiten</li>
        <li>✅ Mehrsprachige Unterstützung (PT/FR/DE)</li>
    </ul>

    <h2>Verwendungsbeispiel (curl)</h2>
    <pre><code>curl -X POST http://localhost:8080/txt2img \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "ein wunderschöner Sonnenuntergang über den Bergen",
    "width": 512,
    "height": 512,
    "lang": "de"
  }'</code></pre>

    <footer style="margin-top: 50px; padding-top: 20px; border-top: 1px solid #ddd; color: #7f8c8d; text-align: center;">
        <p><strong>Avila Diffusion v1.0</strong> - Technologische Souveränität 🇩🇪</p>
        <p style="font-size: 12px;">
            <a href="?lang=pt">🇧🇷 Português</a> |
            <a href="?lang=fr">🇫🇷 Français</a> |
            <a href="?lang=de">🇩🇪 Deutsch</a>
        </p>
    </footer>
</body>
</html>"#;
