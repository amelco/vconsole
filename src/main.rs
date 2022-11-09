use cpu::Cpu;
use ram::Ram;

mod ram;
mod cpu;


fn main() {
    let mut ram = Ram::new();
    let mut cpu = Cpu::new();

    ram.data[0x3940] = 0xAA;
    ram.data[0x3941] = 0x21;
    ram.data[0x3942] = 0xBA;
    ram.data[0x3943] = 0x39;
    ram.data[0x3944] = 0x50;
    ram.data[0x3945] = 0xCA;
    ram.data[0x3946] = 0x39;
    ram.data[0x3947] = 0x60;

    ram.data[0x3960] = 0xAA;
    ram.data[0x3961] = 0xDE;
    ram.data[0x3962] = 0xCF;

    ram.data[0x3948] = 0xBA;
    ram.data[0x3949] = 0x39;
    ram.data[0x394A] = 0x50;


    ram.print(8, 0x3940, 0x39ff);
    loop {
        println!("");
        cpu.execute(&mut ram);
        cpu.print();
        ram.print(8, 0x3940, 0x39ff);
    }

}
