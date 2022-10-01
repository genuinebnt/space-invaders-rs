use crate::disassembler::disassamble8080;

struct CpuFlags {
    z: bool,
    s: bool,
    p: bool,
    c: bool,
    ac: bool,
}

struct Cpu8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: CpuFlags,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: [u8; 0xFFFF],
}

impl From<CpuFlags> for u8 {
    fn from(flags: CpuFlags) -> Self {
        (flags.s as u8) << 7
            | (flags.z as u8) << 6
            | (flags.ac as u8) << 4
            | (flags.p as u8) << 2
            | (flags.c as u8)
    }
}

impl From<u8> for CpuFlags {
    fn from(value: u8) -> Self {
        CpuFlags {
            z: ((value & 0b10000000) >> 7) == 1,
            s: ((value & 0b01000000) >> 6) == 1,
            p: ((value & 0b00010000) >> 4) == 1,
            c: ((value & 0b00000100) >> 2) == 1,
            ac: (value & 0b00000001) == 1,
        }
    }
}

impl Cpu8080 {
    fn new() -> Self {
        Cpu8080 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: CpuFlags {
                z: false,
                s: false,
                p: false,
                c: false,
                ac: false,
            },
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn load_rom(&mut self, path: &str, addr: usize) {
        std::fs::read(path)
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(i, value)| self.memory[addr + i] = *value);
        //println!("{:x?}", self.memory);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_rom_test() {
        let mut cpu = Cpu8080::new();
        cpu.load_rom("roms/spaceinvaders/invaders.concatenated", 0);

        let mut pc: usize = 0;

        while pc < 0x100 {
            pc += disassamble8080(&cpu.memory, pc);
        }
    }
}
