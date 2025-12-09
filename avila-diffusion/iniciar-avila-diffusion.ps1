# Iniciar Avila Diffusion Server

Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         🎨 AVILA DIFFUSION - Gerador de Imagens IA       ║" -ForegroundColor Cyan
Write-Host "║                  100% Soberania Tecnológica              ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$exePath = Join-Path $PSScriptRoot "target\release\avila-diffusion-server.exe"

if (!(Test-Path $exePath)) {
    Write-Host "❌ Binário não encontrado!" -ForegroundColor Red
    Write-Host "   Compile primeiro com: .\build.ps1" -ForegroundColor Yellow
    Write-Host ""
    exit 1
}

# Configurar GPU (auto-detect)
$env:AVX_GPU_DEVICE = "auto"

# Porta padrão
$env:PORT = "7860"

Write-Host "🚀 Iniciando servidor..." -ForegroundColor Green
Write-Host "   GPU: $env:AVX_GPU_DEVICE" -ForegroundColor Gray
Write-Host "   Porta: $env:PORT" -ForegroundColor Gray
Write-Host ""
Write-Host "Pressione Ctrl+C para parar" -ForegroundColor Yellow
Write-Host ""

& $exePath
