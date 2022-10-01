pub fn disassamble8080(memory: &[u8; 0xFFFF], pc: usize) -> usize {
    let mut op_bytes: usize = 1;

    let opcode = memory[pc];

    match opcode {
        0x00 => println!("NOP"),
        0x01 => {
            println!("LXI {:X},{:X}", memory[pc + 2], memory[pc + 1]);
            op_bytes = 3;
        }
        0x02 => {
            println!("STAX {:X}", memory[pc + 1]);
        }
        0x03 => {
            println!("INX {:X}", memory[pc + 1])
        }
        0x04 => {
            println!("INR {:X}", memory[pc + 1])
        }
        0x05 => {
            println!("DCR {:X}", memory[pc + 1])
        }
        _ => {}
    }

    op_bytes
}
