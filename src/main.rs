use eframe::egui;

fn main() /* -> eframe::Result */ {
	println!("woooopaint will now be real.");
	eframe::run_native("woooopaint",
		eframe::NativeOptions::default(),
		Box::new(|cc| Ok(Box::new(App::new(cc))))
	);
}

#[derive(Default)]
struct App {}

impl App {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		Self::default()
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.label("Hello Label");
			ui.heading("Hello Heading");
		});
	}
}