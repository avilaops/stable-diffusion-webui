# Script de teste da API Avila Diffusion

Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║           🧪 Testando Avila Diffusion API                ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Verificar se servidor está rodando
Write-Host "1️⃣  Verificando servidor..." -ForegroundColor Yellow
try {
    $health = Invoke-RestMethod -Uri "http://localhost:7860/health" -Method GET
    Write-Host "   ✅ Servidor online!" -ForegroundColor Green
    Write-Host "   Status: $($health.status)" -ForegroundColor Gray
    Write-Host "   Versão: $($health.version)" -ForegroundColor Gray
    Write-Host "   Modelo: $($health.model)" -ForegroundColor Gray
} catch {
    Write-Host "   ❌ Servidor offline ou inacessível!" -ForegroundColor Red
    Write-Host "   Execute: .\iniciar-avila-diffusion.ps1" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "2️⃣  Testando geração de imagem..." -ForegroundColor Yellow

$request = @{
    prompt = "beautiful sunset over mountains"
    width = 256
    height = 256
    steps = 25
} | ConvertTo-Json

try {
    $response = Invoke-RestMethod -Uri "http://localhost:7860/txt2img" -Method POST -Body $request -ContentType "application/json"

    Write-Host "   ✅ Imagem gerada com sucesso!" -ForegroundColor Green
    Write-Host "   Prompt: $($response.info.prompt)" -ForegroundColor Gray
    Write-Host "   Tamanho: $($response.info.width)x$($response.info.height)" -ForegroundColor Gray
    Write-Host "   Tempo: $($response.info.time_taken)s" -ForegroundColor Gray
    Write-Host "   Imagens: $($response.images.Count)" -ForegroundColor Gray

    # Salvar imagem
    $imageData = $response.images[0]
    $bytes = [System.Convert]::FromBase64String($imageData)
    $outputPath = "d:\stable-diffusion-webui\avila-diffusion\test_output.png"
    [System.IO.File]::WriteAllBytes($outputPath, $bytes)

    Write-Host ""
    Write-Host "   💾 Imagem salva em: $outputPath" -ForegroundColor Cyan
    Write-Host ""

    # Abrir imagem
    Write-Host "   🖼️  Abrindo imagem..." -ForegroundColor Yellow
    Start-Process $outputPath

} catch {
    Write-Host "   ❌ Erro ao gerar imagem!" -ForegroundColor Red
    Write-Host "   $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host "✅ TODOS OS TESTES PASSARAM!" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Green
