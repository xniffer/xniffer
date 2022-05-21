use eframe::egui;
use native_dialog::FileDialog;
use std::{ops::Index, path::PathBuf};

#[derive(Default, Clone)]
struct File {
	file_path: PathBuf,
	name_stem: String,
	data: Option<std::vec::Vec<crate::Data>>,
}

impl PartialEq for File {
	fn eq(&self, other: &Self) -> bool {
		self.file_path == other.file_path
	}
}

impl File {
	pub fn path_str(&self) -> String {
		self.file_path.to_string_lossy().to_string()
	}
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
	index: Option<usize>,
}

impl Xniffer {
	fn new(_cc: &eframe::CreationContext<'_>, files_in: Vec<String>) -> Self {
		Self {
			files: files_in
				.iter()
				.map(|file| File {
					file_path: PathBuf::from(file),
					name_stem: file.to_string(),
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
				// File open
				ui.heading("Files");
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
							file_path: path,
							name_stem: path_str.clone(),
							data: crate::parse(&path_str),
						})
					}
				}
				ui.separator();

				// File selection
				egui::ScrollArea::vertical().show(ui, |ui| {
					egui::Grid::new("some_unique_id").show(ui, |ui| {
						for file in &self.files {
							if ui.button(&file.name_stem).clicked() {
								self.index = self.files.iter().position(|f| f == file);
							}
							ui.end_row()
						}
					});
				});
			});
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.label(if self.index.is_some() {
				self.files.index(self.index.unwrap()).path_str()
			} else {
				"No file selected!".to_string()
			});
			ui.separator();

			// TODO convert to guard clause patern
			if self.index.is_some() {
				if self.files.index(self.index.unwrap()).data.is_some() {
					let data = self.files.index(self.index.unwrap()).data.as_ref().unwrap();

					// File selection
					egui::ScrollArea::vertical().show(ui, |ui| {
						egui::Grid::new("some_unique_id").show(ui, |ui| {
							for dat in data {
								ui.label(&dat.tag);
								ui.label(&dat.value);
								ui.end_row()
							}
						});
					});
				}
			}
		});
	}
}
