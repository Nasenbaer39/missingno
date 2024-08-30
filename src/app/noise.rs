use eframe::egui;
use rand::prelude::*;
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

const NOISE_SCALE: usize = 32;

const INITIAL_TEMPERATURE: f64 = 1.0;
const ITERATIONS: usize = 128;
const ALPHA: f64 = 0.9;
const SIGMA: f64 = 2.1;

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
        let mut current_energy = Self::energy(&data, mode);
        let mut t = INITIAL_TEMPERATURE;

        loop {
            for _ in 0..ITERATIONS {
                let first = thread_rng().gen_range(0..NOISE_SCALE * NOISE_SCALE);
                let mut second = thread_rng().gen_range(0..NOISE_SCALE * NOISE_SCALE - 1);

                if second >= first {
                    second += 1;
                }

                Self::swap(&mut data, first, second);

                let new_energy = Self::energy(&data, mode);

                if Self::accept(current_energy, new_energy, t) >= thread_rng().gen() {
                    Self::swap(&mut *self.data.write().unwrap(), first, second);
                    current_energy = new_energy;
                } else {
                    Self::swap(&mut data, first, second);
                }
            }
            t *= ALPHA;
            println!("Current energy:      {current_energy}");
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

    fn energy(data: &[u8], mode: &ColorMode) -> f64 {
        let mut overall_energy = 0.0;
        for i in 0..NOISE_SCALE * NOISE_SCALE - 1 {
            for j in i + 1..NOISE_SCALE * NOISE_SCALE {
                let mut energy = (-Self::pixel_distance_sqr(i, j) / SIGMA.powi(2)).exp();
                energy /= (1.0
                    + Self::sample_distance(
                        &data[i * 3..i * 3 + 3],
                        &data[j * 3..j * 3 + 3],
                        mode,
                    ))
                .powi(mode.dimension())
                .sqrt();
                overall_energy += energy;
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

    fn sample_distance(first: &[u8], second: &[u8], mode: &ColorMode) -> f64 {
        match mode {
            ColorMode::Gray => (first[0].abs_diff(second[0])) as f64,
            ColorMode::Rg => (((first[0].abs_diff(second[0]) as u32).pow(2)
                + (first[1].abs_diff(second[1]) as u32).pow(2))
                as f64)
                .sqrt(),
            ColorMode::Rgb => (((first[0].abs_diff(second[0]) as u32).pow(2)
                + (first[1].abs_diff(second[1]) as u32).pow(2)
                + (first[2].abs_diff(second[2]) as u32).pow(2))
                as f64)
                .sqrt(),
        }
    }
}
