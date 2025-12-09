# 🎨 Avila Diffusion - Interface Moderna

## Frontend 100% Proprietário

Interface web moderna e responsiva para o **Avila Diffusion**, sistema de geração de imagens IA com soberania tecnológica total.

---

## 🚀 Acesso Rápido

```bash
# Iniciar servidor com frontend
.\start-app.ps1

# Acessar interface
http://localhost:8080/app
```

---

## ✨ Características

### Design Moderno
- 🎨 **Gradient Background**: Fundo gradiente roxo/azul profissional
- 📱 **Responsivo**: Layout adaptativo para desktop, tablet e mobile
- 🌙 **UI Limpa**: Interface minimalista e focada
- ⚡ **Animações Suaves**: Transições e hover effects elegantes

### Funcionalidades
- ✍️ **Editor de Prompt**: Textarea expandível para descrições detalhadas
- ➖ **Prompt Negativo**: Controle avançado do que evitar na geração
- 📐 **Seletor de Tamanho**: 3 tamanhos pré-configurados (256, 512, 768)
- 🌍 **Multilíngue**: Português 🇧🇷, Français 🇫🇷, Deutsch 🇩🇪
- 📊 **Estatísticas**: Tempo de geração e dimensões da imagem
- ⬇️ **Download Direto**: Botão para salvar imagem gerada

### Experiência do Usuário
- 🔄 **Loading Spinner**: Indicador visual durante geração
- ✅ **Alertas de Status**: Feedback de sucesso/erro
- 🖼️ **Preview em Tempo Real**: Visualização imediata da imagem
- ⌨️ **Atalho de Teclado**: Ctrl+Enter para gerar
- 📦 **Placeholder Elegante**: Ícone SVG antes da primeira geração

---

## 🏗️ Arquitetura

### Stack Tecnológico
- **Backend**: Rust + Warp (HTTP server)
- **Frontend**: HTML5 + CSS3 + JavaScript Vanilla
- **Comunicação**: REST API + JSON
- **Encoding**: Base64 para transferência de imagens

### Estrutura de Arquivos
```
frontend/
└── index.html          # Interface completa (HTML + CSS + JS inline)
    ├── Estilos CSS     # Design system com variáveis CSS
    ├── JavaScript      # Lógica de interação e API
    └── Traduções       # Suporte a 3 idiomas
```

---

## 🎯 Como Usar

### 1. Iniciar Servidor
```powershell
cd d:\stable-diffusion-webui\avila-diffusion
.\start-app.ps1
```

### 2. Acessar Interface
Abra o navegador em: **http://localhost:8080/app**

### 3. Gerar Imagem
1. **Digite a descrição** no campo "Descrição da Imagem"
   - Ex: "uma bela paisagem com montanhas ao pôr do sol"

2. **(Opcional) Prompt Negativo**
   - Ex: "borrado, baixa qualidade"

3. **Selecione o tamanho**
   - 256x256 (rápido)
   - 512x512 (balanceado) ⭐
   - 768x768 (alta qualidade)

4. **Clique em "Gerar Imagem"** ou pressione **Ctrl+Enter**

5. **Aguarde** (5-15 segundos)

6. **Visualize e Baixe** a imagem gerada

---

## 🌍 Suporte Multilíngue

### Português 🇧🇷
- Interface completa em português brasileiro
- Mensagens de erro localizadas
- Documentação em PT-BR

### Français 🇫🇷
- Interface complète en français
- Messages d'erreur localisés
- Documentation en français

### Deutsch 🇩🇪
- Vollständige Benutzeroberfläche auf Deutsch
- Lokalisierte Fehlermeldungen
- Dokumentation auf Deutsch

**Trocar idioma**: Clique nos botões de bandeira no topo da página.

---

## 🔌 API Integration

### Endpoint: POST /txt2img

**Request:**
```json
{
  "prompt": "a beautiful landscape, mountains, sunset, 4k",
  "negative_prompt": "blurry, low quality",
  "width": 512,
  "height": 512,
  "lang": "pt"
}
```

**Response:**
```json
{
  "images": ["iVBORw0KGgoAAAANSUhEUgAA...base64..."],
  "info": {
    "prompt": "a beautiful landscape...",
    "width": 512,
    "height": 512,
    "time_taken": 0.123
  }
}
```

### JavaScript Example
```javascript
const response = await fetch('http://localhost:8080/txt2img', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
        prompt: "seu prompt aqui",
        width: 512,
        height: 512,
        lang: "pt"
    })
});

const data = await response.json();
const imageBase64 = data.images[0];
```

