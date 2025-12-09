# Configuração para usar GPUs Remotas do Cluster Arxis
# Este script conecta o Stable Diffusion local às GPUs do servidor Arxis

$ErrorActionPreference = "Stop"

Write-Host "=== Configuração GPU Remota Arxis ===" -ForegroundColor Cyan
Write-Host ""

# Detectar GPUs locais
Write-Host "[1/4] Verificando GPUs locais..." -ForegroundColor Yellow
$hasNvidia = (Get-WmiObject Win32_VideoController | Where-Object { $_.Name -like "*NVIDIA*" }) -ne $null
$hasAMD = (Get-WmiObject Win32_VideoController | Where-Object { $_.Name -like "*AMD*" -or $_.Name -like "*Radeon*" }) -ne $null

if (-not $hasNvidia -and -not $hasAMD) {
    Write-Host "⚠️  Nenhuma GPU NVIDIA/AMD detectada localmente" -ForegroundColor Red
    Write-Host "✅ Configurando para usar GPUs remotas do Arxis..." -ForegroundColor Green
} else {
    Write-Host "✅ GPUs locais detectadas:" -ForegroundColor Green
    Get-WmiObject Win32_VideoController | Select-Object Name, AdapterRAM | Format-Table -AutoSize
}

Write-Host ""
Write-Host "[2/4] Verificando cluster Arxis..." -ForegroundColor Yellow

# Verificar se o cluster Arxis está acessível
$arxisPath = "D:\arxis\avx-gpu"
if (Test-Path $arxisPath) {
    Write-Host "✅ Cluster Arxis encontrado em: $arxisPath" -ForegroundColor Green

    # Verificar se há servidor GPU rodando
    $gpuServers = Get-Process | Where-Object { $_.ProcessName -like "*avx*" -or $_.ProcessName -like "*arxis*" }
    if ($gpuServers) {
        Write-Host "✅ Servidores GPU Arxis em execução:" -ForegroundColor Green
        $gpuServers | Select-Object ProcessName, Id, CPU | Format-Table -AutoSize
    } else {
        Write-Host "⚠️  Nenhum servidor GPU Arxis em execução" -ForegroundColor Yellow
        Write-Host "   Inicie o servidor GPU com: cd $arxisPath; cargo run --release" -ForegroundColor Gray
    }
} else {
    Write-Host "❌ Cluster Arxis não encontrado em $arxisPath" -ForegroundColor Red
}

Write-Host ""
Write-Host "[3/4] Opções de configuração:" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. CPU Local (Atual - Lento)" -ForegroundColor White
Write-Host "   - Usa processador Intel/AMD" -ForegroundColor Gray
Write-Host "   - Tempo: ~5-10 minutos por imagem" -ForegroundColor Gray
Write-Host ""
Write-Host "2. GPU Local via DirectML (Windows)" -ForegroundColor White
Write-Host "   - Funciona com qualquer GPU (NVIDIA/AMD/Intel)" -ForegroundColor Gray
Write-Host "   - Tempo: ~30-60 segundos por imagem" -ForegroundColor Gray
Write-Host "   - Comando: --use-directml" -ForegroundColor Gray
Write-Host ""
Write-Host "3. GPU Remota via Arxis Cluster (Recomendado)" -ForegroundColor Green
Write-Host "   - Usa múltiplas GPUs do servidor Arxis" -ForegroundColor Gray
Write-Host "   - Tempo: ~5-15 segundos por imagem" -ForegroundColor Gray
Write-Host "   - Requer: Arxis GPU Server rodando" -ForegroundColor Gray
Write-Host ""

Write-Host "[4/4] Para ativar GPU remota Arxis:" -ForegroundColor Yellow
Write-Host ""
Write-Host "OPÇÃO A - Túnel SSH (Mais seguro):" -ForegroundColor Cyan
Write-Host '  ssh -L 7861:localhost:7861 usuario@servidor-arxis' -ForegroundColor White
Write-Host '  # No servidor: cd /path/to/stable-diffusion-webui && ./webui.sh --api --port 7861' -ForegroundColor Gray
Write-Host ""

Write-Host "OPÇÃO B - API Direta (Mesma rede):" -ForegroundColor Cyan
Write-Host '  # Edite webui-user.bat e adicione:' -ForegroundColor White
Write-Host '  set COMMANDLINE_ARGS=--api --api-server-stop --listen --port 7860' -ForegroundColor Gray
Write-Host '  # Configure proxy reverso para IP_SERVIDOR_ARXIS:7860' -ForegroundColor Gray
Write-Host ""

Write-Host "OPÇÃO C - Usar DirectML local (GPU integrada):" -ForegroundColor Cyan
Write-Host '  # Instale: pip install torch-directml' -ForegroundColor White
Write-Host '  # Edite webui-user.bat:' -ForegroundColor White
Write-Host '  set COMMANDLINE_ARGS=--use-directml --precision full' -ForegroundColor Gray
Write-Host ""

Write-Host "Deseja configurar agora? (Y/N)" -ForegroundColor Yellow
$resposta = Read-Host

if ($resposta -eq "Y" -or $resposta -eq "y") {
    Write-Host ""
    Write-Host "Escolha uma opção (1-3):" -ForegroundColor Yellow
    Write-Host "1. Configurar DirectML (GPU local)" -ForegroundColor White
    Write-Host "2. Configurar API remota Arxis" -ForegroundColor White
    Write-Host "3. Manter CPU (atual)" -ForegroundColor White

    $opcao = Read-Host

    switch ($opcao) {
        "1" {
            Write-Host "Instalando DirectML..." -ForegroundColor Cyan
            Set-Location "D:\stable-diffusion-webui"
            .\venv\Scripts\python.exe -m pip install torch-directml

            # Atualizar webui-user.bat
            $batchContent = @"
@echo off

set PYTHON=python
set GIT=
set VENV_DIR=
set COMMANDLINE_ARGS=--use-directml --precision full --api --listen --port 7860

call webui.bat %*
"@
            Set-Content -Path "webui-user.bat" -Value $batchContent
            Write-Host "✅ DirectML configurado! Reinicie o WebUI." -ForegroundColor Green
        }
        "2" {
            Write-Host "Digite o IP do servidor Arxis:" -ForegroundColor Cyan
            $serverIP = Read-Host
            Write-Host "Digite a porta (padrão: 7861):" -ForegroundColor Cyan
            $serverPort = Read-Host
            if ([string]::IsNullOrEmpty($serverPort)) { $serverPort = "7861" }

            Write-Host ""
            Write-Host "✅ Configure o proxy reverso:" -ForegroundColor Green
            Write-Host "   Redirecione http://localhost:7860 -> http://$serverIP`:$serverPort" -ForegroundColor White
            Write-Host ""
            Write-Host "   Ou use SSH:" -ForegroundColor White
            Write-Host "   ssh -L 7860:localhost:$serverPort usuario@$serverIP" -ForegroundColor Gray
        }
        "3" {
            Write-Host "✅ Mantendo configuração CPU atual." -ForegroundColor Yellow
        }
    }
}

Write-Host ""
Write-Host "=== Status Atual ===" -ForegroundColor Cyan
Write-Host "WebUI: http://127.0.0.1:7860" -ForegroundColor White
Write-Host "API: http://127.0.0.1:7860/docs" -ForegroundColor White
Write-Host ""
