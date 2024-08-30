use eframe::egui;
use rand::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorMode {
    Gray,
    Rg,
    Rgb,
}

const NOISE_SCALE: usize = 16;
const SIGMA: f64 = 2.1_f64;

pub struct NoiseTexture {
    data: [u8; NOISE_SCALE * NOISE_SCALE * 3],
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            data: [0; NOISE_SCALE * NOISE_SCALE * 3],
        }
    }

    pub fn scramble(&mut self, mode: &ColorMode) {
        for pixel in &mut self.data.chunks_mut(3) {
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
        egui::ColorImage::from_rgb([NOISE_SCALE, NOISE_SCALE], &self.data)
    }

    pub fn refine(&mut self) {
        let first = thread_rng().gen_range(0..NOISE_SCALE * NOISE_SCALE);
        let mut second = thread_rng().gen_range(0..NOISE_SCALE * NOISE_SCALE - 1);

        if second >= first {
            second += 1;
        }

        let initial_energy = self.energy();

        self.swap(first, second);

        let new_energy = self.energy();

        if new_energy >= initial_energy {
            self.swap(first, second);
        } else {
            println!("New energy: {new_energy}");
        }
    }

    fn swap(&mut self, first: usize, second: usize) {
        let color1 = self.data[first * 3];
        let color2 = self.data[first * 3 + 1];
        let color3 = self.data[first * 3 + 2];

        self.data[first * 3] = self.data[second * 3];
        self.data[first * 3 + 1] = self.data[second * 3];
        self.data[first * 3 + 2] = self.data[second * 3];

        self.data[second * 3] = color1;
        self.data[second * 3 + 1] = color2;
        self.data[second * 3 + 2] = color3;
    }

    fn energy(&self) -> f64 {
        let mut overall_energy = 0.0;
        for i in 0..NOISE_SCALE * NOISE_SCALE {
            for j in i..NOISE_SCALE * NOISE_SCALE {
                if i == j {
                    continue;
                }

                let mut energy = (-self.distance_sqr(i, j) / SIGMA.powi(2)).exp();
                energy /= (1.0 + f64::from(self.data[i * 3].abs_diff(self.data[j * 3]))).sqrt();
                overall_energy += energy;
            }
        }

        overall_energy
    }

    fn distance_sqr(&self, first: usize, second: usize) -> f64 {
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
}
