# 🌍 Avila Diffusion - Suporte Multilíngue

## ✅ Idiomas Suportados

- 🇧🇷 **Português** (pt)
- 🇫🇷 **Français** (fr)
- 🇩🇪 **Deutsch** (de)

---

## 🚀 Como usar

### 1. Iniciar servidor
```powershell
$env:PORT="8080"
.\target\release\avila-diffusion-server.exe
```

### 2. Acessar documentação

**Português:**
```
http://localhost:8080/?lang=pt
```

**Français:**
```
http://localhost:8080/?lang=fr
```

**Deutsch:**
```
http://localhost:8080/?lang=de
```

---

## 📡 API REST

### Gerar imagem com idioma específico

**Português:**
```json
{
  "prompt": "uma bela paisagem com montanhas",
  "width": 512,
  "height": 512,
  "lang": "pt"
}
```

**Français:**
```json
{
  "prompt": "un beau paysage avec des montagnes",
  "width": 512,
  "height": 512,
  "lang": "fr"
}
```

**Deutsch:**
```json
{
  "prompt": "eine schöne Landschaft mit Bergen",
  "width": 512,
  "height": 512,
  "lang": "de"
}
```

---

## 🧪 Testes

Execute o script de teste multilíngue:

```powershell
.\test-multilang.ps1
```

Este script irá:
- ✅ Verificar status do servidor
- ✅ Gerar 3 imagens (uma em cada idioma)
- ✅ Salvar como test_pt.png, test_fr.png, test_de.png
- ✅ Exibir links para documentação em cada idioma

---

## 📊 Exemplos de uso

### PowerShell

**Português:**
```powershell
$request = @{
    prompt = "pôr do sol nas montanhas"
    width = 512
    height = 512
    lang = "pt"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:8080/txt2img" `
    -Method POST `
    -Body $request `
    -ContentType "application/json"
```

**Français:**
```powershell
$request = @{
    prompt = "coucher de soleil sur les montagnes"
    width = 512
    height = 512
    lang = "fr"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:8080/txt2img" `
    -Method POST `
    -Body $request `
    -ContentType "application/json"
```

**Deutsch:**
```powershell
$request = @{
    prompt = "Sonnenuntergang in den Bergen"
    width = 512
    height = 512
    lang = "de"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:8080/txt2img" `
    -Method POST `
    -Body $request `
    -ContentType "application/json"
```

### cURL

**Português:**
```bash
curl -X POST http://localhost:8080/txt2img \
  -H "Content-Type: application/json" \
  -d '{"prompt":"pôr do sol nas montanhas","lang":"pt"}'
```

**Français:**
```bash
curl -X POST http://localhost:8080/txt2img \
  -H "Content-Type: application/json" \
  -d '{"prompt":"coucher de soleil sur les montagnes","lang":"fr"}'
```

**Deutsch:**
```bash
curl -X POST http://localhost:8080/txt2img \
  -H "Content-Type: application/json" \
  -d '{"prompt":"Sonnenuntergang in den Bergen","lang":"de"}'
```

---

## 🎯 Health Check com idioma

**Português:**
```
GET http://localhost:8080/health?lang=pt
```

**Français:**
```
GET http://localhost:8080/health?lang=fr
```

**Deutsch:**
```
GET http://localhost:8080/health?lang=de
```

---

## 📝 Logs do Servidor

O servidor agora exibe a bandeira do país ao receber requisições:

```
🇧🇷 Requisição em pt
🎨 Gerando imagem: 'pôr do sol nas montanhas' (512x512)
✅ Imagem gerada com sucesso!

🇫🇷 Requisição em fr
🎨 Gerando imagem: 'coucher de soleil...' (512x512)
✅ Imagem gerada com sucesso!

🇩🇪 Requisição em de
🎨 Gerando imagem: 'Sonnenuntergang...' (512x512)
✅ Imagem gerada com sucesso!
```

---

## 🌟 Recursos Adicionados

- ✅ Documentação HTML completa em 3 idiomas
- ✅ Seletor de idioma nos footers
- ✅ Logs coloridos com bandeiras
- ✅ Mensagens de erro localizadas
- ✅ Health check localizado
- ✅ Script de teste automático

---

## 💡 Expansão Futura

Para adicionar novos idiomas, edite `src/server.rs`:

1. Adicione nova constante `INDEX_HTML_XX`
2. Atualize função `get_html_for_lang()`
3. Adicione emoji de bandeira em `handle_txt2img()`
4. Recompile: `cargo build --release`

---

**Data:** 09/12/2024
**Status:** ✅ Suporte Multilíngue Completo
**Idiomas:** 🇧🇷 🇫🇷 🇩🇪
