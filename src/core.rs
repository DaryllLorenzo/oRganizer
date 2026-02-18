use std::fs;
use std::path::Path;
use std::collections::HashMap;

// Define a callback type for progress updates
pub type ProgressCallback = dyn Fn(usize, usize) + Send; // (current, total)

/// Operation mode: either move (cut) or copy files
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileOperationMode {
    Cut,  // Move files
    Copy, // Copy files
}

impl Default for FileOperationMode {
    fn default() -> Self {
        FileOperationMode::Cut
    }
}

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
    pub fn organize_by_extension(path: &str, mode: FileOperationMode) -> Result<OrganizerResult, String> {
        // Call the new function with a no-op progress callback
        Self::organize_by_extension_with_progress(path, mode, &|_, _| {})
    }

    /// Organiza archivos por extensión en una carpeta "Organizer" con callback de progreso
    pub fn organize_by_extension_with_progress<F>(
        path: &str,
        mode: FileOperationMode,
        progress_callback: F
    ) -> Result<OrganizerResult, String>
    where
        F: Fn(usize, usize), // (current, total)
    {
        // Call the new function with an empty exclusion list
        Self::organize_by_extension_with_progress_and_exclusions(path, mode, &Vec::new(), progress_callback)
    }

    /// Organiza archivos por extensión en una carpeta "Organizer" con callback de progreso y exclusiones
    pub fn organize_by_extension_with_progress_and_exclusions<F>(
        path: &str,
        mode: FileOperationMode,
        excluded_items: &[String],
        progress_callback: F
    ) -> Result<OrganizerResult, String>
    where
        F: Fn(usize, usize), // (current, total)
    {
        let path = Path::new(path);

        // Verificar si la ruta existe
        if !path.exists() {
            return Err(format!("La ruta '{}' no existe", path.display()));
        }

        // Verificar si es un directorio
        if !path.is_dir() {
            return Err("Solo se pueden organizar archivos dentro de directorios".to_string());
        }

        // Convertimos los elementos excluidos a minúsculas para comparación
        let excluded_items_lower: Vec<String> = excluded_items
            .iter()
            .map(|item| item.trim().to_lowercase())
            .collect();

        // Recursively collect all files to process (excluding specified items)
        let mut all_files = Vec::new();
        Self::collect_files_recursive(path, &excluded_items_lower, &mut all_files)?;

        if all_files.is_empty() {
            return Ok(OrganizerResult::empty());
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

        // Procesar todos los archivos encontrados
        for (idx, file_path) in all_files.iter().enumerate() {
            // Reportar progreso
            progress_callback(idx + 1, all_files.len());

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

            // Move or copy file to the corresponding folder
            let file_name = file_path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "archivo".to_string());

            let new_path = extension_folder.join(&file_name);

            // Perform the operation based on mode
            let operation_result = match mode {
                FileOperationMode::Cut => fs::rename(file_path, &new_path),
                FileOperationMode::Copy => fs::copy(file_path, &new_path).map(|_| ()),
            };

            match operation_result {
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

        // Create summary of organization
        let summary = Self::create_organization_summary(&extension_map, moved_files, created_folders, mode);

        Ok(OrganizerResult {
            total_moved: moved_files,
            folders_created: created_folders,
            extension_map,
            summary,
            errors: if errors.is_empty() { None } else { Some(errors) },
        })
    }

    /// Recursively collects all files in a directory and subdirectories, excluding specified items
    fn collect_files_recursive(
        dir_path: &Path,
        excluded_items: &[String],
        files: &mut Vec<std::path::PathBuf>,
    ) -> Result<(), String> {
        match fs::read_dir(dir_path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            
                            // Check if the item should be excluded
                            if let Some(item_name) = path.file_name() {
                                let item_name_lower = item_name.to_string_lossy().to_lowercase();
                                
                                // Skip if it's the Organizer folder or in the exclusion list
                                if item_name_lower == "organizer" || excluded_items.contains(&item_name_lower) {
                                    continue;
                                }
                            }

                            if path.is_file() {
                                files.push(path);
                            } else if path.is_dir() {
                                // Recursively collect files from subdirectories
                                Self::collect_files_recursive(&path, excluded_items, files)?;
                            }
                        }
                        Err(e) => {
                            return Err(format!("Error leyendo entrada: {}", e));
                        }
                    }
                }
            }
            Err(e) => return Err(format!("Error al leer el directorio: {}", e)),
        }

        Ok(())
    }

    /// Crea un resumen de la organización
    fn create_organization_summary(
        extension_map: &HashMap<String, Vec<String>>,
        total_moved: usize,
        folders_created: usize,
        mode: FileOperationMode,
    ) -> String {
        let operation_name = match mode {
            FileOperationMode::Cut => "movidos",
            FileOperationMode::Copy => "copiados",
        };
        
        let mut summary = format!(
            "Organizacion completada\n\n\
             Archivos {}: {}\n\
             Carpetas creadas: {}\n\n\
             Extensiones organizadas:\n",
            operation_name, total_moved, folders_created
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

impl OrganizerResult {
    pub fn empty() -> Self {
        OrganizerResult {
            total_moved: 0,
            folders_created: 0,
            extension_map: HashMap::new(),
            summary: "No files were processed.".to_string(),
            errors: None,
        }
    }
}