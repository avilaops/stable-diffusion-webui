# 🎨 Avila Diffusion - Iniciar com Frontend Moderno
Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         🎨 AVILA DIFFUSION - Interface Moderna           ║" -ForegroundColor Cyan
Write-Host "║                  100% Soberania Tecnológica              ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$exePath = ".\target\release\avila-diffusion-server.exe"

if (-not (Test-Path $exePath)) {
    Write-Host "❌ Binário não encontrado. Compilando..." -ForegroundColor Red
    cargo build --release
}

Write-Host "🚀 Iniciando servidor..." -ForegroundColor Green
Write-Host ""
Write-Host "📱 Interface Moderna: " -NoNewline
Write-Host "http://localhost:8080/app" -ForegroundColor Yellow
Write-Host "📖 Documentação API:  " -NoNewline
Write-Host "http://localhost:8080/" -ForegroundColor Yellow
Write-Host "❤️  Health Check:     " -NoNewline
Write-Host "http://localhost:8080/health" -ForegroundColor Yellow
Write-Host ""
Write-Host "Pressione Ctrl+C para parar" -ForegroundColor Gray
Write-Host ""

& $exePath
