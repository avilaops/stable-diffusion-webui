# Guia de Integração: Stable Diffusion + Cluster GPU Arxis

## 🎯 Objetivo
Conectar o Stable Diffusion WebUI local às GPUs do cluster Arxis para processamento acelerado.

## 📊 Comparação de Performance

| Método | Hardware | Tempo/Imagem | Qualidade |
|--------|----------|--------------|-----------|
| CPU Local | Intel/AMD | 5-10 minutos | Normal |
| GPU Local (DirectML) | Qualquer GPU | 30-60 segundos | Normal |
| GPU Remota (Arxis 1x) | NVIDIA/AMD Server | 10-20 segundos | Normal |
| **GPU Cluster (Arxis Multi)** | **Múltiplas GPUs** | **5-10 segundos** | **Normal** |

## 🚀 Opções de Configuração

### Opção 1: API REST (Recomendado)

**Vantagens:**
- ✅ Usa GPUs remotas automaticamente
- ✅ Balanceamento de carga entre GPUs
- ✅ Interface web continua local
- ✅ Processamento distribuído

**Como configurar:**

1. **No servidor Arxis (com GPUs):**
```bash
cd /caminho/para/stable-diffusion-webui
./webui.sh --api --listen --port 7861 --nowebui
```

2. **No PC local:**
```powershell
# Criar túnel SSH seguro
ssh -L 7861:localhost:7861 usuario@servidor-arxis

# Ou usar script Python para proxy
cd D:\stable-diffusion-webui
python arxis_gpu_client.py
```

3. **Usar o cliente Python:**
```python
from arxis_gpu_client import ArxisGPUClient

client = ArxisGPUClient(
    local_url="http://127.0.0.1:7860",      # Interface local
    remote_url="http://localhost:7861"       # GPUs remotas via SSH
)

# Gera usando GPUs remotas automaticamente
images = client.txt2img(
    prompt="beautiful landscape",
    steps=25
)
images[0].save("output.png")
```

### Opção 2: DirectML (GPU Local)

**Vantagens:**
- ✅ Funciona com qualquer GPU (NVIDIA/AMD/Intel)
- ✅ Sem configuração de rede
- ✅ Mais rápido que CPU

**Como configurar:**
```powershell
cd D:\stable-diffusion-webui
.\venv\Scripts\python.exe -m pip install torch-directml

# Editar webui-user.bat
set COMMANDLINE_ARGS=--use-directml --precision full --api --listen
```

### Opção 3: Distribuição via Arxis AVX-GPU

**Vantagens:**
- ✅ Usa framework proprietário Arxis
- ✅ Suporte multi-vendor (NVIDIA/AMD/Intel)
- ✅ Otimizações customizadas
- ✅ 100% soberania tecnológica

**Como configurar:**

1. **Compilar servidor GPU Arxis:**
```bash
cd D:\arxis\avx-gpu
cargo build --release --bin gpu-server
```

2. **Iniciar servidor:**
```bash
.\target\release\gpu-server --port 8080 --gpus all
```

3. **Conectar Stable Diffusion ao Arxis:**
```python
# Criar extensão customizada
import requests

class ArxisBackend:
    def __init__(self, arxis_url="http://localhost:8080"):
        self.url = arxis_url
    
    def process_latents(self, latents, model):
        # Envia latents para Arxis processar
        response = requests.post(
            f"{self.url}/api/v1/sd/process",
            json={"latents": latents, "model": model}
        )
        return response.json()["result"]
```

## 🔧 Arquivos Criados

1. **`configurar-gpu-arxis.ps1`** - Script interativo de configuração
2. **`arxis_gpu_client.py`** - Cliente Python para API remota
3. **`webui-user.bat`** - Configurado com `--api --listen`

## 📝 Como Usar Agora

### Para testar com API remota:

1. **Execute o configurador:**
```powershell
cd D:\stable-diffusion-webui
.\configurar-gpu-arxis.ps1
```

2. **Ou use DirectML imediatamente:**
```powershell
.\venv\Scripts\python.exe -m pip install torch-directml
# Reinicie o WebUI
```

3. **Teste o cliente Python:**
```powershell
python arxis_gpu_client.py
```

## 🌐 Endpoints da API

Com `--api` habilitado, você tem acesso a:

- `http://localhost:7860/docs` - Documentação Swagger
- `http://localhost:7860/sdapi/v1/txt2img` - Gerar imagem
- `http://localhost:7860/sdapi/v1/img2img` - Processar imagem
- `http://localhost:7860/sdapi/v1/progress` - Ver progresso
- `http://localhost:7860/sdapi/v1/sd-models` - Listar modelos

## 🎮 Exemplo Prático: Batch com GPUs

```python
from arxis_gpu_client import ArxisGPUClient
import concurrent.futures

client = ArxisGPUClient(remote_url="http://localhost:7861")

prompts = [
    "beautiful sunset",
    "mountain landscape", 
    "ocean waves",
    "forest path"
]

# Processar em paralelo usando múltiplas GPUs
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
    futures = [
        executor.submit(client.txt2img, prompt=p, steps=20)
        for p in prompts
    ]
    
    results = [f.result() for f in futures]
    
# Salvar todas
for i, images in enumerate(results):
    images[0].save(f"output_{i}.png")
```

## 🔐 Segurança

Para produção, adicione autenticação:
```bash
# No servidor
./webui.sh --api --api-auth "usuario:senha123"
```

```python
# No cliente
client.session.auth = ("usuario", "senha123")
```

## 📊 Monitoramento

Verifique uso das GPUs Arxis:
```bash
# No servidor
nvidia-smi -l 1  # NVIDIA
rocm-smi -l 1    # AMD

# Ou use API do Arxis
curl http://localhost:8080/api/v1/stats
```

## 🆘 Troubleshooting

**Problema:** "Connection refused"
- Verifique se o servidor está rodando
- Teste: `curl http://localhost:7861/internal/ping`

**Problema:** "Muito lento ainda"
- Verifique se está usando GPU: veja logs do servidor
- Teste DirectML: `--use-directml`

**Problema:** "Out of memory"
- Reduza batch size
- Use `--medvram` ou `--lowvram`
- Distribua entre múltiplas GPUs

## 📚 Próximos Passos

1. ✅ **Agora:** Use CPU local ou teste DirectML
2. 🚀 **Curto prazo:** Configure servidor Arxis com GPUs
3. 🎯 **Médio prazo:** Implemente balanceamento multi-GPU
4. 🌟 **Longo prazo:** Integre AVX-GPU framework nativo
