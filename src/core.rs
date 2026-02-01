use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub struct FileOrganizerCore;

impl FileOrganizerCore {
    /// Lista todos los archivos en la ruta especificada
    pub fn list_files_in_path(path: &str) -> Result<Vec<String>, String> {
        let path = Path::new(path);
        
        // Verificar si la ruta existe
        if !path.exists() {
            return Err(format!("La ruta '{}' no existe", path.display()));
        }
        
        // Verificar si es un archivo o directorio
        if path.is_file() {
            return Self::list_single_file(path);
        } else if path.is_dir() {
            return Self::list_files_in_directory(path);
        }
        
        Err("Ruta no válida".to_string())
    }

    /// Organiza archivos por extensión en una carpeta "Organizer"
    pub fn organize_by_extension(path: &str) -> Result<OrganizerResult, String> {
        let path = Path::new(path);
        
        // Verificar si la ruta existe
        if !path.exists() {
            return Err(format!("La ruta '{}' no existe", path.display()));
        }
        
        // Verificar si es un directorio
        if !path.is_dir() {
            return Err("Solo se pueden organizar archivos dentro de directorios".to_string());
        }
        
        // Crear carpeta Organizer
        let organizer_path = path.join("Organizer");
        if !organizer_path.exists() {
            if let Err(e) = fs::create_dir(&organizer_path) {
                return Err(format!("Error al crear carpeta Organizer: {}", e));
            }
        }
        
        // Contadores y estadísticas
        let mut moved_files = 0;
        let mut created_folders = 0;
        let mut extension_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut errors = Vec::new();
        
        // Leer todos los archivos en el directorio
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let file_path = entry.path();
                            
                            // Saltar si es un directorio o la carpeta Organizer
                            if file_path.is_dir() || file_path.file_name()
                                .map(|n| n.to_string_lossy() == "Organizer")
                                .unwrap_or(false) {
                                continue;
                            }
                            
                            // Obtener extensión del archivo
                            let extension = if let Some(ext) = file_path.extension() {
                                ext.to_string_lossy().to_lowercase().to_string()
                            } else {
                                "sin_extension".to_string()
                            };
                            
                            // Crear nombre de carpeta para la extensión
                            let folder_name = if extension == "sin_extension" {
                                "Sin_Extension".to_string()
                            } else {
                                extension.to_uppercase()
                            };
                            
                            // Crear carpeta para la extensión si no existe
                            let extension_folder = organizer_path.join(&folder_name);
                            if !extension_folder.exists() {
                                if let Err(e) = fs::create_dir(&extension_folder) {
                                    errors.push(format!("Error al crear carpeta {}: {}", folder_name, e));
                                    continue;
                                }
                                created_folders += 1;
                            }
                            
                            // Mover archivo a la carpeta correspondiente
                            let file_name = file_path.file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| "archivo".to_string());
                            
                            let new_path = extension_folder.join(&file_name);
                            
                            match fs::rename(&file_path, &new_path) {
                                Ok(_) => {
                                    moved_files += 1;
                                    extension_map
                                        .entry(folder_name.clone())
                                        .or_insert_with(Vec::new)
                                        .push(file_name);
                                }
                                Err(e) => {
                                    errors.push(format!("Error al mover {}: {}", file_name, e));
                                }
                            }
                        }
                        Err(e) => {
                            errors.push(format!("Error leyendo entrada: {}", e));
                        }
                    }
                }
                
                // Crear resumen de organización
                let summary = Self::create_organization_summary(&extension_map, moved_files, created_folders);
                
                Ok(OrganizerResult {
                    total_moved: moved_files,
                    folders_created: created_folders,
                    extension_map,
                    summary,
                    errors: if errors.is_empty() { None } else { Some(errors) },
                })
            }
            Err(e) => Err(format!("Error al leer el directorio: {}", e)),
        }
    }

    /// Crea un resumen de la organización
    fn create_organization_summary(
        extension_map: &HashMap<String, Vec<String>>,
        total_moved: usize,
        folders_created: usize,
    ) -> String {
        let mut summary = format!(
            "Organizacion completada\n\n\
             Archivos movidos: {}\n\
             Carpetas creadas: {}\n\n\
             Extensiones organizadas:\n",
            total_moved, folders_created
        );
        
        for (extension, files) in extension_map {
            summary.push_str(&format!("  - {}: {} archivo(s)\n", extension, files.len()));
        }
        
        summary
    }

    /// Maneja cuando la ruta es un archivo individual
    fn list_single_file(path: &Path) -> Result<Vec<String>, String> {
        if let Some(file_name) = path.file_name() {
            let file_info = format!(
                "Archivo {} ({} bytes)",
                file_name.to_string_lossy(),
                fs::metadata(path).map(|m| m.len()).unwrap_or(0)
            );
            Ok(vec![file_info])
        } else {
            Ok(vec!["Archivo sin nombre".to_string()])
        }
    }

    /// Lista archivos en un directorio
    fn list_files_in_directory(path: &Path) -> Result<Vec<String>, String> {
        let mut files = Vec::new();
        
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            let metadata = fs::metadata(&path).ok();
                            
                            let file_type = if path.is_dir() { "[DIR] " } else { "[FILE]" };
                            let file_name = path.file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| "sin_nombre".to_string());
                            
                            let size_info = if path.is_file() {
                                metadata.map(|m| format!(" ({} bytes)", m.len()))
                                    .unwrap_or_default()
                            } else {
                                "".to_string()
                            };
                            
                            files.push(format!("{} {}{}", file_type, file_name, size_info));
                        }
                        Err(e) => {
                            files.push(format!("Error leyendo entrada: {}", e));
                        }
                    }
                }
                
                // Ordenar alfabéticamente
                files.sort();
                Ok(files)
            }
            Err(e) => Err(format!("Error al leer el directorio: {}", e)),
        }
    }
}

/// Resultado de la operación de organización
#[derive(Debug)]
pub struct OrganizerResult {
    pub total_moved: usize,
    pub folders_created: usize,
    pub extension_map: HashMap<String, Vec<String>>,
    pub summary: String,
    pub errors: Option<Vec<String>>,
}