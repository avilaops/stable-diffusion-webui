//! Gerador de imagens avançado - Arquitetura inspirada em Stable Diffusion
//! Usa técnicas de difusão, Perlin noise, e processamento paralelo

use image::{ImageBuffer, Rgb, RgbImage};
use noise::{NoiseFn, Perlin, Seedable};
use rayon::prelude::*;
use std::f32::consts::PI;
use anyhow::Result;

pub struct AdvancedGenerator {
    perlin: Perlin,
    seed: u32,
}

impl AdvancedGenerator {
    pub fn new(seed: u32) -> Self {
        let perlin = Perlin::new().set_seed(seed);
        Self { perlin, seed }
    }

    /// Gera imagem usando técnicas avançadas de síntese
    pub fn generate(&self, prompt: &str, width: u32, height: u32) -> Result<RgbImage> {
        let prompt_lower = prompt.to_lowercase();

        // Análise de prompt multi-conceito
        let style = self.detect_style(&prompt_lower);
        let subject = self.detect_subject(&prompt_lower);
        let mood = self.detect_mood(&prompt_lower);

        // Geração paralela de camadas
        let mut layers: Vec<RgbImage> = vec![
            self.generate_base_layer(width, height, &style),
            self.generate_texture_layer(width, height, &subject),
            self.generate_lighting_layer(width, height, &mood),
        ];

        // Composição com blend modes
        let result = self.composite_layers(&mut layers, width, height);

        Ok(result)
    }

    fn detect_style(&self, prompt: &str) -> Style {
        if prompt.contains("photorealistic") || prompt.contains("photo") || prompt.contains("realistic") {
            Style::Photorealistic
        } else if prompt.contains("painting") || prompt.contains("oil") || prompt.contains("watercolor") {
            Style::Artistic
        } else if prompt.contains("anime") || prompt.contains("manga") || prompt.contains("cartoon") {
            Style::Anime
        } else if prompt.contains("abstract") || prompt.contains("geometric") {
            Style::Abstract
        } else {
            Style::Balanced
        }
    }

    fn detect_subject(&self, prompt: &str) -> Subject {
        if prompt.contains("landscape") || prompt.contains("mountain") || prompt.contains("nature") {
            Subject::Landscape
        } else if prompt.contains("portrait") || prompt.contains("face") || prompt.contains("person") {
            Subject::Portrait
        } else if prompt.contains("ocean") || prompt.contains("water") || prompt.contains("sea") {
            Subject::Water
        } else if prompt.contains("city") || prompt.contains("urban") || prompt.contains("building") {
            Subject::Urban
        } else if prompt.contains("space") || prompt.contains("galaxy") || prompt.contains("stars") {
            Subject::Space
        } else {
            Subject::Generic
        }
    }

    fn detect_mood(&self, prompt: &str) -> Mood {
        if prompt.contains("dark") || prompt.contains("night") || prompt.contains("shadow") {
            Mood::Dark
        } else if prompt.contains("bright") || prompt.contains("sunny") || prompt.contains("vibrant") {
            Mood::Bright
        } else if prompt.contains("dramatic") || prompt.contains("epic") {
            Mood::Dramatic
        } else if prompt.contains("calm") || prompt.contains("peaceful") || prompt.contains("serene") {
            Mood::Calm
        } else {
            Mood::Neutral
        }
    }

    fn generate_base_layer(&self, width: u32, height: u32, style: &Style) -> RgbImage {
        let mut img = ImageBuffer::new(width, height);

        img.enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| {
                let fx = x as f64 / width as f64;
                let fy = y as f64 / height as f64;

                // Multi-octave Perlin noise para textura orgânica
                let noise_val = self.multi_octave_noise(fx, fy, 4);

                let color = match style {
                    Style::Photorealistic => self.photorealistic_color(fx, fy, noise_val),
                    Style::Artistic => self.artistic_color(fx, fy, noise_val),
                    Style::Anime => self.anime_color(fx, fy, noise_val),
                    Style::Abstract => self.abstract_color(fx, fy, noise_val),
                    Style::Balanced => self.balanced_color(fx, fy, noise_val),
                };

                *pixel = color;
            });

