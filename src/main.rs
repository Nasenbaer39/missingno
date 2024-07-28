use eframe::egui::{self, Color32, ColorImage};
use std::time::Instant;
use rand::prelude::*;

struct MyApp {
    texture_handle: Option<egui::TextureHandle>,
    start_time: Instant,
}

impl MyApp {
    fn new() -> Self {
        Self {
            texture_handle: None,
            start_time: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let gray_value = ((1.0 - (elapsed / 5.0)).max(0.0) * 255.0) as u8; // Change over 5 seconds

        // Create an image buffer with the current color
        let mut image: ColorImage = ColorImage::new([256, 256], Color32::from_gray(gray_value));

        for pixel in &mut image.pixels {
            *pixel = Color32::from_gray(rand::thread_rng().gen::<u8>());
        }

        // Load the image into a texture if not already done
        if self.texture_handle.is_none() {
            self.texture_handle =
                Some(ctx.load_texture("color_texture", image, egui::TextureOptions::default()));
        } else {
            // Update the existing texture with the new image
            self.texture_handle
                .as_mut()
                .unwrap()
                .set(image, egui::TextureOptions::default());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture_handle {
                ui.add(egui::Image::new(texture).fit_to_fraction(egui::Vec2::new(1.0, 1.0)));
            }
        });

        // Request a repaint to keep the animation going
        ctx.request_repaint();
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Image Color Change Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    );
}
