// Related docs: http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    // Quick note on u8 it can only store 0 -> 255
    // This technically creates a 4kb storage
    // 0x1000 = 4096
    // 8 bit * 4096 = 4.0960 kilobytes
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        // For example this is: 0x0080 -> 128
        // Shitfing to a 16bit field so that the left shift doesn't
        // return a 0
        let op_bytes = self.memory[p] as u16;
        // For example this is: 0x0014 -> 20
        let op_bytes2 = self.memory[p + 1] as u16;

        // Left shift 8 changes 0x0080 which in binary is
        // 0b0000000010000000 to 0x8000 which is 32768 and in binary
        // 0b1000000000000000. A left shift is the same as multiplying the number by 2
        // In this case we multiply the number by 2 eight times. So the left shift technically does
        // 128 * 2^8
        //
        // op_bytes2 is 0x0014 and in binary is 0b10100
        //
        // Logical or between the two: If the shared position of the bits is 0 the output is 0 if
        // either is 1 the output is 1 i.e
        // 0b1000000000000000 / 0x8000 / 32678
        //         OR
        // 0b0000000000010100 / 0x0014 / 20
        //         =
        // 0b1000000000010100 / 0x8014 / 32788
        op_bytes << 8 | op_bytes2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
