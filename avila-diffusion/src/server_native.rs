//! Servidor HTTP ultra-leve - 100% std::net

use crate::history::{HistoryEntry, HistoryStore};
use crate::AvilaDiffusion;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
// Removed unused thread import

pub fn run_server(port: u16) -> Result<(), String> {
    let mut diffusion = AvilaDiffusion::new()?;

    let history_path = std::env::var("AVILA_HISTORY_PATH")
        .unwrap_or_else(|_| "data/history/history.json".to_string());
    let history_capacity = std::env::var("AVILA_HISTORY_CAPACITY")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(12);
    let mut history = HistoryStore::load(&history_path, history_capacity);

    println!(
        "🗂️  Histórico persistente: {} itens carregados (máx {})",
        history.entries().len(),
        history.capacity()
    );
    println!("   Arquivo: {}", history_path);

    let addr = format!("127.0.0.1:{}", port);
    let listener =
        TcpListener::bind(&addr).map_err(|e| format!("Erro ao abrir porta {}: {}", port, e))?;

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║         🎨 AVILA DIFFUSION - 100% Nativo                 ║");
    println!("║              ZERO Dependências Externas                   ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("🌐 Servidor rodando em http://127.0.0.1:{}", port);
    println!("🎨 Interface Principal: http://localhost:{}/app", port);
    println!("📖 Documentação API:    http://localhost:{}/", port);
    println!(
        "✨ Endpoint txt2img:    POST http://localhost:{}/txt2img",
        port
    );
    println!();
    println!("Pressione Ctrl+C para parar");
    println!();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream, &mut diffusion, &mut history);
            }
            Err(e) => eprintln!("❌ Erro na conexão: {}", e),
        }
    }

    Ok(())
}

fn handle_connection(
    mut stream: TcpStream,
    diffusion: &mut AvilaDiffusion,
    history: &mut HistoryStore,
) {
    let mut buffer = vec![0; 16384]; // 16KB buffer

    let n = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("❌ Erro ao ler stream: {}", e);
            return;
        }
    };

    if n == 0 {
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..n]);
    let lines: Vec<&str> = request.lines().collect();

    if lines.is_empty() {
        return;
    }

    let parts: Vec<&str> = lines[0].split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }

    let method = parts[0];
    let path = parts[1];

    println!("📥 {} {}", method, path);

    let response = match (method, path) {
        ("GET", "/") => serve_api_docs(),
        ("GET", "/app") => serve_frontend(),
        ("GET", "/favicon.ico") => serve_favicon(),
        ("GET", "/health") => serve_health(),
        ("POST", "/txt2img") => {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                handle_txt2img(&request, diffusion, history)
            })) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("❌ PANIC em handle_txt2img: {:?}", e);
                    error_response("Server panic during image generation")
                }
            }
        }
        ("GET", "/history") => serve_history(history),
        ("DELETE", "/history") => clear_history(history),
        _ => serve_404(),
    };

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

fn serve_api_docs() -> String {
    let html = r#"
<!DOCTYPE html>
<html><head><meta charset="UTF-8"><title>Avila Diffusion API</title>
<style>body{font-family:sans-serif;max-width:800px;margin:50px auto;padding:20px}
h1{color:#0071e3}code{background:#f5f5f7;padding:2px 8px;border-radius:4px}</style>
</head><body>
<h1>🎨 Avila Diffusion API</h1>
<p><strong>100% Nativo - ZERO Dependências</strong></p>
<h2>Endpoints:</h2>
<ul>
<li><code>GET /app</code> - Interface web</li>
<li><code>POST /txt2img</code> - Gerar imagem</li>
<li><code>GET /health</code> - Status</li>
</ul>
<h3>POST /txt2img</h3>
<p>Body JSON: <code>{"prompt": "sua descrição", "width": 512, "height": 512}</code></p>
<p>Retorna: <code>{"image": "base64..."}</code></p>
</body></html>
"#;

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    )
}

fn serve_frontend() -> String {
    let html = include_str!("../frontend/index.html");
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    )
}

fn serve_favicon() -> String {
    let icon = include_bytes!("../frontend/favicon.ico");
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: image/x-icon\r\nContent-Length: {}\r\n\r\n",
        icon.len()
    );
    let mut response = Vec::new();
    response.extend_from_slice(header.as_bytes());
    response.extend_from_slice(icon);
    String::from_utf8_lossy(&response).to_string()
}

fn serve_health() -> String {
    let json = r#"{"status":"healthy","version":"2.0.0","native":true}"#;
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        json.len(),
        json
    )
}

fn serve_404() -> String {
    let html = "<h1>404 Not Found</h1>";
    format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    )
}

#[derive(Deserialize)]
struct Txt2ImgPayload {
    prompt: Option<String>,
    negative_prompt: Option<String>,
    width: Option<usize>,
    height: Option<usize>,
    steps: Option<usize>,
    guidance: Option<f32>,
    #[serde(alias = "cfg_scale")]
    cfg_scale: Option<f32>,
    seed: Option<i64>,
    lang: Option<String>,
}

