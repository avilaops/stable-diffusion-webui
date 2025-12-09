//! Gerador de imagens AVANÇADO - Usa matemática procedural avançada
//! Sistema de geração multi-camada com Perlin noise, FFT-like patterns, etc.

use std::f32::consts::PI;

pub struct NativeImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>, // RGB: [r,g,b,r,g,b,...]
}

impl NativeImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height * 3],
        }
    }

    pub fn from_rgb_bytes(width: usize, height: usize, data: Vec<u8>) -> Result<Self, String> {
        let expected = width * height * 3;
        if data.len() != expected {
            return Err(format!(
                "Dados RGB com tamanho incorreto: esperado {}, recebeu {}",
                expected,
                data.len()
            ));
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let idx = (y * self.width + x) * 3;
        if idx + 2 < self.data.len() {
            self.data[idx] = r;
            self.data[idx + 1] = g;
            self.data[idx + 2] = b;
        }
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
        for chunk in self.data.chunks(3) {
            ppm.push_str(&format!("{} {} {} ", chunk[0], chunk[1], chunk[2]));
        }
        ppm
    }

    pub fn to_base64_png(&self) -> String {
        // Retornar PPM (ASCII formato) convertido para data URI
        // Muito mais simples e seguro
        let ppm = self.to_ppm();
        format!("data:text/plain;base64,{}", base64_encode(ppm.as_bytes()))
    }

    pub fn to_data_url_svg(&self) -> String {
        // SVG com blocos 2x2 pixels (ótimo equilíbrio qualidade/tamanho)
        let block_size = 2;
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">"#,
            self.width, self.height
        );

        for y in (0..self.height).step_by(block_size) {
            for x in (0..self.width).step_by(block_size) {
                let idx = (y * self.width + x) * 3;
                if idx + 2 < self.data.len() {
                    let r = self.data[idx];
                    let g = self.data[idx + 1];
                    let b = self.data[idx + 2];

                    svg.push_str(&format!(
                        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="rgb({},{},{})"/>"#,
                        x, y, block_size, block_size, r, g, b
                    ));
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }

    /// Salva imagem como PPM (formato simples ASCII)
    pub fn save_png(&self, path: &str) {
        let ppm_content = self.to_ppm();
        // Usar extensão .ppm em vez de .png (formato simples sem compressão)
        let ppm_path = path.replace(".png", ".ppm");

        if let Err(e) = std::fs::write(&ppm_path, ppm_content) {
            eprintln!("⚠️  Erro ao salvar: {}", e);
        } else {
            println!("💾 Imagem salva: {}", ppm_path);
        }
    }
}

fn write_chunk(output: &mut Vec<u8>, data: &[u8]) {
    let len = (data.len() - 4) as u32;
    output.extend_from_slice(&len.to_be_bytes());
    output.extend_from_slice(data);

    let crc = crc32(data);
    output.extend_from_slice(&crc.to_be_bytes());
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFF_u32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xEDB88320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn base64_encode(data: &[u8]) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let b1 = chunk[0];
        let b2 = *chunk.get(1).unwrap_or(&0);
        let b3 = *chunk.get(2).unwrap_or(&0);

        result.push(BASE64_CHARS[(b1 >> 2) as usize] as char);
        result.push(BASE64_CHARS[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);
        result.push(if chunk.len() > 1 {
            BASE64_CHARS[(((b2 & 0x0F) << 2) | (b3 >> 6)) as usize] as char
        } else {
            '='
        });
        result.push(if chunk.len() > 2 {
            BASE64_CHARS[(b3 & 0x3F) as usize] as char
        } else {
            '='
        });
    }

    result
}

pub struct UltraLightGenerator {
    seed: u32,
}

impl UltraLightGenerator {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn generate(&self, prompt: &str, width: usize, height: usize) -> NativeImage {
        let mut img = NativeImage::new(width, height);
        let prompt_lower = prompt.to_lowercase();

        // Detecção de tipo com geradores avançados
        if prompt_lower.contains("landscape")
            || prompt_lower.contains("mountain")
            || prompt_lower.contains("montanha")
        {
            self.gen_landscape_advanced(&mut img);
        } else if prompt_lower.contains("ocean")
            || prompt_lower.contains("water")
            || prompt_lower.contains("mar")
            || prompt_lower.contains("oceano")
        {
            self.gen_ocean_advanced(&mut img);
        } else if prompt_lower.contains("sunset")
            || prompt_lower.contains("por do sol")
            || prompt_lower.contains("sol")
            || prompt_lower.contains("pôr do sol")
        {
            self.gen_sunset_advanced(&mut img);
        } else if prompt_lower.contains("forest") || prompt_lower.contains("floresta") {
            self.gen_forest_advanced(&mut img);
        } else if prompt_lower.contains("city") || prompt_lower.contains("cidade") {
            self.gen_city_advanced(&mut img);
        } else if prompt_lower.contains("space")
            || prompt_lower.contains("espaço")
            || prompt_lower.contains("estrela")
        {
            self.gen_space_advanced(&mut img);
        } else if prompt_lower.contains("fire")
            || prompt_lower.contains("fogo")
            || prompt_lower.contains("flame")
        {
            self.gen_fire_advanced(&mut img);
        } else if prompt_lower.contains("dog")
            || prompt_lower.contains("cachorro")
            || prompt_lower.contains("animal")
        {
            self.gen_animal_abstract(&mut img, prompt);
        } else {
            self.gen_abstract_advanced(&mut img, prompt);
        }

        img
    }

    // Perlin Noise multi-octave (MUITO mais suave e realista)
    fn perlin_noise(&self, x: f32, y: f32, octaves: usize) -> f32 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            value += self.noise(x * frequency, y * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        value / max_value
    }

    fn noise(&self, x: f32, y: f32) -> f32 {
        let seed_offset = self.seed as f32 * 0.01;
        let n = ((x + seed_offset) * 12.9898 + y * 78.233).sin() * 43758.5453;
        n - n.floor()
    }

    // GERADOR AVANÇADO: Paisagem fotorealística
    fn gen_landscape_advanced(&self, img: &mut NativeImage) {
        for y in 0..img.height {
            for x in 0..img.width {
                let fx = x as f32 / img.width as f32;
                let fy = y as f32 / img.height as f32;

                // Múltiplas camadas de Perlin noise
                let terrain = self.perlin_noise(fx * 4.0, fy * 4.0, 4);
                let detail = self.perlin_noise(fx * 20.0, fy * 20.0, 3);

                if fy < 0.4 + terrain * 0.1 {
                    // Céu gradiente realista (azul → laranja no horizonte)
                    let horizon_factor = (fy / 0.4).powf(0.5);
                    let r = (100.0 + horizon_factor * 155.0) as u8;
                    let g = (150.0 + horizon_factor * 105.0) as u8;
                    let b = (255.0 - horizon_factor * 50.0) as u8;

                    // Adicionar nuvens suaves
                    let clouds = self.perlin_noise(fx * 6.0, fy * 3.0 + 100.0, 3);
                    let cloud_brightness = if clouds > 0.55 {
                        (clouds * 255.0) as u8
                    } else {
                        0
                    };

                    img.set_pixel(
                        x,
                        y,
                        r.saturating_add(cloud_brightness / 2),
                        g.saturating_add(cloud_brightness / 2),
                        b.saturating_add(cloud_brightness / 3),
                    );
                } else if fy < 0.65 + terrain * 0.15 {
                    // Montanhas com iluminação e sombra
                    let elevation = terrain + detail * 0.3;
                    let shadow = (elevation * 100.0 + 60.0) as u8;
                    let r = shadow;
                    let g = (shadow as f32 * 0.9) as u8;
                    let b = (shadow as f32 * 0.8) as u8;
                    img.set_pixel(x, y, r, g, b);
                } else {
                    // Grama/floresta com textura rica
                    let grass = self.perlin_noise(fx * 30.0, fy * 30.0, 4);
                    let r = (30.0 + grass * 40.0) as u8;
                    let g = (100.0 + grass * 80.0) as u8;
                    let b = (30.0 + grass * 30.0) as u8;
                    img.set_pixel(x, y, r, g, b);
                }
            }
        }
    }

    // GERADOR AVANÇADO: Oceano com ondas
    fn gen_ocean_advanced(&self, img: &mut NativeImage) {
        for y in 0..img.height {
            for x in 0..img.width {
                let fx = x as f32 / img.width as f32;
                let fy = y as f32 / img.height as f32;

                if fy < 0.3 {
                    // Céu
                    let b = (200.0 + fy * 55.0) as u8;
                    img.set_pixel(x, y, 180, 220, b);
                } else {
                    // Água com ondas multi-camada
                    let wave1 = self.perlin_noise(fx * 8.0, fy * 4.0, 3);
                    let wave2 = self.perlin_noise(fx * 15.0 + 100.0, fy * 6.0, 2);
                    let foam = self.perlin_noise(fx * 40.0, fy * 20.0, 2);

                    let depth = (fy - 0.3) / 0.7;
                    let base_blue = 150.0 + depth * 50.0;

                    let r =
                        (20.0 + wave1 * 40.0 + (foam > 0.7).then_some(100.0).unwrap_or(0.0)) as u8;
                    let g = (100.0 + wave2 * 50.0) as u8;
                    let b = (base_blue + wave1 * 60.0) as u8;

                    img.set_pixel(x, y, r, g, b);
                }
            }
        }
    }

    // GERADOR AVANÇADO: Pôr do sol cinematográfico
    fn gen_sunset_advanced(&self, img: &mut NativeImage) {
        for y in 0..img.height {
            for x in 0..img.width {
                let fx = x as f32 / img.width as f32;
                let fy = y as f32 / img.height as f32;

                // Gradiente complexo do pôr do sol
                let horizon = 0.5;
                let dist_to_horizon = (fy - horizon).abs();

                // Camadas de cor do pôr do sol
                let sun_glow = (1.0 - dist_to_horizon * 3.0).max(0.0).powf(2.0);
                let orange_band = (1.0 - (dist_to_horizon - 0.1).abs() * 10.0).max(0.0);

                let r = (255.0 * (0.6 + sun_glow * 0.4 + orange_band * 0.3)) as u8;
                let g = ((150.0 - fy * 100.0) * (0.8 + sun_glow * 0.2)) as u8;
                let b = ((100.0 - fy * 80.0) * (0.5 + sun_glow * 0.1)) as u8;

                // Nuvens dramáticas
                let clouds = self.perlin_noise(fx * 5.0, fy * 3.0, 4);
                let cloud_alpha = if clouds > 0.5 {
                    (clouds - 0.5) * 2.0
                } else {
                    0.0
                };

                let final_r = (r as f32 * (1.0 - cloud_alpha * 0.3)) as u8;
                let final_g = (g as f32 * (1.0 - cloud_alpha * 0.5)) as u8;
                let final_b = (b as f32 * (1.0 - cloud_alpha * 0.6)) as u8;

                img.set_pixel(x, y, final_r, final_g, final_b);
            }
        }
    }

    // GERADOR AVANÇADO: Floresta densa
    fn gen_forest_advanced(&self, img: &mut NativeImage) {
        for y in 0..img.height {
            for x in 0..img.width {
                let fx = x as f32 / img.width as f32;
                let fy = y as f32 / img.height as f32;

                // Árvores procedurais
                let tree_noise = self.perlin_noise(fx * 10.0, fy * 8.0, 4);
                let leaf_detail = self.perlin_noise(fx * 40.0, fy * 40.0, 3);

                let is_trunk = tree_noise > 0.7 && (fx * 100.0) % 10.0 < 2.0;

                if is_trunk {
                    // Troncos escuros
                    let brown = (40.0 + leaf_detail * 30.0) as u8;
                    img.set_pixel(x, y, brown, brown / 2, brown / 3);
                } else {
                    // Folhagem verde rica
                    let depth = tree_noise + leaf_detail * 0.3;
                    let r = (20.0 + depth * 40.0) as u8;
                    let g = (80.0 + depth * 120.0) as u8;
                    let b = (30.0 + depth * 50.0) as u8;
                    img.set_pixel(x, y, r, g, b);
                }
            }
        }
    }

    fn gen_forest(&self, img: &mut NativeImage) {
        self.gen_forest_advanced(img);
    }

    fn gen_city_advanced(&self, img: &mut NativeImage) {
        for y in 0..img.height {
            for x in 0..img.width {
                let fx = x as f32 / img.width as f32;
                let fy = y as f32 / img.height as f32;

                // Skyline procedural
                let building_pattern = self.perlin_noise(fx * 20.0, 0.0, 2);
                let building_height = building_pattern * 0.6 + 0.2;

                if fy < 0.3 {
                    // Céu noturno/dusk
                    let r = (30.0 + fy * 100.0) as u8;
                    let g = (40.0 + fy * 120.0) as u8;
                    let b = (80.0 + fy * 150.0) as u8;
                    img.set_pixel(x, y, r, g, b);
                } else if fy > building_height {
                    // Prédios
                    let windows = self.perlin_noise(fx * 50.0, fy * 80.0, 1);
                    let lit = windows > 0.6;
                    let gray = if lit {
                        250
                    } else {
                        (40.0 + building_pattern * 30.0) as u8
                    };
                    img.set_pixel(x, y, gray, gray, if lit { 200 } else { gray });
                } else {
                    // Reflexos/rua
                    let r = (20.0 + building_pattern * 50.0) as u8;
                    img.set_pixel(x, y, r, r / 2, r / 4);
                }
            }
        }
    }

    // GERADOR AVANÇADO: Espaço com estrelas e nebulosas
    fn gen_space_advanced(&self, img: &mut NativeImage) {
        for y in 0..img.height {
            for x in 0..img.width {
                let fx = x as f32 / img.width as f32;
                let fy = y as f32 / img.height as f32;

                // Nebulosa com cores variadas
                let nebula1 = self.perlin_noise(fx * 3.0, fy * 3.0, 5);
                let nebula2 = self.perlin_noise(fx * 4.0 + 100.0, fy * 4.0, 4);

                let r = (nebula1 * 150.0 + 20.0) as u8;
                let g = (nebula2 * 100.0 + 10.0) as u8;
                let b = ((nebula1 + nebula2) * 80.0 + 40.0) as u8;

                // Estrelas brilhantes
                let star = self.noise(fx * 200.0, fy * 200.0);
                if star > 0.98 {
                    img.set_pixel(x, y, 255, 255, 255);
                } else if star > 0.95 {
                    img.set_pixel(x, y, 200, 200, 255);
                } else {
                    img.set_pixel(x, y, r, g, b);
                }
            }
        }
    }

    // GERADOR AVANÇADO: Fogo com chamas dinâmicas
    fn gen_fire_advanced(&self, img: &mut NativeImage) {
        for y in 0..img.height {
            for x in 0..img.width {
                let fx = x as f32 / img.width as f32;
                let fy = y as f32 / img.height as f32;

                // Chamas turbulentas
                let flame1 = self.perlin_noise(fx * 6.0, fy * 8.0 - self.seed as f32 * 0.1, 4);
                let flame2 = self.perlin_noise(fx * 12.0, fy * 10.0, 3);
                let intensity = (1.0 - fy) * (flame1 + flame2 * 0.5);

                let r = (255.0 * intensity.min(1.0)) as u8;
                let g = (180.0 * (intensity - 0.3).max(0.0).min(1.0)) as u8;
                let b = (50.0 * (intensity - 0.7).max(0.0).min(1.0)) as u8;

                // Brasas
                let ember = self.noise(fx * 80.0, fy * 80.0);
                if ember > 0.9 && fy > 0.6 {
                    img.set_pixel(x, y, 255, 100, 0);
                } else {
                    img.set_pixel(x, y, r, g, b);
                }
            }
        }
    }

    // GERADOR: Animal abstrato (padrão orgânico)
    fn gen_animal_abstract(&self, img: &mut NativeImage, prompt: &str) {
        // Cores baseadas no prompt
        let is_dog =
            prompt.to_lowercase().contains("dog") || prompt.to_lowercase().contains("cachorro");
        let base_color = if is_dog {
            (180, 140, 100)
        } else {
            (120, 100, 80)
        };

        for y in 0..img.height {
            for x in 0..img.width {
                let fx = x as f32 / img.width as f32;
                let fy = y as f32 / img.height as f32;

                // Forma orgânica (silhueta)
                let body = self.perlin_noise(fx * 3.0, fy * 3.0, 4);
                let fur_texture = self.perlin_noise(fx * 30.0, fy * 30.0, 3);

                if body > 0.3 {
                    // Dentro da forma - adicionar textura de pelo
                    let r = (base_color.0 as f32 * (0.7 + fur_texture * 0.3)) as u8;
                    let g = (base_color.1 as f32 * (0.7 + fur_texture * 0.3)) as u8;
                    let b = (base_color.2 as f32 * (0.7 + fur_texture * 0.3)) as u8;
                    img.set_pixel(x, y, r, g, b);
                } else {
                    // Fundo suave
                    let bg_r = (200.0 - fy * 50.0) as u8;
                    let bg_g = (220.0 - fy * 60.0) as u8;
                    let bg_b = (240.0 - fy * 40.0) as u8;
                    img.set_pixel(x, y, bg_r, bg_g, bg_b);
                }
            }
        }
    }

    // GERADOR AVANÇADO: Arte abstrata baseada no prompt
    fn gen_abstract_advanced(&self, img: &mut NativeImage, prompt: &str) {
        // Análise de cor do prompt
        let (hue_base, saturation) = self.analyze_prompt_colors(prompt);

        for y in 0..img.height {
            for x in 0..img.width {
                let fx = x as f32 / img.width as f32;
                let fy = y as f32 / img.height as f32;

                // Padrão fractal multi-camada
                let pattern1 = self.perlin_noise(fx * 5.0, fy * 5.0, 5);
                let pattern2 = self.perlin_noise(fx * 10.0 + 50.0, fy * 10.0, 4);
                let pattern3 = self.perlin_noise(fx * 20.0, fy * 20.0 + 100.0, 3);

                let combined = (pattern1 + pattern2 * 0.5 + pattern3 * 0.3) / 1.8;

                // Converter para RGB com base no hue
                let angle = hue_base + combined * PI * 2.0;
                let r = ((angle.cos() * 0.5 + 0.5) * 255.0 * saturation) as u8;
                let g = (((angle + PI * 0.66).cos() * 0.5 + 0.5) * 255.0 * saturation) as u8;
                let b = (((angle + PI * 1.33).cos() * 0.5 + 0.5) * 255.0 * saturation) as u8;

                img.set_pixel(x, y, r, g, b);
            }
        }
    }

    fn analyze_prompt_colors(&self, prompt: &str) -> (f32, f32) {
        let p = prompt.to_lowercase();

        // Determinar hue base (matiz) baseado em palavras-chave
        let hue = if p.contains("red") || p.contains("vermelho") {
            0.0
        } else if p.contains("orange") || p.contains("laranja") {
            0.3
        } else if p.contains("yellow") || p.contains("amarelo") {
            0.7
        } else if p.contains("green") || p.contains("verde") {
            1.5
        } else if p.contains("blue") || p.contains("azul") {
            3.0
        } else if p.contains("purple") || p.contains("roxo") {
            4.0
        } else {
            self.seed as f32 * 0.01
        }; // Aleatório baseado na seed

        // Saturação baseada em intensidade
        let saturation = if p.contains("vibrant") || p.contains("bright") || p.contains("colorful")
        {
            1.0
        } else if p.contains("pastel") || p.contains("soft") {
            0.6
        } else if p.contains("dark") || p.contains("escuro") {
            0.4
        } else {
            0.8
        };

        (hue, saturation)
    }

    // Manter compatibilidade com métodos antigos
    fn gen_sunset(&self, img: &mut NativeImage) {
        self.gen_sunset_advanced(img);
    }

    fn gen_city(&self, img: &mut NativeImage) {
        self.gen_city_advanced(img);
    }

    fn gen_space(&self, img: &mut NativeImage) {
        self.gen_space_advanced(img);
    }

    fn gen_abstract(&self, img: &mut NativeImage, prompt: &str) {
        self.gen_abstract_advanced(img, prompt);
    }

    fn gen_ocean(&self, img: &mut NativeImage) {
        self.gen_ocean_advanced(img);
    }
}