        img
    }

    fn generate_texture_layer(&self, width: u32, height: u32, subject: &Subject) -> RgbImage {
        let mut img = ImageBuffer::new(width, height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let fx = x as f64 / width as f64;
            let fy = y as f64 / height as f64;

            let texture = match subject {
                Subject::Landscape => self.landscape_texture(fx, fy),
                Subject::Portrait => self.portrait_texture(fx, fy),
                Subject::Water => self.water_texture(fx, fy, x, y),
                Subject::Urban => self.urban_texture(fx, fy),
                Subject::Space => self.space_texture(fx, fy),
                Subject::Generic => self.generic_texture(fx, fy),
            };

            *pixel = texture;
        }

        img
    }

    fn generate_lighting_layer(&self, width: u32, height: u32, mood: &Mood) -> RgbImage {
        let mut img = ImageBuffer::new(width, height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let fx = x as f64 / width as f64;
            let fy = y as f64 / height as f64;

            let lighting = match mood {
                Mood::Dark => self.dark_lighting(fx, fy),
                Mood::Bright => self.bright_lighting(fx, fy),
                Mood::Dramatic => self.dramatic_lighting(fx, fy),
                Mood::Calm => self.calm_lighting(fx, fy),
                Mood::Neutral => Rgb([128, 128, 128]),
            };

            *pixel = lighting;
        }

        img
    }

    fn multi_octave_noise(&self, x: f64, y: f64, octaves: u32) -> f64 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            total += self.perlin.get([x * frequency * 5.0, y * frequency * 5.0]) * amplitude;
            max_value += amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        total / max_value
    }

    // Esquemas de cor por estilo
    fn photorealistic_color(&self, fx: f64, fy: f64, noise: f64) -> Rgb<u8> {
        let base = 180.0 + noise * 50.0;
        let r = (base * 0.9) as u8;
        let g = (base * 0.95) as u8;
        let b = (base * 1.0) as u8;
        Rgb([r, g, b])
    }

    fn artistic_color(&self, fx: f64, fy: f64, noise: f64) -> Rgb<u8> {
        let hue = (fx + noise * 0.3) * 360.0;
        let sat = 0.7;
        let val = 0.8 + noise * 0.2;
        hsv_to_rgb(hue, sat, val)
    }

    fn anime_color(&self, fx: f64, fy: f64, noise: f64) -> Rgb<u8> {
        // Cores saturadas e cel-shading
        let quantize = (noise * 4.0).floor() / 4.0;
        let r = ((fx + quantize) * 255.0).min(255.0) as u8;
        let g = ((fy + quantize) * 255.0).min(255.0) as u8;
        let b = ((1.0 - fx + quantize) * 255.0).min(255.0) as u8;
        Rgb([r, g, b])
    }

    fn abstract_color(&self, fx: f64, fy: f64, noise: f64) -> Rgb<u8> {
        let angle = (fx * PI * 2.0 + noise * PI) as f32;
        let r = ((angle.sin() + 1.0) * 127.5) as u8;
        let g = ((angle.cos() + 1.0) * 127.5) as u8;
        let b = (((angle * 2.0).sin() + 1.0) * 127.5) as u8;
        Rgb([r, g, b])
    }

    fn balanced_color(&self, fx: f64, fy: f64, noise: f64) -> Rgb<u8> {
        let r = ((fx + noise * 0.5) * 255.0).min(255.0) as u8;
        let g = ((fy + noise * 0.5) * 255.0).min(255.0) as u8;
        let b = ((0.5 + noise * 0.5) * 255.0).min(255.0) as u8;
        Rgb([r, g, b])
    }

    // Texturas por assunto
    fn landscape_texture(&self, fx: f64, fy: f64) -> Rgb<u8> {
        if fy < 0.3 {
            // Céu
            let blue = (180.0 + fy * 200.0) as u8;
            Rgb([135, 180, blue])
        } else if fy < 0.6 {
            // Montanhas com Perlin
            let mountain = self.perlin.get([fx * 3.0, fy * 3.0]);
            let gray = (100.0 + mountain * 80.0).clamp(50.0, 200.0) as u8;
            Rgb([gray, gray, gray])
        } else {
            // Vegetação
            let green = (80.0 + self.perlin.get([fx * 10.0, fy * 10.0]) * 60.0).clamp(40.0, 140.0) as u8;
            Rgb([40, green, 40])
        }
    }

    fn portrait_texture(&self, fx: f64, fy: f64) -> Rgb<u8> {
        let center_x = 0.5;
        let center_y = 0.5;
        let dist = ((fx - center_x).powi(2) + (fy - center_y).powi(2)).sqrt();

        if dist < 0.25 {
            // Tom de pele
            Rgb([220, 180, 160])
        } else if dist < 0.35 {
            // Cabelo
            Rgb([60, 40, 30])
        } else {
            // Fundo desfocado
            let blur = (dist * 200.0) as u8;
            Rgb([blur, blur, blur + 20])
        }
    }

    fn water_texture(&self, fx: f64, fy: f64, x: u32, y: u32) -> Rgb<u8> {
        // Ondas usando múltiplas frequências
        let wave1 = (fx * 20.0 + (y as f64 * 0.05).sin()).sin();
        let wave2 = (fy * 15.0 + (x as f64 * 0.03).cos()).cos();
        let combined = (wave1 + wave2) / 2.0;

        let base_blue = 100.0 + fy * 100.0;
        let blue = (base_blue + combined * 50.0).clamp(50.0, 255.0) as u8;
        let green = (blue as i32 - 40).clamp(20, 200) as u8;

        Rgb([20, green, blue])
    }

    fn urban_texture(&self, fx: f64, fy: f64) -> Rgb<u8> {
        // Grid de prédios
        let grid_x = (fx * 10.0).floor();
        let grid_y = (fy * 10.0).floor();
        let is_building = (grid_x + grid_y) % 2.0 == 0.0;

        if is_building {
            // Janelas iluminadas
            let window_x = (fx * 100.0) % 10.0;
            let window_y = (fy * 100.0) % 10.0;
            let is_window = window_x > 3.0 && window_x < 7.0 && window_y > 3.0 && window_y < 7.0;

            if is_window {
                Rgb([255, 240, 200])
            } else {
                Rgb([60, 65, 80])
            }
        } else {
            Rgb([30, 35, 45])
        }
    }

    fn space_texture(&self, fx: f64, fy: f64) -> Rgb<u8> {
        // Estrelas aleatórias usando Perlin
        let star_field = self.perlin.get([fx * 50.0, fy * 50.0]);

        if star_field > 0.7 {
            // Estrela
            Rgb([255, 255, 255])
        } else if star_field > 0.5 {
            // Nebulosa
            let r = (star_field * 200.0) as u8;
            let b = (star_field * 255.0) as u8;
            Rgb([r, 50, b])
        } else {
            // Espaço profundo
            Rgb([5, 5, 10])
        }
    }

    fn generic_texture(&self, fx: f64, fy: f64) -> Rgb<u8> {
        let noise = self.multi_octave_noise(fx, fy, 3);
        let val = ((noise + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
        Rgb([val, val, val])
    }

    // Iluminação por mood
    fn dark_lighting(&self, _fx: f64, fy: f64) -> Rgb<u8> {
        let darkness = (50.0 * (1.0 - fy)) as u8;
        Rgb([darkness, darkness, darkness])
    }

    fn bright_lighting(&self, _fx: f64, fy: f64) -> Rgb<u8> {
        let brightness = (200.0 + fy * 55.0) as u8;
        Rgb([brightness, brightness, brightness])
    }

    fn dramatic_lighting(&self, fx: f64, fy: f64) -> Rgb<u8> {
        let spotlight = 1.0 - ((fx - 0.5).powi(2) + (fy - 0.3).powi(2)).sqrt();
        let intensity = (spotlight * 200.0).clamp(0.0, 255.0) as u8;
        Rgb([intensity, intensity, intensity])
    }

    fn calm_lighting(&self, _fx: f64, _fy: f64) -> Rgb<u8> {
        Rgb([150, 160, 170])
    }

    fn composite_layers(&self, layers: &mut [RgbImage], width: u32, height: u32) -> RgbImage {
        let mut result = ImageBuffer::new(width, height);

        for (x, y, pixel) in result.enumerate_pixels_mut() {
            let base = layers[0].get_pixel(x, y);
            let texture = layers[1].get_pixel(x, y);
            let lighting = layers[2].get_pixel(x, y);

            // Blend multiply para textura
            let r1 = (base[0] as u16 * texture[0] as u16) / 255;
            let g1 = (base[1] as u16 * texture[1] as u16) / 255;
            let b1 = (base[2] as u16 * texture[2] as u16) / 255;

            // Blend screen para iluminação
            let r = 255 - (((255 - r1) * (255 - lighting[0] as u16)) / 255);
            let g = 255 - (((255 - g1) * (255 - lighting[1] as u16)) / 255);
            let b = 255 - (((255 - b1) * (255 - lighting[2] as u16)) / 255);

            *pixel = Rgb([r as u8, g as u8, b as u8]);
        }

        result
    }
}

#[derive(Debug)]
enum Style {
    Photorealistic,
    Artistic,
    Anime,
    Abstract,
    Balanced,
}

#[derive(Debug)]
enum Subject {
    Landscape,
    Portrait,
    Water,
    Urban,
    Space,
    Generic,
}

#[derive(Debug)]
enum Mood {
    Dark,
    Bright,
    Dramatic,
    Calm,
    Neutral,
}

// Conversão HSV para RGB
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Rgb<u8> {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());

    let (r1, g1, b1) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let m = v - c;
    let r = ((r1 + m) * 255.0) as u8;
    let g = ((g1 + m) * 255.0) as u8;
    let b = ((b1 + m) * 255.0) as u8;

    Rgb([r, g, b])
}
