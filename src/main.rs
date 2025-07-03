mod passgen;
use eframe::{egui, NativeOptions};

use crate::passgen::passgen;



struct PassGenApp {
    pass_length: i32,
    uppercase_letters: bool,
    letters: bool,
    numbers: bool,
    special_characters: bool,
    final_pass: String,
   }
   
   impl Default for PassGenApp {
       fn default() -> Self {

           Self {
           pass_length: 9,
           uppercase_letters: true,
           letters: true,
           numbers: true,
           special_characters: true,
           final_pass: String::new(),
       }
   }
}

impl eframe::App for PassGenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Password config");
                ui.horizontal(|ui|{
                    ui.label("Password Length");
//                    ui.add(egui::Slider::new(&mut self.pass_length, 1..=16));
                    ui.add(egui::DragValue::new(&mut self.pass_length))
                });

                ui.checkbox(&mut self.letters, "Letters");
                ui.checkbox(&mut self.numbers, "Numbers");
                ui.checkbox(&mut self.uppercase_letters, "Uppercase Letters");
                ui.checkbox(&mut self.special_characters, "Special Characters");


                if ui.button("Generate Password").clicked() {
                    self.final_pass = passgen
                    (
                        self.pass_length,
                        self.uppercase_letters,
                        self.letters,
                        self.numbers,
                        self.special_characters
                    )
                }

                ui.label(format!("Password: {}", self.final_pass));

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
