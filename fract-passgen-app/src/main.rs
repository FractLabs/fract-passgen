use fract_passgen_core::passgen;
use std::{time::{Duration, Instant}};
use eframe::{egui, NativeOptions};
use egui::TextEdit;



struct PassGenApp {
    pass_length: i32,
    uppercase_letters: bool,
    letters: bool,
    numbers: bool,
    special_characters: bool,
    final_pass: String,
    last_copied: Option<Instant>
   }
   
   impl Default for PassGenApp {
       fn default() -> Self {

           Self {
           pass_length: 9,
           uppercase_letters: true,
           letters: true,
           numbers: true,
           special_characters: true,
           last_copied: None,
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
                ui.horizontal(|ui|{
                    ui.label("Password: ");
                    ui.add(
                        TextEdit::multiline(&mut self.final_pass)
                        .code_editor()
                        .desired_rows(1)
                        .desired_width(f32::INFINITY)
                        .lock_focus(true)
                        .interactive(false)
                        .cursor_at_end(false),
                    );

                });
                ui.horizontal(|ui|{
                    if ui.button("Copy Password").clicked() {
                        ctx.copy_text(self.final_pass.clone());
                        self.last_copied = Some(Instant::now());
                        ctx.request_repaint();
                }

                if let Some(t) = self.last_copied {
                    if t.elapsed() < Duration::from_secs(1) {
                        ui.label("Copied!");
                        ctx.request_repaint();
                    } else {
                        self.last_copied = None;
                    }
                }
            });

            });


        
}
}

fn main() -> eframe::Result {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size(egui::vec2(300.0, 200.0)),
        ..Default::default()
    };
    eframe::run_native("Fract-Passgen",
    options,
    Box::new(|_cc| Ok(Box::new(PassGenApp::default()))),
    )?;



    Ok(())
}
