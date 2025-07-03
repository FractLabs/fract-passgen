mod passgen;
use eframe::{egui, NativeOptions};



struct PassGenApp {
    pass_length: i32,
    uppercase_letters: bool,
    letters: bool,
    numbers: bool,
    special_characters: bool,
   }
   
   impl Default for PassGenApp {
       fn default() -> Self {

           Self {
           pass_length: 9,
           uppercase_letters: true,
           letters: true,
           numbers: true,
           special_characters: true,
       }
   }
}

impl eframe::App for PassGenApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Password config");
            });
    }
}

fn main() -> eframe::Result {
    let options = NativeOptions::default();

    eframe::run_native("PassGen",
    options,
    Box::new(|_cc| Ok(Box::new(PassGenApp::default()))),
    )?;



    Ok(())
}
