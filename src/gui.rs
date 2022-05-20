use eframe::egui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Call this once from the HTML.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
	eframe::start_web(canvas_id, Box::new(|cc| Box::new(MyApp::new(cc))))
}

pub fn display(files: Vec<String> )
{
	let native_options = eframe::NativeOptions::default();
	eframe::run_native("Xniffer", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
		// Restore app state using cc.storage (requires the "persistence" feature).
		// Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
		// for e.g. egui::PaintCallback.
		Self::default()
	}
}

impl eframe::App for MyEguiApp {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		// File selection panel
		egui::SidePanel::new(egui::panel::Side::Left, "file_selection")
		.show(ctx, |ui| {
			ui.heading("Files");
			ui.separator();
			ui.button("Open");
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Hello World!");
		});
	}
}
