use eframe::egui;
use eframe::egui::RichText;
use rfd::FileDialog;

pub struct ORganizer {
    pub ruta_seleccionada: String,
}

impl ORganizer {
    pub fn new() -> Self {
        Self {
            ruta_seleccionada: String::new(),
        }
    }
}

impl eframe::App for ORganizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Título grande
            ui.add(egui::Label::new(
                RichText::new("📁 Selector de Ruta").size(28.0).strong()
            ));

            // Descripción
            ui.label("Por favor, selecciona una carpeta o archivo y luego haz clic en Aceptar.");

            ui.add_space(15.0);

            // Selector de ruta
            ui.horizontal(|ui| {
                ui.label("Ruta:");
                if ui.button("Seleccionar Carpeta...").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.ruta_seleccionada = path.to_string_lossy().to_string();
                    }
                }
            });

            // Mostrar ruta seleccionada (solo lectura)
            ui.text_edit_singleline(&mut self.ruta_seleccionada).sense.interactive();

            ui.add_space(15.0);

            // Botón Aceptar
            if ui.button("✅ Aceptar").clicked() {
                if !self.ruta_seleccionada.is_empty() {
                    println!("✔ Ruta aceptada: {}", self.ruta_seleccionada);
                    // Aquí tu lógica de procesamiento
                } else {
                    println!("⚠ Por favor, selecciona una ruta primero.");
                }
            }
        });
    }
}