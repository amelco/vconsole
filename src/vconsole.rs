mod ram;
mod cpu;

use ram::Ram;
use cpu::Cpu;

pub struct VConsole {
    cpu: Cpu,
    ram: Ram,
    name: String,
    age: u8,
}

impl Default for VConsole {
    fn default() -> Self {
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(),
            name: "Andre".to_owned(),
            age: 40,
        }
    }
}

impl eframe::App for VConsole {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}