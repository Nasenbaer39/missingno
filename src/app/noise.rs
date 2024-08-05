use rand::prelude::*;
use eframe::egui;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorMode {
    Gray,
    Rg,
    Rgb,
}

// TODO: separate code into modules
const NOISE_SCALE: usize = 64;

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
}


