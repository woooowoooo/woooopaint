use eframe::egui;

fn main() -> eframe::Result {
	println!("woooopaint will now be real.");
	return eframe::run_native("woooopaint",
		eframe::NativeOptions {
			viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
			..Default::default()
		},
		Box::new(|cc| {
			egui_extras::install_image_loaders(&cc.egui_ctx);
			Ok(Box::<App>::default())
		})
	);
}

struct App {
	image_name: String,
	color: egui::Color32
}

impl Default for App {
	fn default() -> Self {
		Self {
			image_name: "Default Image".to_owned(),
			color: egui::Color32::from_rgb(255, 255, 255)
		}
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading(format!("Image: {}", self.image_name));
			ui.horizontal(|ui| {
				let label = ui.label("Change image name:");
				ui.text_edit_singleline(&mut self.image_name).labelled_by(label.id);
			});
			if ui.button("Change Color").clicked() {
				self.color = egui::Color32::from_rgb(rand::random(), rand::random(), rand::random());
			}
			// Color picker
			ui.color_edit_button_srgba(&mut self.color);
			ui.image(egui::include_image!("../crimsoncode-sally.png"));
		});
	}
}