use crate::ram::Ram;

pub struct Cpu {
    pub init_address: u16,
    pub reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    pc: u16,
    stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { 
            init_address: 0x3940,
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            pc: 0x3940,
            stack: vec![0; 64],
        }
    }

    pub fn execute(&mut self, ram: &mut Ram) {
        let opcode = ram.read(self.pc);
        self.pc += 1;
        match opcode {
            0xaa => {
                // lda absolute mode
                self.reg_a = ram.read(self.pc);
            }
            0xba => {
                // sta
                let addr = self.read_addr(ram);
                ram.write(addr, self.reg_a);
            }
            0xca => {
                // jump
                let addr = self.read_addr(ram);
                self.stack.push(self.pc + 1);
                self.pc = addr - 1;
            }
            0xcf => {
                // rts
                let addr = self.stack.pop().unwrap();
                self.pc = addr - 1;
            }
            0xff => {
                // shutdown
                panic!("SAIU");
            }
            _ => return
        }
        self.pc += 1;
    }

    pub fn print(&self) {
        println!("");
        println!("PC:     {:#04x}", self.pc);
        println!("A:      {:02x}",  self.reg_a);
    }

    fn read_addr(&mut self, ram: &mut Ram) -> u16 {
        let hb = ram.read(self.pc);
        self.pc += 1;
        let lb = ram.read(self.pc);
        let addr : u16= (hb as u16) << 8 | lb as u16;
        addr
    }
}