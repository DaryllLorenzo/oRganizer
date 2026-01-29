use eframe::egui;
use eframe::egui::RichText;
use rfd::FileDialog;
use crate::core::FileOrganizerCore; 

pub struct ORganizer {
    pub ruta_seleccionada: String,
    pub archivos_listados: Vec<String>, 
}

impl ORganizer {
    pub fn new() -> Self {
        Self {
            ruta_seleccionada: String::new(),
            archivos_listados: Vec::new(),
        }
    }
}

impl eframe::App for ORganizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Título grande
            ui.add(egui::Label::new(
                RichText::new("Selector de Ruta").size(28.0).strong()
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
                        self.archivos_listados.clear(); // Limpiar lista anterior
                    }
                }
            });

            // Mostrar ruta seleccionada
            ui.text_edit_singleline(&mut self.ruta_seleccionada).sense.interactive();

            ui.add_space(15.0);

            // Botón Aceptar 
            if ui.button(" Aceptar y Listar Archivos").clicked() {
                if !self.ruta_seleccionada.is_empty() {
                    println!("✔ Ruta aceptada: {}", self.ruta_seleccionada);
                    
                    // Usar el core para listar archivos
                    match FileOrganizerCore::list_files_in_path(&self.ruta_seleccionada) {
                        Ok(archivos) => {
                            println!("\n Contenido de '{}':", self.ruta_seleccionada);
                            println!("{}", "=".repeat(50));
                            
                            for (i, archivo) in archivos.iter().enumerate() {
                                println!("{}. {}", i + 1, archivo);
                            }
                            
                            println!("{}", "=".repeat(50));
                            println!("Total: {} elementos", archivos.len());
                            
                            // Guardar para mostrar en UI si quieres
                            self.archivos_listados = archivos;
                        }
                        Err(e) => {
                            println!(" Error: {}", e);
                        }
                    }
                } else {
                    println!(" Por favor, selecciona una ruta primero.");
                }
            }

            // Mostrar conteo de archivos encontrados
            if !self.archivos_listados.is_empty() {
                ui.add_space(10.0);
                ui.label(format!(" Archivos encontrados: {}", self.archivos_listados.len()));
            }
        });
    }
}