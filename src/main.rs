use eframe::egui;
use rand::prelude::*;

const NOISE_SCALE: usize = 64;

struct NoiseTexture {
    data: [u8; NOISE_SCALE * NOISE_SCALE * 3],
}

impl NoiseTexture {
    fn new() -> Self {
        Self {
            data: [0; NOISE_SCALE * NOISE_SCALE * 3],
        }
    }

    fn scramble(&mut self, mode: &ColorMode) {
        for pixel in &mut self.data.chunks_mut(3) {
            match mode {
                ColorMode::Gray => pixel.fill(rand::thread_rng().gen()),
                ColorMode::RG => {
                    pixel[0] = rand::thread_rng().gen();
                    pixel[1] = rand::thread_rng().gen();
                    pixel[2] = 0;
                }
                ColorMode::RGB => pixel.fill_with(|| rand::thread_rng().gen()),
            }
        }
    }

    fn as_color_image(&self) -> egui::ColorImage {
        egui::ColorImage::from_rgb([NOISE_SCALE, NOISE_SCALE], &self.data)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ColorMode {
    Gray,
    RG,
    RGB,
}

struct MyApp {
    texture_handle: Option<egui::TextureHandle>,
    image: NoiseTexture,
    color_mode: ColorMode,
}

impl MyApp {
    fn new() -> Self {
        Self {
            texture_handle: None,
            image: NoiseTexture::new(),
            color_mode: ColorMode::Gray,
        }
    }
}

impl eframe::App for MyApp {
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
                .selected_text(format!("{:?}", self.color_mode))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.color_mode, ColorMode::Gray, "Gray");
                    ui.selectable_value(&mut self.color_mode, ColorMode::RG, "RG");
                    ui.selectable_value(&mut self.color_mode, ColorMode::RGB, "RGB");
                });
            if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture_handle {
                ui.add(egui::Image::new(texture).fit_to_fraction(egui::vec2(1.0, 1.0)));
            }
        });

        // Request a repaint to keep the animation going
        ctx.request_repaint();
    }
}

fn main() {
    let mut options = eframe::NativeOptions::default();
    let viewport = &mut options.viewport;
    viewport.inner_size = Some(egui::Vec2::new(900.0, 600.0));
    viewport.resizable = Some(false);
    let _ = eframe::run_native(
        "Missingno",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    );
}
