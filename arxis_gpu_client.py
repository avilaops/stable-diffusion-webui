"""
Cliente Python para conectar Stable Diffusion local às GPUs do cluster Arxis
Usa a API REST do SD WebUI para enviar jobs para processamento remoto
"""

import requests
import json
import base64
from io import BytesIO
from PIL import Image
import time

class ArxisGPUClient:
    """Cliente para distribuir tarefas SD para cluster Arxis"""

    def __init__(self, local_url="http://127.0.0.1:7860", remote_url=None):
        self.local_url = local_url
        self.remote_url = remote_url or local_url  # Usar remoto se disponível
        self.session = requests.Session()

    def check_health(self, url):
        """Verifica se o servidor está respondendo"""
        try:
            response = self.session.get(f"{url}/internal/ping", timeout=5)
            return response.status_code == 200
        except:
            return False

    def get_best_endpoint(self):
        """Retorna o melhor endpoint disponível (prioriza remoto com GPU)"""
        if self.remote_url and self.remote_url != self.local_url:
            if self.check_health(self.remote_url):
                print(f"✅ Usando servidor remoto Arxis: {self.remote_url}")
                return self.remote_url

        if self.check_health(self.local_url):
            print(f"⚠️  Usando servidor local (CPU): {self.local_url}")
            return self.local_url

        raise ConnectionError("Nenhum servidor Stable Diffusion disponível")

    def txt2img(self, prompt, negative_prompt="", steps=20, width=512, height=512, **kwargs):
        """Gera imagem a partir de texto"""
        endpoint = self.get_best_endpoint()

        payload = {
            "prompt": prompt,
            "negative_prompt": negative_prompt,
            "steps": steps,
            "width": width,
            "height": height,
            "sampler_name": "DPM++ 2M",
            "cfg_scale": 7.0,
            **kwargs
        }

        print(f"🎨 Gerando imagem: '{prompt[:50]}...'")
        start_time = time.time()

        response = self.session.post(f"{endpoint}/sdapi/v1/txt2img", json=payload)
        response.raise_for_status()

        elapsed = time.time() - start_time
        print(f"✅ Imagem gerada em {elapsed:.2f}s")

        result = response.json()
        return self._decode_images(result['images'])

    def img2img(self, image, prompt, negative_prompt="", strength=0.75, **kwargs):
        """Transforma imagem existente"""
        endpoint = self.get_best_endpoint()

        # Converte imagem para base64
        buffered = BytesIO()
        image.save(buffered, format="PNG")
        img_base64 = base64.b64encode(buffered.getvalue()).decode()

        payload = {
            "init_images": [img_base64],
            "prompt": prompt,
            "negative_prompt": negative_prompt,
            "denoising_strength": strength,
            **kwargs
        }

        print(f"🎨 Processando img2img: '{prompt[:50]}...'")
        start_time = time.time()

        response = self.session.post(f"{endpoint}/sdapi/v1/img2img", json=payload)
        response.raise_for_status()

        elapsed = time.time() - start_time
        print(f"✅ Imagem processada em {elapsed:.2f}s")

        result = response.json()
        return self._decode_images(result['images'])

    def get_models(self):
        """Lista modelos disponíveis"""
        endpoint = self.get_best_endpoint()
        response = self.session.get(f"{endpoint}/sdapi/v1/sd-models")
        response.raise_for_status()
        return response.json()

    def set_model(self, model_name):
        """Troca o modelo ativo"""
        endpoint = self.get_best_endpoint()
        payload = {"sd_model_checkpoint": model_name}
        response = self.session.post(f"{endpoint}/sdapi/v1/options", json=payload)
        response.raise_for_status()
        print(f"✅ Modelo alterado para: {model_name}")

    def get_progress(self):
        """Obtém progresso da geração atual"""
        endpoint = self.get_best_endpoint()
        response = self.session.get(f"{endpoint}/sdapi/v1/progress")
        return response.json()

    def _decode_images(self, base64_images):
        """Decodifica imagens base64 para PIL Images"""
        images = []
        for img_data in base64_images:
            image = Image.open(BytesIO(base64.b64decode(img_data)))
            images.append(image)
        return images


# Exemplo de uso
if __name__ == "__main__":
    # Inicializar cliente
    client = ArxisGPUClient(
        local_url="http://127.0.0.1:7860",
        remote_url="http://192.168.1.100:7861"  # IP do servidor Arxis com GPUs
    )

    # Listar modelos disponíveis
    print("📦 Modelos disponíveis:")
    models = client.get_models()
    for model in models[:3]:  # Mostrar primeiros 3
        print(f"  - {model['title']}")

    # Gerar imagem
    images = client.txt2img(
        prompt="a beautiful landscape with mountains and lake, sunset, 4k, highly detailed",
        negative_prompt="blurry, low quality, distorted",
        steps=25,
        width=768,
        height=512
    )

    # Salvar resultado
    images[0].save("output.png")
    print("💾 Imagem salva como: output.png")

    # Processar com img2img
    processed = client.img2img(
        image=images[0],
        prompt="same scene but at night with stars",
        strength=0.6
    )
    processed[0].save("output_night.png")
    print("💾 Imagem processada salva como: output_night.png")
