use eframe::egui;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use std::sync::RwLock;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ColorMode {
    Gray,
    Rg,
    Rgb,
}

impl ColorMode {
    pub fn dimension(&self) -> i32 {
        match self {
            ColorMode::Gray => 1,
            ColorMode::Rg => 2,
            ColorMode::Rgb => 3,
        }
    }
}

const NOISE_SCALE: usize = 64;

const INITIAL_TEMPERATURE: f64 = 1.0;
const ITERATIONS: usize = NOISE_SCALE * 2_usize.pow(NOISE_SCALE as u32 / 32);
const ALPHA: f64 = 0.9;
const SIGMA: f64 = 2.42;

pub struct NoiseTexture {
    data: RwLock<[u8; NOISE_SCALE * NOISE_SCALE * 3]>,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            data: RwLock::new([0; NOISE_SCALE * NOISE_SCALE * 3]),
        }
    }

    pub fn scramble(&self, mode: &ColorMode) {
        let mut data = self.data.write().unwrap();
        for pixel in &mut data.chunks_mut(3) {
            match mode {
                ColorMode::Gray => pixel.fill(thread_rng().gen()),
                ColorMode::Rg => {
                    pixel.copy_from_slice(&[thread_rng().gen(), thread_rng().gen(), 0])
                }
                ColorMode::Rgb => thread_rng().fill_bytes(pixel),
            }
        }
    }

    pub fn as_color_image(&self) -> egui::ColorImage {
        egui::ColorImage::from_rgb([NOISE_SCALE, NOISE_SCALE], &*self.data.read().unwrap())
    }

    pub fn refine(&self, mode: &ColorMode) {
        let mut data = self.data.read().unwrap().clone();
        let mut t = INITIAL_TEMPERATURE;
        let mut distribution;

        while t > 1e-64 {
            distribution = Normal::new(t, 0.15).unwrap();

            for _ in 0..ITERATIONS {
                let first = thread_rng().gen_range(0..NOISE_SCALE * NOISE_SCALE);

                let sample = distribution.sample(&mut thread_rng());
                let second = Self::pos_in_range(first, Self::sample_dist(sample));

                let current_energy = Self::pair_energy(first, second, &data, mode);

                Self::swap(&mut data, first, second);

                let new_energy = Self::pair_energy(first, second, &data, mode);

                if Self::accept(current_energy, new_energy, t) >= thread_rng().gen() {
                    Self::swap(&mut *self.data.write().unwrap(), first, second);
                } else {
                    Self::swap(&mut data, first, second);
                }
            }
            t *= ALPHA;
            println!("Current temperature: {t}");
        }
    }

    fn accept(e_old: f64, e_new: f64, t: f64) -> f64 {
        if e_new < e_old {
            1.0
        } else {
            ((e_old - e_new) / t).exp()
        }
    }

    fn swap(data: &mut [u8], first: usize, second: usize) {
        for i in 0..3 {
            data.swap(first * 3 + i, second * 3 + i);
        }
    }

    fn energy(first: usize, second: usize, data: &[u8], mode: &ColorMode) -> f64 {
        let mut energy = (-Self::pixel_distance_sqr(first, second) / SIGMA).exp();
        energy /= (1.0
            + Self::color_distance(
                &data[first * 3..first * 3 + 3],
                &data[second * 3..second * 3 + 3],
                mode,
            ))
        .powi(mode.dimension())
        .sqrt();
        energy
    }

    fn pair_energy(first: usize, second: usize, data: &[u8], mode: &ColorMode) -> f64 {
        Self::pixel_energy(first, data, mode) + Self::pixel_energy(second, data, mode)
    }

    fn pixel_energy(pixel: usize, data: &[u8], mode: &ColorMode) -> f64 {
        let mut pixel_energy = 0.0;
        for j in 0..NOISE_SCALE * NOISE_SCALE {
            if j != pixel {
                pixel_energy += Self::energy(pixel, j, data, mode);
            }
        }
        pixel_energy
    }

    #[allow(dead_code)]
    fn total_energy(data: &[u8], mode: &ColorMode) -> f64 {
        let mut overall_energy = 0.0;
        for i in 0..NOISE_SCALE * NOISE_SCALE - 1 {
            for j in i + 1..NOISE_SCALE * NOISE_SCALE {
                overall_energy += Self::energy(i, j, data, mode);
            }
        }

        overall_energy
    }

    fn pixel_distance_sqr(first: usize, second: usize) -> f64 {
        let x1 = first % NOISE_SCALE;
        let x2 = second % NOISE_SCALE;

        let y1 = first / NOISE_SCALE;
        let y2 = second / NOISE_SCALE;

        let mut x = x1.abs_diff(x2);
        let mut y = y1.abs_diff(y2);

        if x > NOISE_SCALE / 2 {
            x = NOISE_SCALE - x;
        }

        if y > NOISE_SCALE / 2 {
            y = NOISE_SCALE - y;
        }

        return (x * x + y * y) as f64;
    }

    fn color_distance(first: &[u8], second: &[u8], mode: &ColorMode) -> f64 {
        match mode {
            ColorMode::Gray => (first[0].abs_diff(second[0])) as f64,
            ColorMode::Rg => ((first[0].abs_diff(second[0]) as f64).powi(2)
                + (first[1].abs_diff(second[1]) as f64).powi(2))
            .sqrt(),
            ColorMode::Rgb => ((first[0].abs_diff(second[0]) as f64).powi(2)
                + (first[1].abs_diff(second[1]) as f64).powi(2)
                + (first[2].abs_diff(second[2]) as f64).powi(2))
            .sqrt(),
        }
    }

    fn pos_in_range(first: usize, dist: usize) -> usize {
        let mut rand = thread_rng().gen_range(0..dist * dist - 1);

        if rand >= (dist * dist - 1) / 2 {
            rand += 1;
        }

        let x = (NOISE_SCALE + rand % dist - dist / 2 + first % NOISE_SCALE) % NOISE_SCALE;
        let y = (NOISE_SCALE + rand / dist - dist / 2 + first / NOISE_SCALE) % NOISE_SCALE;

        y * NOISE_SCALE + x
    }

    fn sample_dist(sample: f64) -> usize {
        let mut sample = sample.clamp(-1.0, 2.0).abs();
        if sample > 1.0 {
            sample = 2.0 - sample;
        }
        2 * (sample * NOISE_SCALE as f64).ceil() as usize + 1
    }
}
