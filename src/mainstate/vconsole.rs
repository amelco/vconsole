use core::time;
use std::error::Error;
use std::thread;
use tetra::Context;

use crate::mainstate::vconsole::cpu::Cpu;
use crate::mainstate::vconsole::ram::Ram;

mod cpu;
mod ram;

pub struct VConsole {
    pub cpu: Cpu,
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

impl VConsole {
    pub fn new(ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
        println!("### Construtor!!!");
        Ok(Self::default())
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let dt = tetra::time::get_delta_time(ctx).as_secs_f32();
        thread::sleep(time::Duration::from_secs(1));
        self.cpu.execute(&mut self.ram);
    }

    pub fn update_cpu_info(&mut self, ctx: &mut Context) -> String {
        self.cpu.show()
    }
}
