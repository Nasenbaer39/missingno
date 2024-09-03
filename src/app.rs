mod noise;

use eframe::egui;
use noise::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[derive(Debug, PartialEq)]
enum NoiseType {
    White,
    Blue,
}

pub struct Missingno {
    texture_handle: Option<egui::TextureHandle>,
    image: Arc<NoiseTexture>,
    stop: Arc<AtomicBool>,
    color_mode: ColorMode,
    noise_type: NoiseType,
    size: usize,
}

impl Missingno {
    pub fn new() -> Self {
        Self {
            texture_handle: None,
            image: Arc::new(NoiseTexture::new(64)),
            stop: Arc::new(AtomicBool::new(false)),
            color_mode: ColorMode::Gray,
            noise_type: NoiseType::White,
            size: 64,
        }
    }
}

impl Missingno {
    fn scramble(&self) {
        self.stop.store(true, Ordering::Relaxed);
        self.image.scramble(&self.color_mode);
    }

    fn update_texture(&mut self, ctx: &egui::Context) {
        if let Some(texture) = &mut self.texture_handle {
            texture.set(self.image.as_color_image(), {
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..egui::TextureOptions::default()
                }
            })
        } else {
            // Load the image data into a texture
            self.texture_handle =
                Some(
                    ctx.load_texture("color_texture", self.image.as_color_image(), {
                        egui::TextureOptions {
                            magnification: egui::TextureFilter::Nearest,
                            minification: egui::TextureFilter::Nearest,
                            ..egui::TextureOptions::default()
                        }
                    }),
                );
            self.scramble();
        }
    }
}

impl Missingno {
    fn white_noise(&self, _ui: &mut egui::Ui) {}

    fn blue_noise(&self, ui: &mut egui::Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Refine").clicked() {
                self.stop.store(false, Ordering::Relaxed);

                let img = Arc::clone(&self.image);
                let stop = Arc::clone(&self.stop);
                let mode = self.color_mode.clone();

                std::thread::spawn(move || {
                    println!("Starting refinement process...");
                    img.refine(&mode, stop);
                });
            }
            if ui.button("Stop").clicked() {
                self.stop.store(true, Ordering::Relaxed);
            }
        });
    }
}

impl eframe::App for Missingno {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Options").show(ctx, |ui| {
            ui.heading("Options");
            ui.separator();
            egui::ComboBox::from_label("Noise Type")
                .selected_text(format!("{:?}", self.noise_type).to_uppercase())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.noise_type, NoiseType::White, "White");
                    ui.selectable_value(&mut self.noise_type, NoiseType::Blue, "Blue");
                });
            ui.add_space(8.0);
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.color_mode).to_uppercase())
                .show_ui(ui, |ui| {
                    // check if an option was selected and scramble if that is the case
                    if ui
                        .selectable_value(&mut self.color_mode, ColorMode::Gray, "Gray")
                        .clicked()
                        || ui
                            .selectable_value(&mut self.color_mode, ColorMode::Rg, "RG")
                            .clicked()
                        || ui
                            .selectable_value(&mut self.color_mode, ColorMode::Rgb, "RGB")
                            .clicked()
                    {
                        self.scramble();
                    }
                });
            let resized = ui
                .add(
                    egui::DragValue::new(&mut self.size)
                        .range(2..=512)
                        .suffix("px")
                        .update_while_editing(false),
                )
                .changed();
            if ui.button("Scramble").clicked() {
                self.scramble();
            }

            if resized {
                self.stop.store(true, Ordering::Relaxed);
                self.image.resize(self.size);
                Self::scramble(&self);
            }

            match self.noise_type {
                NoiseType::White => self.white_noise(ui),
                NoiseType::Blue => self.blue_noise(ui),
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture_handle {
                ui.add(egui::Image::new(texture).fit_to_fraction(egui::vec2(1.0, 1.0)));
            }
        });

        self.update_texture(ctx);

        ctx.request_repaint();
    }
}
