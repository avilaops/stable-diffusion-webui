//! Teste completo do sistema - Carrega modelo real e gera imagem

use avila_diffusion::{AvilaDiffusion, SafeTensorsLoader};

fn main() -> Result<(), String> {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  🧪 TESTE COMPLETO - AVILA STABLE DIFFUSION                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // 1. Listar modelos disponíveis
    println!("📋 Modelos disponíveis:");
    let models = SafeTensorsLoader::list_available_models(
        "d:\\stable-diffusion-webui\\models\\Stable-diffusion",
    );

    for (i, model) in models.iter().enumerate() {
        println!("   {}: {}", i + 1, model);
    }

    if models.is_empty() {
        return Err("Nenhum modelo encontrado!".to_string());
    }

    // 2. Escolher modelo (usar dreamshaper_8.safetensors)
    let model_name = models
        .iter()
        .find(|m| m.contains("dreamshaper"))
        .or(models.first())
        .ok_or("Nenhum modelo válido")?;

    let model_path = format!(
        "d:\\stable-diffusion-webui\\models\\Stable-diffusion\\{}",
        model_name
    );

    println!("\n✅ Modelo selecionado: {}", model_name);
    println!("📂 Path: {}\n", model_path);

    // 3. Tentar carregar modelo COMPLETO
    println!("⏳ Carregando modelo completo (isso pode demorar 30-60 segundos)...\n");

    let result = AvilaDiffusion::with_model(&model_path, 20);

    match result {
        Ok(mut diffusion) => {
            println!("\n✅ Modelo carregado com sucesso!");

            // Gerar imagem
            let prompt = "a beautiful sunset over mountains, photorealistic, 8k, detailed";
            let img = diffusion.text_to_image(prompt, None, 512, 512, 30, 7.5, None)?;

            // Salvar
            let output_path = "d:\\test_output_full.png";
            img.save_png(output_path);
            println!("\n💾 Salvo em: {}", output_path);
        }
        Err(e) => {
            println!("\n⚠️  Erro ao carregar modelo completo: {}", e);
            println!("\n🔄 Tentando modo LEVE (fallback)...\n");

            let mut diffusion = AvilaDiffusion::new()?;
            let prompt = "sunset over mountains";
            let img = diffusion.text_to_image(prompt, None, 512, 512, 20, 7.5, None)?;

            let output_path = "d:\\test_output_light.png";
            img.save_png(output_path);
            println!("\n💾 Salvo em: {}", output_path);
        }
    }

    println!("\n✅ TESTE CONCLUÍDO!\n");

    Ok(())
}
