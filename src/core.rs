use std::fs;
use std::path::Path;

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
                            
                            let file_type = if path.is_dir() { "Carpeta" } else { "Archivo" };
                            let file_name = path.file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| "sin_nombre".to_string());
                            
                            let size_info = if path.is_file() {
                                metadata.map(|m| format!(" ({} bytes)", m.len()))
                                    .unwrap_or_default()
                            } else {
                                "".to_string()
                            };
                            
                            files.push(format!("{}{}{}", file_type, file_name, size_info));
                        }
                        Err(e) => {
                            files.push(format!(" Error leyendo entrada: {}", e));
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