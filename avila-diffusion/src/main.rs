//! Avila Diffusion Server v3.0 - 100% Nativo

fn main() -> Result<(), String> {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("\n🚀 Iniciando Avila Diffusion Server na porta {}...", port);

    avila_diffusion::run_server(port)
}
