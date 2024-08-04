use eframe::egui;
use rand::prelude::*;

struct NoiseTexture {
    data: [u8; 64 * 64],
}

impl NoiseTexture {
    fn new() -> Self {
        Self { data: [0; 64 * 64] }
    }

    fn scramble(&mut self) -> () {
        for pixel in &mut self.data {
            let rand: u8 = rand::thread_rng().gen::<u8>();
            *pixel = rand;
        }
    }

    fn data(&self) -> &[u8] {
        return &self.data;
    }

    fn as_color_image(&self, scale: usize) -> egui::ColorImage {
        let size = 64 * scale;
        let mut scaled_image: Vec<u8> = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                let index: usize = 64 * (y / scale) + x / scale;
                scaled_image.push(self.data[index]); 
            }
        }
        egui::ColorImage::from_gray([size, size], &scaled_image)
    }
}

struct MyApp {
    texture_handle: Option<egui::TextureHandle>,
    image: NoiseTexture,
}

impl MyApp {
    fn new() -> Self {
        Self {
            texture_handle: None,
            image: NoiseTexture::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load the image into a texture if not already done
        let space = ctx.available_rect().size();
        let space = space.x.min(space.y) as usize / 64;
        if self.texture_handle.is_none() {
            self.image.scramble();
            self.texture_handle = Some(ctx.load_texture(
                "color_texture",
                self.image.as_color_image(space),
                egui::TextureOptions::default(),
            ));
        } else {
            // Update the existing texture with the new image
            self.image.scramble();
            self.texture_handle
                .as_mut()
                .unwrap()
                .set(self.image.as_color_image(space), egui::TextureOptions::default());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture_handle {
                ui.add(egui::Image::new(texture));
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
