mod noise;

use eframe::egui;
use noise::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct Missingno {
    texture_handle: Option<egui::TextureHandle>,
    image: Arc<NoiseTexture>,
    stop: Arc<AtomicBool>,
    color_mode: ColorMode,
}

impl Missingno {
    pub fn new() -> Self {
        Self {
            texture_handle: None,
            image: Arc::new(NoiseTexture::new()),
            stop: Arc::new(AtomicBool::new(false)),
            color_mode: ColorMode::Gray,
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

impl eframe::App for Missingno {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Options").show(ctx, |ui| {
            ui.label("Options");
            if ui.button("Scramble").clicked() {
                self.scramble();
            }
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
            if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
