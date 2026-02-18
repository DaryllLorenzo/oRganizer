use eframe::egui;
use eframe::egui::RichText;
use rfd::FileDialog;
use crate::core::{FileOrganizerCore, OrganizerResult, FileOperationMode};

pub struct ORganizer {
    pub ruta_seleccionada: String,
    pub archivos_listados: Vec<String>,
    pub resultado_organizacion: Option<OrganizerResult>,
    pub mostrar_resumen: bool,
    pub exclude_folders: String,
    pub operation_mode: FileOperationMode,
}

impl ORganizer {
    pub fn new() -> Self {
        Self {
            ruta_seleccionada: String::new(),
            archivos_listados: Vec::new(),
            resultado_organizacion: None,
            mostrar_resumen: false,
            exclude_folders: String::new(),
            operation_mode: FileOperationMode::Cut,
        }
    }
}

impl eframe::App for ORganizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Título grande
            ui.add(egui::Label::new(
                RichText::new("Organizador de Archivos").size(28.0).strong()
            ));

            // Descripción
            ui.label("Selecciona una carpeta para organizar archivos por extension.");

            ui.add_space(15.0);

            // Selector de ruta
            ui.horizontal(|ui| {
                ui.label("Ruta:");
                if ui.button("Seleccionar Carpeta...").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.ruta_seleccionada = path.to_string_lossy().to_string();
                        self.archivos_listados.clear(); // Limpiar lista anterior
                        self.resultado_organizacion = None;
                        self.mostrar_resumen = false;
                    }
                }
            });

            // Mostrar ruta seleccionada
            ui.text_edit_singleline(&mut self.ruta_seleccionada).sense.interactive();

            ui.add_space(15.0);

            // Campo para excluir archivos/carpetas
            ui.horizontal(|ui| {
                ui.label("Archivos/Carpetas a excluir (separados por coma):");
            });
            ui.text_edit_singleline(&mut self.exclude_folders);

            ui.add_space(10.0);

            // Operation mode selection
            ui.label(RichText::new("Modo de operacion:").strong());
            ui.horizontal(|ui| {
                let cut_selected = ui.selectable_value(&mut self.operation_mode, FileOperationMode::Cut, "✂ Cortar (Mover)").changed();
                if cut_selected {
                    println!("Modo Cortar seleccionado - Los archivos se moveran");
                }
                
                let copy_selected = ui.selectable_value(&mut self.operation_mode, FileOperationMode::Copy, "📋 Copiar").changed();
                if copy_selected {
                    println!("Modo Copiar seleccionado - Los archivos se copiaran");
                }
            });

            ui.add_space(10.0);

            // Botones de acción
            ui.horizontal(|ui| {
                if ui.button("Listar Archivos").clicked() {
                    self.listar_archivos();
                }

                if ui.button("Limpiar").clicked() {
                    self.ruta_seleccionada.clear();
                    self.archivos_listados.clear();
                    self.resultado_organizacion = None;
                    self.mostrar_resumen = false;
                    self.exclude_folders.clear();
                }
            });

            ui.add_space(5.0);
            
            // Organize buttons based on mode
            ui.horizontal(|ui| {
                let enabled = !self.ruta_seleccionada.is_empty();
                ui.add_enabled_ui(enabled, |ui| {
                    if ui.button("✂ Cortar y Organizar").clicked() {
                        self.operation_mode = FileOperationMode::Cut;
                        self.organizar_archivos();
                    }
                    
                    if ui.button("📋 Copiar y Organizar").clicked() {
                        self.operation_mode = FileOperationMode::Copy;
                        self.organizar_archivos();
                    }
                });
            });

            // Mostrar resultados según el estado
            if self.mostrar_resumen {
                self.mostrar_resumen_organizacion(ui);
            } else {
                self.mostrar_lista_archivos(ui);
            }
        });
    }
}

