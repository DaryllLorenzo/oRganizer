mod ui; 
mod core;
use eframe::Result;

fn main() -> Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Selector de Ruta",
        options,
        Box::new(|_cc| Ok(Box::new(ui::ORganizer::new()))), 
    )
}