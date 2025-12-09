# Build script para Avila Diffusion
# Compila o projeto com otimizações máximas

Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║     🎨 Compilando Avila Diffusion - Gerador de IA        ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$ErrorActionPreference = "Stop"

# Verificar Rust instalado
Write-Host "✓ Verificando Rust..." -ForegroundColor Yellow
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ ERRO: Rust não instalado!" -ForegroundColor Red
    Write-Host "Instale em: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

$rustVersion = cargo --version
Write-Host "  └─ $rustVersion" -ForegroundColor Green

# Verificar paths das dependências
Write-Host ""
Write-Host "✓ Verificando dependências Avila/Arxis..." -ForegroundColor Yellow

$deps = @(
    "..\..\arxis\avx-gpu\avx-gpu-core",
    "..\..\arxis\avx-gpu\avx-gpu-std",
    "..\..\arxis\avila-image",
    "..\..\arxis\avila-error"
)

$allExist = $true
foreach ($dep in $deps) {
    $fullPath = Join-Path $PSScriptRoot $dep
    if (Test-Path $fullPath) {
        Write-Host "  ✓ $dep" -ForegroundColor Green
    } else {
        Write-Host "  ❌ $dep (NÃO ENCONTRADO)" -ForegroundColor Red
        $allExist = $false
    }
}

if (!$allExist) {
    Write-Host ""
    Write-Host "⚠️  AVISO: Algumas dependências não foram encontradas!" -ForegroundColor Yellow
    Write-Host "   O build pode falhar. Certifique-se que os projetos Arxis existem." -ForegroundColor Yellow
    Write-Host ""
    $continue = Read-Host "Continuar mesmo assim? (s/N)"
    if ($continue -ne "s" -and $continue -ne "S") {
        exit 1
    }
}

# Limpar build anterior
Write-Host ""
Write-Host "✓ Limpando builds anteriores..." -ForegroundColor Yellow
if (Test-Path "target") {
    Remove-Item -Recurse -Force "target" -ErrorAction SilentlyContinue
    Write-Host "  └─ target/ removido" -ForegroundColor Green
}

# Compilar
Write-Host ""
Write-Host "🔨 Compilando (modo release)..." -ForegroundColor Cyan
Write-Host "   Isso pode levar alguns minutos..." -ForegroundColor Gray
Write-Host ""

Push-Location $PSScriptRoot
try {
    cargo build --release

    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Green
        Write-Host "✅ COMPILAÇÃO CONCLUÍDA COM SUCESSO!" -ForegroundColor Green
        Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Green
        Write-Host ""
        Write-Host "📦 Binário: target\release\avila-diffusion-server.exe" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "Para executar:" -ForegroundColor Yellow
        Write-Host "  .\target\release\avila-diffusion-server.exe" -ForegroundColor White
        Write-Host ""
        Write-Host "Ou usar o atalho:" -ForegroundColor Yellow
        Write-Host "  .\iniciar-avila-diffusion.ps1" -ForegroundColor White
        Write-Host ""
    } else {
        Write-Host ""
        Write-Host "❌ ERRO NA COMPILAÇÃO!" -ForegroundColor Red
        Write-Host ""
        Write-Host "Próximos passos:" -ForegroundColor Yellow
        Write-Host "1. Verifique se todas as dependências Avila existem" -ForegroundColor White
        Write-Host "2. Verifique se os paths em Cargo.toml estão corretos" -ForegroundColor White
        Write-Host "3. Execute: cargo build --release --verbose" -ForegroundColor White
        Write-Host ""
        exit 1
    }
} finally {
    Pop-Location
}
