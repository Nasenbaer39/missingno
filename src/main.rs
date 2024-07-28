use eframe::egui::{self, Color32, ColorImage};
use rand::prelude::*;

struct MyApp {
    texture_handle: Option<egui::TextureHandle>,
    image: ColorImage,
}

impl MyApp {
    fn new() -> Self {
        Self {
            texture_handle: None,
            image: ColorImage::new([256, 256], Color32::from_gray(255)),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load the image into a texture if not already done
        if self.texture_handle.is_none() {
            for pixel in &mut self.image.pixels {
                *pixel = Color32::from_gray(rand::thread_rng().gen::<u8>());
            }
            self.texture_handle =
                Some(ctx.load_texture("color_texture", self.image.clone(), egui::TextureOptions::default()));
        } else {
            // Update the existing texture with the new image
            self.texture_handle
                .as_mut()
                .unwrap()
                .set(self.image.clone(), egui::TextureOptions::default());
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
