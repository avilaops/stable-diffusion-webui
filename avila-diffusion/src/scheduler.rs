//! Schedulers - Processos de difusão (DDPM, Euler, DPM++, etc.)

use std::f32::consts::PI;

/// Tipo de scheduler
#[derive(Debug, Clone, Copy)]
pub enum SchedulerType {
    DDPM,      // Denoising Diffusion Probabilistic Models
    Euler,     // Euler method
    EulerA,    // Euler Ancestral
    DPMSolver, // DPM-Solver++
}

/// Scheduler para o processo de difusão
pub struct Scheduler {
    scheduler_type: SchedulerType,
    pub num_steps: usize,
    beta_start: f32,
    beta_end: f32,
    timesteps: Vec<f32>,
    alphas: Vec<f32>,
}

impl Scheduler {
    pub fn new(scheduler_type: SchedulerType, num_steps: usize) -> Self {
        println!(
            "⏱️  Inicializando Scheduler: {:?} ({} steps)",
            scheduler_type, num_steps
        );

        // Parâmetros padrão do SD
        let beta_start = 0.00085;
        let beta_end = 0.012;

        // Gerar timesteps e alphas
        let (timesteps, alphas) = Self::compute_schedule(num_steps, beta_start, beta_end);

        Self {
            scheduler_type,
            num_steps,
            beta_start,
            beta_end,
            timesteps,
            alphas,
        }
    }

    /// Computa schedule de noise (betas -> alphas -> sigmas)
    fn compute_schedule(steps: usize, beta_start: f32, beta_end: f32) -> (Vec<f32>, Vec<f32>) {
        let mut betas = Vec::with_capacity(steps);
        let mut alphas = Vec::with_capacity(steps);

        // Linear schedule
        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let beta = beta_start + t * (beta_end - beta_start);
            betas.push(beta);
        }

        // Alphas (cumulativo)
        let mut alpha_cumprod = 1.0;
        for beta in &betas {
            alpha_cumprod *= 1.0 - beta;
            alphas.push(alpha_cumprod);
        }

        // Timesteps (reverso)
        let timesteps: Vec<f32> = (0..steps)
            .map(|i| 1.0 - (i as f32 / steps as f32))
            .collect();

        (timesteps, alphas)
    }

    /// Adiciona noise ao latent (forward diffusion)
    pub fn add_noise(&self, latents: &[f32], noise: &[f32], timestep_idx: usize) -> Vec<f32> {
        let alpha = self.alphas[timestep_idx.min(self.alphas.len() - 1)];
        let sqrt_alpha = alpha.sqrt();
        let sqrt_one_minus_alpha = (1.0 - alpha).sqrt();

        latents
            .iter()
            .zip(noise.iter())
            .map(|(l, n)| l * sqrt_alpha + n * sqrt_one_minus_alpha)
            .collect()
    }

    /// Step de denoising
    pub fn step(&self, model_output: &[f32], latents: &[f32], timestep_idx: usize) -> Vec<f32> {
        match self.scheduler_type {
            SchedulerType::DDPM => self.step_ddpm(model_output, latents, timestep_idx),
            SchedulerType::Euler => self.step_euler(model_output, latents, timestep_idx),
            SchedulerType::EulerA => self.step_euler_ancestral(model_output, latents, timestep_idx),
            SchedulerType::DPMSolver => self.step_dpm(model_output, latents, timestep_idx),
        }
    }

    /// DDPM step
    fn step_ddpm(&self, model_output: &[f32], latents: &[f32], timestep_idx: usize) -> Vec<f32> {
        let alpha = self.alphas[timestep_idx];
        let alpha_prev = if timestep_idx > 0 {
            self.alphas[timestep_idx - 1]
        } else {
            1.0
        };

        let _beta = 1.0 - alpha / alpha_prev;
        let sqrt_alpha = alpha.sqrt();
        let sqrt_one_minus_alpha = (1.0 - alpha).sqrt();

        // Predizer x0 do noise
        latents
            .iter()
            .zip(model_output.iter())
            .map(|(lat, pred)| {
                let pred_original = (lat - sqrt_one_minus_alpha * pred) / sqrt_alpha;
                let direction = pred;

                // Direção para timestep anterior
                alpha_prev.sqrt() * pred_original + (1.0 - alpha_prev).sqrt() * direction
            })
            .collect()
    }

    /// Euler step
    fn step_euler(&self, model_output: &[f32], latents: &[f32], timestep_idx: usize) -> Vec<f32> {
        let timestep = self.timesteps[timestep_idx];
        let dt = if timestep_idx > 0 {
            self.timesteps[timestep_idx - 1] - timestep
        } else {
            -timestep
        };

        latents
            .iter()
            .zip(model_output.iter())
            .map(|(lat, pred)| lat + pred * dt)
            .collect()
    }

    /// Euler Ancestral step (com noise injection)
    fn step_euler_ancestral(
        &self,
        model_output: &[f32],
        latents: &[f32],
        timestep_idx: usize,
    ) -> Vec<f32> {
        let mut result = self.step_euler(model_output, latents, timestep_idx);

        // Adicionar noise ancestral
        if timestep_idx > 0 {
            let noise_scale = 0.01;
            for val in &mut result {
                *val += (timestep_idx as f32 * 0.1).sin() * noise_scale;
            }
        }

        result
    }

    /// DPM-Solver++ step (simplificado)
    fn step_dpm(&self, model_output: &[f32], latents: &[f32], timestep_idx: usize) -> Vec<f32> {
        // Versão simplificada do DPM-Solver
        // A versão completa usa múltiplas ordens e predições
        self.step_euler(model_output, latents, timestep_idx)
    }

    /// Gera noise aleatório gaussiano
    pub fn generate_noise(size: usize, seed: u64) -> Vec<f32> {
        let mut rng_state = seed;

        (0..size)
            .map(|_| {
                // LCG simples para geração de números pseudo-aleatórios
                rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
                let uniform = (rng_state as f32) / (u64::MAX as f32);

                // Box-Muller para gaussiana
                rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
                let uniform2 = (rng_state as f32) / (u64::MAX as f32);

                let r = (-2.0 * uniform.ln()).sqrt();
                let theta = 2.0 * PI * uniform2;

                r * theta.cos()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = Scheduler::new(SchedulerType::Euler, 50);
        assert_eq!(scheduler.timesteps.len(), 50);
        assert_eq!(scheduler.alphas.len(), 50);
    }

    #[test]
    fn test_noise_generation() {
        let noise = Scheduler::generate_noise(1000, 42);

        // Verificar propriedades gaussianas
        let mean: f32 = noise.iter().sum::<f32>() / noise.len() as f32;
        let variance: f32 =
            noise.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / noise.len() as f32;

        assert!(mean.abs() < 0.1); // Média ~0
        assert!((variance - 1.0).abs() < 0.2); // Variância ~1
    }
}
