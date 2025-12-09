# Download rápido usando aria2c ou wget
$modelsDir = "D:\stable-diffusion-webui\models\Stable-diffusion"

Write-Host "Verificando ferramenta de download..." -ForegroundColor Green

# Tentar instalar aria2c (muito mais rápido)
if (!(Get-Command aria2c -ErrorAction SilentlyContinue)) {
    Write-Host "Instalando aria2c para downloads rápidos..." -ForegroundColor Cyan
    choco install aria2 -y
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
}

Write-Host "`nBaixando modelos com aria2c (16 conexões paralelas)..." -ForegroundColor Green

# Realistic Vision V6.0 - Fotorrealismo (4.3 GB)
Write-Host "`n[1/3] Baixando Realistic Vision V6.0..." -ForegroundColor Cyan
aria2c -x 16 -s 16 -k 1M `
    "https://civitai.com/api/download/models/245598" `
    -d "$modelsDir" `
    -o "realisticVisionV60B1_v51VAE.safetensors"

# DreamShaper 8 (2.1 GB)
Write-Host "`n[2/3] Baixando DreamShaper 8..." -ForegroundColor Cyan
aria2c -x 16 -s 16 -k 1M `
    "https://civitai.com/api/download/models/128713" `
    -d "$modelsDir" `
    -o "dreamshaper_8.safetensors"

# Anything V5 (2.3 GB)
Write-Host "`n[3/3] Baixando Anything V5..." -ForegroundColor Cyan
aria2c -x 16 -s 16 -k 1M `
    "https://civitai.com/api/download/models/114367" `
    -d "$modelsDir" `
    -o "anythingV5_PrtRE.safetensors"

Write-Host "`n✅ Todos os modelos baixados!" -ForegroundColor Green
Write-Host "Execute .\webui-user.bat para iniciar o Stable Diffusion" -ForegroundColor Yellow
