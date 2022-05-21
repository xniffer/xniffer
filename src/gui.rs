use eframe::egui;
use native_dialog::FileDialog;
use std::path::PathBuf;

#[derive(Default, Clone)]
struct File {
	file_path: PathBuf,
	name_stim: String,
	data: Option<std::vec::Vec<crate::Data>>,
}

// Entry point from main.rs
pub fn display(files: Vec<String>) {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native(
		"Xniffer",
		native_options,
		Box::new(|cc| Box::new(Xniffer::new(cc, files))),
	);
}

#[derive(Default)]
struct Xniffer {
	files: Vec<File>,
	index: Option<u16>,
}

impl Xniffer {
	fn new(_cc: &eframe::CreationContext<'_>, files_in: Vec<String>) -> Self {
		Self {
			files: files_in
				.iter()
				.map(|file| File {
					file_path: PathBuf::from(file),
					name_stim: file.to_string(),
					data: crate::parse(file),
				})
				.collect(),
			index: None,
		}
	}
}

impl eframe::App for Xniffer {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// File selection panel
		egui::SidePanel::new(egui::panel::Side::Left, "file_selection")
			.min_width(20f32)
			.show(ctx, |ui| {
				ui.heading("Files");
				ui.separator();
				if ui.button("Open").clicked() {
					let paths = FileDialog::new()
						.add_filter("PNG Image", &["png"])
						.add_filter("JPEG Image", &["jpg", "jpeg"])
						.add_filter("EXV Image", &["exv"])
						.add_filter("CR2 Image", &["cr2"])
						.add_filter("TIFF Image", &["tiff", "tif"])
						.add_filter("DNG Image", &["dng"])
						.add_filter("PSD Image", &["psd"])
						.show_open_multiple_file()
						.unwrap();

					for path in paths {
						let path_str = path.clone().as_os_str().to_string_lossy().to_string();

						// Check for duplicates
						if self
							.files
							.clone()
							.into_iter()
							.find_map(|f| {
								if path == f.file_path {
									Some(true)
								} else {
									None
								}
							})
							.is_some()
						{
							continue;
						}

						self.files.push(File {
							file_path: path.clone(),
							name_stim: path_str.clone(),
							data: crate::parse(&path_str),
						})
					}
				}
			});
		egui::CentralPanel::default().show(ctx, |ui| ui.code(format!("{}", self.files.len())));
	}
}
