# Script de teste multilíngue da API Avila Diffusion

Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║        🌍 Teste Multilíngue - Avila Diffusion           ║" -ForegroundColor Cyan
Write-Host "║            🇧🇷 Português | 🇫🇷 Français | 🇩🇪 Deutsch        ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$port = 8080

# Verificar se servidor está rodando
Write-Host "🔍 Verificando servidor..." -ForegroundColor Yellow
try {
    $null = Invoke-RestMethod -Uri "http://localhost:$port/health" -Method GET -ErrorAction Stop
    Write-Host "   ✅ Servidor online na porta $port!" -ForegroundColor Green
} catch {
    Write-Host "   ❌ Servidor offline!" -ForegroundColor Red
    Write-Host "   Execute: " -NoNewline -ForegroundColor Yellow
    Write-Host "`$env:PORT=`"$port`"; .\target\release\avila-diffusion-server.exe" -ForegroundColor White
    exit 1
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host " 🇧🇷 TESTE EM PORTUGUÊS" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan

$request_pt = @{
    prompt = "uma bela paisagem com montanhas ao pôr do sol"
    width = 256
    height = 256
    lang = "pt"
} | ConvertTo-Json

try {
    $response_pt = Invoke-RestMethod -Uri "http://localhost:$port/txt2img" -Method POST -Body $request_pt -ContentType "application/json"
    Write-Host "✅ Imagem gerada em Português!" -ForegroundColor Green
    Write-Host "   Prompt: $($response_pt.info.prompt)" -ForegroundColor Gray
    Write-Host "   Tempo: $($response_pt.info.time_taken)s" -ForegroundColor Gray

    $bytes_pt = [System.Convert]::FromBase64String($response_pt.images[0])
    $output_pt = "d:\stable-diffusion-webui\avila-diffusion\test_pt.png"
    [System.IO.File]::WriteAllBytes($output_pt, $bytes_pt)
    Write-Host "   💾 Salvo: test_pt.png" -ForegroundColor Cyan
} catch {
    Write-Host "❌ Erro: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host " 🇫🇷 TEST EN FRANÇAIS" -ForegroundColor Blue
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan

$request_fr = @{
    prompt = "un magnifique paysage avec des montagnes au coucher du soleil"
    width = 256
    height = 256
    lang = "fr"
} | ConvertTo-Json

try {
    $response_fr = Invoke-RestMethod -Uri "http://localhost:$port/txt2img" -Method POST -Body $request_fr -ContentType "application/json"
    Write-Host "✅ Image générée en Français!" -ForegroundColor Green
    Write-Host "   Prompt: $($response_fr.info.prompt)" -ForegroundColor Gray
    Write-Host "   Temps: $($response_fr.info.time_taken)s" -ForegroundColor Gray

    $bytes_fr = [System.Convert]::FromBase64String($response_fr.images[0])
    $output_fr = "d:\stable-diffusion-webui\avila-diffusion\test_fr.png"
    [System.IO.File]::WriteAllBytes($output_fr, $bytes_fr)
    Write-Host "   💾 Sauvegardé: test_fr.png" -ForegroundColor Cyan
} catch {
    Write-Host "❌ Erreur: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host " 🇩🇪 TEST AUF DEUTSCH" -ForegroundColor Yellow
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan

$request_de = @{
    prompt = "eine wunderschöne Landschaft mit Bergen bei Sonnenuntergang"
    width = 256
    height = 256
    lang = "de"
} | ConvertTo-Json

try {
    $response_de = Invoke-RestMethod -Uri "http://localhost:$port/txt2img" -Method POST -Body $request_de -ContentType "application/json"
    Write-Host "✅ Bild auf Deutsch generiert!" -ForegroundColor Green
    Write-Host "   Prompt: $($response_de.info.prompt)" -ForegroundColor Gray
    Write-Host "   Zeit: $($response_de.info.time_taken)s" -ForegroundColor Gray

    $bytes_de = [System.Convert]::FromBase64String($response_de.images[0])
    $output_de = "d:\stable-diffusion-webui\avila-diffusion\test_de.png"
    [System.IO.File]::WriteAllBytes($output_de, $bytes_de)
    Write-Host "   💾 Gespeichert: test_de.png" -ForegroundColor Cyan
} catch {
    Write-Host "❌ Fehler: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host " ✅ TESTES MULTILÍNGUES CONCLUÍDOS!" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host ""
Write-Host "📂 Imagens geradas:" -ForegroundColor Cyan
Write-Host "   🇧🇷 test_pt.png" -ForegroundColor White
Write-Host "   🇫🇷 test_fr.png" -ForegroundColor White
Write-Host "   🇩🇪 test_de.png" -ForegroundColor White
Write-Host ""
Write-Host "🌐 Acesse a documentação:" -ForegroundColor Yellow
Write-Host "   🇧🇷 http://localhost:$port/?lang=pt" -ForegroundColor White
Write-Host "   🇫🇷 http://localhost:$port/?lang=fr" -ForegroundColor White
Write-Host "   🇩🇪 http://localhost:$port/?lang=de" -ForegroundColor White
