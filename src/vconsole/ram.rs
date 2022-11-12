// 64 KB: 
//   0x0000 - 0x383f => VRAM (80 x 60 pixels); 
//     1 pixel = 8 bits = 256 colors
//   0x3840 - 0x393f => characters (6 x 6 pixels)
//   0x3940 - 0xffff => RAM
// VRAM:
//   80 x 60 rgb pixels

pub struct Ram {
    pub data: Vec<u8>
}

impl Ram {
    pub fn new() -> Ram {
        Ram { 
            data: vec![0xff; 0x10000],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    pub fn load_cartridge(&mut self) {
        self.data[0x3940] = 0xAA;
        self.data[0x3941] = 0x21;
        self.data[0x3942] = 0xBA;
        self.data[0x3943] = 0x39;
        self.data[0x3944] = 0x50;
        self.data[0x3945] = 0xCA;
        self.data[0x3946] = 0x39;
        self.data[0x3947] = 0x60;
    
        self.data[0x3960] = 0xAA;
        self.data[0x3961] = 0xDE;
        self.data[0x3962] = 0xCF;
    
        self.data[0x3948] = 0xBA;
        self.data[0x3949] = 0x39;
        self.data[0x394A] = 0x50;
    }

    pub fn print(&self, num_columns: u8, begin: usize, end: usize) {
        for i in 0..end-begin+1 {
            if i % (num_columns as usize * 2) == 0 {
                if i > 0 {
                    println!("");
                }
                print!("[{:#04X}] ", i+begin)
            }
            if i % 2 as usize == 0 {
                print!(" ")
            }
            print!("{:02X}", self.data[i+begin])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_ok() {
        let ram = Ram::new();
        let data = ram.read(0x1010);

        assert_eq!(data, 0xff);
    }

    fn write_ok() {
        let mut ram = Ram::new();
        ram.write(0x3590, 0xfe);
        let value = ram.read(0x3590);

        assert_eq!(0xfe, value);
    }
}