impl ORganizer {
    fn listar_archivos(&mut self) {
        if !self.ruta_seleccionada.is_empty() {
            println!("Listando archivos en: {}", self.ruta_seleccionada);
            
            match FileOrganizerCore::list_files_in_path(&self.ruta_seleccionada) {
                Ok(archivos) => {
                    self.archivos_listados = archivos;
                    self.mostrar_resumen = false;
                }
                Err(e) => {
                    self.archivos_listados = vec![format!("Error: {}", e)];
                }
            }
        } else {
            self.archivos_listados = vec!["Por favor, selecciona una ruta primero.".to_string()];
        }
    }
    
    fn organizar_archivos(&mut self) {
        if !self.ruta_seleccionada.is_empty() {
            println!("Organizando archivos en: {} (modo: {:?})", self.ruta_seleccionada, self.operation_mode);

            // Parse the excluded folders from the input field
            let excluded_folders: Vec<String> = self.exclude_folders
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            // Run the organization with exclusions and the selected operation mode
            let result = FileOrganizerCore::organize_by_extension_with_progress_and_exclusions(
                &self.ruta_seleccionada,
                self.operation_mode,
                &excluded_folders,
                |_current, _total| {
                    // No progress updates needed - blocking operation
                }
            );

            match result {
                Ok(resultado) => {
                    println!("Organizacion completada exitosamente");
                    self.resultado_organizacion = Some(resultado);
                    self.mostrar_resumen = true;

                    // Actualizar lista de archivos después de organizar
                    self.listar_archivos();
                }
                Err(e) => {
                    self.archivos_listados = vec![format!("Error al organizar: {}", e)];
                    self.mostrar_resumen = false;
                }
            }
        } else {
            self.archivos_listados = vec!["Por favor, selecciona una ruta primero.".to_string()];
            self.mostrar_resumen = false;
        }
    }
    
    fn mostrar_lista_archivos(&mut self, ui: &mut egui::Ui) {
        if !self.archivos_listados.is_empty() {
            ui.add_space(10.0);
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Archivos encontrados:").strong());
                ui.label(format!("({} elementos)", self.archivos_listados.len()));
            });
            
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    for archivo in &self.archivos_listados {
                        ui.label(archivo);
                    }
                });
            
            // Mostrar información sobre la carpeta Organizer si existe
            let organizer_path = format!("{}/Organizer", self.ruta_seleccionada);
            if std::path::Path::new(&organizer_path).exists() {
                ui.add_space(10.0);
                ui.label(RichText::new("Nota: Ya existe una carpeta 'Organizer' en esta ubicacion.").color(egui::Color32::YELLOW));
            }
        }
    }
    
    fn mostrar_resumen_organizacion(&mut self, ui: &mut egui::Ui) {
        if let Some(resultado) = &self.resultado_organizacion {
            ui.add_space(10.0);
            ui.separator();

            ui.heading("Resumen de Organizacion");

            // Show operation mode in the summary
            let operation_text = match self.operation_mode {
                FileOperationMode::Cut => "Archivos movidos:",
                FileOperationMode::Copy => "Archivos copiados:",
            };

            // Mostrar estadísticas principales
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new(operation_text).strong());
                ui.label(format!("{}", resultado.total_moved));
            });
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Carpetas creadas:").strong());
                ui.label(format!("{}", resultado.folders_created));
            });
            
            // Mostrar detalles por extensión
            ui.add_space(10.0);
            ui.label(RichText::new("Detalles por extension:").strong());
            
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    for (extension, files) in &resultado.extension_map {
                        ui.collapsing(format!("{} ({} archivos)", extension, files.len()), |ui| {
                            for file in files {
                                ui.label(format!("  - {}", file));
                            }
                        });
                    }
                });
            
            // Mostrar resumen completo
            ui.add_space(10.0);
            ui.label(RichText::new("Resumen completo:").strong());
            
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    ui.label(&resultado.summary);
                });
            
            // Mostrar errores si los hay
            if let Some(errors) = &resultado.errors {
                if !errors.is_empty() {
                    ui.add_space(10.0);
                    ui.label(RichText::new("Errores encontrados:").color(egui::Color32::RED));
                    
                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            for error in errors {
                                ui.label(format!("- {}", error));
                            }
                        });
                }
            }
            
            // Botón para volver a la lista
            ui.add_space(10.0);
            if ui.button("Volver a la lista de archivos").clicked() {
                self.mostrar_resumen = false;
            }
        }
    }
}