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
        let mut app = Self {
            cpu: Cpu::new(),
            ram: Ram::new(),
            name: "Andre".to_owned(),
            age: 40,
        };
        app.ram.load_cartridge();
        app
    }
}

impl eframe::App for VConsole {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.label(self.cpu.show());
        });
    }
}