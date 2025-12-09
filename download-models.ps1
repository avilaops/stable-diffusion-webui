# Script para baixar modelos do Stable Diffusion
# Modelos sem censura e de alta qualidade

$modelsDir = "D:\stable-diffusion-webui\models\Stable-diffusion"

Write-Host "Baixando modelos para Stable Diffusion..." -ForegroundColor Green

# Realistic Vision V6.0 - Fotorrealismo sem censura
Write-Host "`n[1/3] Baixando Realistic Vision V6.0 (4.3 GB)..." -ForegroundColor Cyan
$url1 = "https://civitai.com/api/download/models/245598"
$output1 = "$modelsDir\realisticVisionV60B1_v51VAE.safetensors"
Invoke-WebRequest -Uri $url1 -OutFile $output1 -UseBasicParsing

# DreamShaper 8 - Versatilidade sem filtros
Write-Host "`n[2/3] Baixando DreamShaper 8 (2.1 GB)..." -ForegroundColor Cyan
$url2 = "https://civitai.com/api/download/models/128713"
$output2 = "$modelsDir\dreamshaper_8.safetensors"
Invoke-WebRequest -Uri $url2 -OutFile $output2 -UseBasicParsing

# Anything V5 - Estilo anime sem restrições
Write-Host "`n[3/3] Baixando Anything V5 (2.3 GB)..." -ForegroundColor Cyan
$url3 = "https://civitai.com/api/download/models/114367"
$output3 = "$modelsDir\anythingV5_PrtRE.safetensors"
Invoke-WebRequest -Uri $url3 -OutFile $output3 -UseBasicParsing

Write-Host "`n✅ Todos os modelos baixados com sucesso!" -ForegroundColor Green
Write-Host "Os modelos estão em: $modelsDir" -ForegroundColor Yellow
Write-Host "`nAgora você pode:" -ForegroundColor White
Write-Host "1. Iniciar o WebUI executando: .\webui-user.bat" -ForegroundColor White
Write-Host "2. Acessar http://127.0.0.1:7860 no navegador" -ForegroundColor White
Write-Host "3. Selecionar um modelo no dropdown superior" -ForegroundColor White
Write-Host "4. Gerar imagens sem restrições!" -ForegroundColor White
