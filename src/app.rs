mod noise;

use eframe::egui;
use noise::*;
use std::sync::Arc;

pub struct Missingno {
    texture_handle: Option<egui::TextureHandle>,
    image: Arc<NoiseTexture>,
    color_mode: ColorMode,
}

impl Missingno {
    pub fn new() -> Self {
        Self {
            texture_handle: None,
            image: Arc::new(NoiseTexture::new()),
            color_mode: ColorMode::Gray,
        }
    }
}

impl eframe::App for Missingno {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load the image into a texture if not already done
        if self.texture_handle.is_none() {
            self.image.scramble(&self.color_mode);
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
        } else {
            // Update the existing texture with the new image
            // TODO: do not update the image if nothing has changed
            self.texture_handle
                .as_mut()
                .unwrap()
                .set(self.image.as_color_image(), {
                    egui::TextureOptions {
                        magnification: egui::TextureFilter::Nearest,
                        minification: egui::TextureFilter::Nearest,
                        ..egui::TextureOptions::default()
                    }
                });
        }

        egui::SidePanel::right("Options").show(ctx, |ui| {
            ui.label("Options");
            if ui.button("Scramble").clicked() {
                self.image.scramble(&self.color_mode)
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
                        self.image.scramble(&self.color_mode);
                    }
                });
            if ui.button("Refine").clicked() {
                let img = Arc::clone(&self.image);
                let mode = self.color_mode.clone();

                std::thread::spawn(move || {
                    println!("Starting refinement process...");
                    img.refine(&mode);
                });
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

        ctx.request_repaint();
    }
}