---

## 🎨 Design System

### Cores
```css
--primary: #2563eb        /* Azul principal */
--primary-dark: #1e40af   /* Azul escuro */
--success: #10b981        /* Verde sucesso */
--danger: #ef4444         /* Vermelho erro */
--dark: #1f2937           /* Texto escuro */
--gray: #6b7280           /* Cinza neutro */
--light: #f3f4f6          /* Fundo claro */
```

### Tipografia
- **Font**: System fonts (-apple-system, Segoe UI, Roboto)
- **Títulos**: Bold, 1.5-2.5rem
- **Corpo**: Regular, 1rem
- **Botões**: Bold, 1.1rem

### Espaçamento
- **Padding**: 20-30px
- **Gaps**: 10-30px
- **Border Radius**: 10-25px

---

## 📊 Performance

### Métricas
- ⚡ **Carregamento**: <100ms (HTML inline)
- 🖼️ **Geração de Imagem**: 5-15s (depende do tamanho)
- 📦 **Tamanho do HTML**: ~25KB (não comprimido)
- 🚀 **Primeiro Byte**: <50ms (servidor local)

### Otimizações
- ✅ HTML inline (zero requisições externas)
- ✅ CSS minificado internamente
- ✅ JavaScript vanilla (sem dependências)
- ✅ Base64 inline para imagens geradas

---

## 🛠️ Customização

### Modificar Cores
Edite as variáveis CSS em `index.html`:
```css
:root {
    --primary: #2563eb;  /* Sua cor primária */
    --success: #10b981;  /* Sua cor de sucesso */
}
```

### Adicionar Tamanhos
Adicione novos botões de tamanho:
```html
<button class="size-btn" onclick="setSize(1024, 1024)">1024x1024</button>
```

### Novo Idioma
Adicione tradução ao objeto `translations`:
```javascript
translations.es = {
    subtitle: 'Generador de Imágenes con IA',
    // ... mais traduções
};
```

---

## 🐛 Troubleshooting

### Frontend não carrega
```powershell
# Verificar se arquivo existe
Test-Path "d:\stable-diffusion-webui\avila-diffusion\frontend\index.html"

# Recompilar servidor
cd d:\stable-diffusion-webui\avila-diffusion
cargo build --release
```

### Erro de CORS
- ✅ Não aplicável: Frontend servido pelo mesmo servidor backend
- ✅ Warp automaticamente lida com CORS para localhost

### Imagem não gera
1. Verifique se servidor está rodando: `http://localhost:8080/health`
2. Abra DevTools (F12) e veja erros no Console
3. Teste API diretamente:
```powershell
curl -X POST http://localhost:8080/txt2img `
  -H "Content-Type: application/json" `
  -d '{"prompt":"test","width":256,"height":256}'
```

---

## 📝 Roadmap

### Próximas Funcionalidades
- [ ] Upload de imagem para img2img
- [ ] Histórico de gerações
- [ ] Galeria com thumbnails
- [ ] Seed control para reprodutibilidade
- [ ] Batch generation (múltiplas imagens)
- [ ] Presets de prompts
- [ ] Exportar configurações
- [ ] Dark/Light mode toggle
- [ ] PWA (Progressive Web App)
- [ ] WebSocket para status em tempo real

### Melhorias de UX
- [ ] Drag & drop de imagens
- [ ] Zoom/Pan na imagem gerada
- [ ] Comparação lado a lado
- [ ] Copiar prompt de imagens anteriores
- [ ] Atalhos de teclado avançados
- [ ] Tour guiado para novos usuários

---

## 🏆 Diferenciais

### vs Stable Diffusion WebUI
- ✅ **Mais Leve**: Interface 10x menor
- ✅ **Mais Rápida**: Carregamento instantâneo
- ✅ **Mais Limpa**: UI focada e sem distrações
- ✅ **100% Proprietário**: Zero dependências externas

### vs Midjourney/DALL-E
- ✅ **Local**: Roda 100% na sua máquina
- ✅ **Privado**: Sem envio de dados para cloud
- ✅ **Ilimitado**: Sem limites de gerações
- ✅ **Customizável**: Código aberto (proprietário)

---

## 📄 Licença

© 2025 Avila Inc. - Todos os direitos reservados.

Sistema proprietário com soberania tecnológica total.

---

## 🤝 Suporte

**Documentação**: http://localhost:8080/
**Health Check**: http://localhost:8080/health
**Interface**: http://localhost:8080/app

---

**Desenvolvido com ❤️ usando Rust + HTML5**