#[derive(Serialize)]
struct HistoryResponse<'a> {
    items: &'a [HistoryEntry],
    count: usize,
    capacity: usize,
}

fn handle_txt2img(
    request: &str,
    diffusion: &mut AvilaDiffusion,
    history: &mut HistoryStore,
) -> String {
    use std::time::Instant;

    // Extrai JSON do body
    let body_start = request.find("\r\n\r\n");
    if body_start.is_none() {
        return error_response("Invalid request");
    }

    let body = &request[body_start.unwrap() + 4..];

    let payload: Txt2ImgPayload = match serde_json::from_str(body) {
        Ok(p) => p,
        Err(e) => return error_response(&format!("Invalid JSON: {}", e)),
    };

    let prompt = payload.prompt.unwrap_or_else(|| "abstract art".to_string());
    let negative_prompt = payload.negative_prompt.and_then(|s| {
        let trimmed = s.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    });
    let lang = payload
        .lang
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let width = payload.width.unwrap_or(512);
    let height = payload.height.unwrap_or(512);
    let steps = payload.steps.unwrap_or(20);
    let guidance = payload.guidance.or(payload.cfg_scale).unwrap_or(7.5);
    let seed_opt = payload
        .seed
        .filter(|value| *value >= 0)
        .map(|value| value as u64);

    println!(
        "🎨 Gerando: {} ({}x{}, {} steps, guidance {:.1})",
        prompt, width, height, steps, guidance
    );
    if let Some(ref neg) = negative_prompt {
        println!("   ▸ Negative prompt: {}", neg);
    }
    if let Some(seed) = seed_opt {
        println!("   ▸ Seed fixo: {}", seed);
    } else {
        println!("   ▸ Seed: aleatório");
    }
    if let Some(ref lang_code) = lang {
        println!("   ▸ Idioma: {}", lang_code);
    } else {
        println!("   ▸ Idioma: padrão (pt)");
    }

    // Gera imagem
    let start = Instant::now();
    match diffusion.text_to_image(
        &prompt,
        negative_prompt.as_deref(),
        width,
        height,
        steps,
        guidance,
        seed_opt,
    ) {
        Ok(img) => {
            println!("✅ Imagem gerada, convertendo para SVG...");
            let elapsed = start.elapsed().as_secs_f32();
            let svg_data = img.to_data_url_svg();
            println!("✅ SVG gerado: {} chars", svg_data.len());

            let timestamp_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                .as_millis() as u64;

            let mode = diffusion.mode_name().to_string();
            let history_entry = HistoryEntry {
                id: timestamp_ms,
                prompt: prompt.clone(),
                negative_prompt: negative_prompt.clone(),
                width,
                height,
                steps,
                guidance,
                seed: seed_opt,
                time_taken: elapsed,
                lang: lang.clone(),
                timestamp_ms,
                image_data: svg_data.clone(),
                mode: mode.clone(),
            };

            let trimmed = history.add_entry(history_entry);
            if trimmed {
                println!(
                    "🧹 Histórico: limite de {} atingido, removendo itens antigos",
                    history.capacity()
                );
            }

            let lang_for_info = lang.clone().unwrap_or_else(|| "pt".to_string());
            let info_payload = json!({
                "prompt": prompt,
                "negative_prompt": negative_prompt,
                "width": width,
                "height": height,
                "steps": steps,
                "guidance": guidance,
                "seed": seed_opt,
                "time_taken": elapsed,
                "mode": mode,
                "lang": lang_for_info,
            });

            let response_payload = json!({
                "images": [svg_data],
                "info": info_payload,
                "history": {
                    "id": timestamp_ms,
                    "stored": true,
                    "trimmed": trimmed,
                }
            });

            let json_body =
                serde_json::to_string(&response_payload).unwrap_or_else(|_| "{}".to_string());

            println!(
                "✅ Gerado em {:.2}s ({} KB)",
                elapsed,
                json_body.len() / 1024
            );

            // Escrever resposta
            let header = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n", json_body.len());
            format!("{}{}", header, json_body)
        }
        Err(e) => {
            eprintln!("❌ Erro na geração: {}", e);
            error_response(&e)
        }
    }
}

fn serve_history(history: &HistoryStore) -> String {
    let payload = HistoryResponse {
        items: history.entries(),
        count: history.entries().len(),
        capacity: history.capacity(),
    };

    let json_body = serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string());

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        json_body.len(),
        json_body
    )
}

fn clear_history(history: &mut HistoryStore) -> String {
    let cleared = history.clear();
    let payload = json!({
        "status": "ok",
        "cleared": cleared,
    });

    let json_body = serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string());

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        json_body.len(),
        json_body
    )
}

fn error_response(msg: &str) -> String {
    let json = format!(r#"{{"error":"{}"}}"#, msg);
    format!("HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", json.len(), json)
}
