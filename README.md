# Organizer - Organizador de Archivos en Rust

Una aplicación de escritorio multiplataforma que escanea una carpeta seleccionada, agrupa archivos sueltos por su extensión y los organiza en subcarpetas categorizadas dentro de un directorio `Organizer/`.

Construida con **Rust**, **egui** (para la interfaz gráfica) y **rfd** (para diálogos nativos de archivos).

## Características

- **Interfaz Gráfica**: Selección fácil de carpetas mediante diálogo nativo
- **Organización Inteligente**: Escanea archivos sueltos (no carpetas) en el directorio seleccionado
- **Auto-Categorización**: Crea carpetas nombradas por extensión de archivo (ej: `PDF`, `JPG`, `MP3`) dentro de `Organizer/`
- **Archivos sin Extensión**: Maneja archivos sin extensiones en una carpeta `Sin_Extension`
- **Movimiento Seguro**: Los archivos se mueven a sus respectivas carpetas
- **Estadísticas Detalladas**: Muestra un reporte completo del proceso de organización
- **Manejo de Errores**: Gestiona errores del sistema de archivos y los muestra al usuario

## Estructura del Proyecto

```
src/
├── main.rs          # Punto de entrada — lanza la interfaz
├── ui.rs            # Capa de interfaz — componentes egui y manejo de eventos
└── core.rs          # Lógica principal — escaneo, organización y movimiento de archivos
```

## Cómo Funciona

1. **Seleccionar Carpeta**: El usuario selecciona una carpeta objetivo mediante el botón "Seleccionar Carpeta..."
2. **Listar Archivos**: Clic en "Listar Archivos" para ver todos los archivos y carpetas en el directorio seleccionado
3. **Organizar Archivos**: Clic en "Organizar por Extension" para:
   - Escanear la carpeta en busca de archivos (ignora subdirectorios y la carpeta Organizer)
   - Crear directorio `Organizer/` si no existe
   - Para cada archivo, extraer su extensión y crear la carpeta correspondiente
   - Archivos sin extensiones van a la carpeta `Sin_Extension/`
   - Mover archivos a sus carpetas correspondientes
4. **Ver Resultados**: Ver archivos movidos, carpetas creadas y cualquier error encontrado

## Estructura de Directorios Creada

Después de la organización, la carpeta seleccionada contendrá:

```
CarpetaSeleccionada/
├── Organizer/
│   ├── PDF/          # Todos los archivos .pdf
│   ├── JPG/          # Todos los archivos .jpg/.jpeg  
│   ├── TXT/          # Todos los archivos .txt
│   ├── ZIP/          # Todos los archivos .zip
│   ├── EXE/          # Todos los archivos .exe
│   └── Sin_Extension/# Archivos sin extensiones
└── (carpetas restantes y el Organizer)

## Requisitos Previos

- Rust (versión estable) — https://rustup.rs
- Cargo (gestor de paquetes de Rust)

## Compilar y Ejecutar

```bash
git clone https://github.com/DaryllLorenzo/oRganizer.git
cd oRganizer
cargo run
```

## Uso de la Aplicación

1. Ejecuta la aplicación con `cargo run`
2. Haz clic en "Seleccionar Carpeta..." para elegir la carpeta que deseas organizar
3. Opcional: Haz clic en "Listar Archivos" para ver el contenido actual
4. Haz clic en "Organizar por Extension" para iniciar el proceso de organización
5. Revisa el resumen con las estadísticas de la operación
6. Usa "Limpiar" para resetear la interfaz

## Notas Importantes

- La aplicación solo organiza archivos en el nivel superior del directorio seleccionado
- No organiza archivos dentro de subcarpetas
- No modifica la carpeta `Organizer/` si ya existe
- Los archivos se mueven, no se copian (operación permanente)
- Se recomienda hacer una copia de seguridad antes de organizar archivos importantes

## Tecnologías Utilizadas

- **Rust**: Lenguaje de programación principal
- **egui**: Biblioteca para interfaz gráfica de usuario
- **rfd**: Diálogos nativos de archivos
- **std::fs**: Operaciones del sistema de archivos de Rust

## Contribuir

1. Haz fork del repositorio
2. Crea una rama para tu funcionalidad (`git checkout -b nueva-funcionalidad`)
3. Haz commit de tus cambios (`git commit -am 'Agrega nueva funcionalidad'`)
4. Push a la rama (`git push origin nueva-funcionalidad`)
5. Crea un Pull Request

## Licencia

Este proyecto está bajo la licencia MIT. Ver el archivo `LICENSE` para más detalles.