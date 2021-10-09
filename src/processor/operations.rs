use super::operation_map::function_from_instruction;
use super::Cpu;
use super::CHIP8_HEIGHT;
use super::CHIP8_WIDTH;
use rand::Rng;

pub struct Opcode {
    instruction_bytes: (u8, u8),
}

impl Opcode {
    pub fn from_cpu(cpu: &Cpu) -> Self {
        Self {
            instruction_bytes: (
                cpu.memory[cpu.program_counter],
                cpu.memory[cpu.program_counter + 1],
            ),
        }
    }

    pub fn _from_bytes(bytes: u16) -> Self {
        Self {
            instruction_bytes: (((bytes & 0xff00) >> 8) as u8, (bytes & 0xff) as u8),
        }
    }

    pub fn bytes(&self) -> (u8, u8) {
        self.instruction_bytes
    }

    pub fn nibbles(&self) -> (u8, u8, u8, u8) {
        (
            self.instruction_bytes.0 >> 4,
            self.instruction_bytes.0 & 0xf,
            self.instruction_bytes.1 >> 4,
            self.instruction_bytes.1 & 0xf,
        )
    }

    pub fn address(&self) -> u16 {
        let b1: u16 = self.instruction_bytes.0.into();
        let b2: u16 = self.instruction_bytes.1.into();

        ((b1 & 0xf) << 8) + b2
    }

    pub fn register_x(&self) -> usize {
        self.nibbles().1.into()
    }

    pub fn register_y(&self) -> usize {
        self.nibbles().2.into()
    }

    pub fn byte(&self) -> u8 {
        self.bytes().1
    }

    pub fn nibble(&self) -> u8 {
        self.nibbles().3
    }
}

pub fn execute_instruction(cpu: &mut Cpu) {
    let ins = Opcode::from_cpu(cpu);
    function_from_instruction(&ins)(cpu, &ins);
}

// 0 out display, set redraw flag to 1
pub fn clear_display(cpu: &mut Cpu, _ins: &Opcode) {
    cpu.display = [0; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize];
    cpu.redraw_flag = true;
}

// set pc to address on top of stack, subtract one from sp
pub fn return_from_subroutine(cpu: &mut Cpu, _ins: &Opcode) {
    cpu.program_counter = cpu.stack[cpu.stack_pointer].into();
    cpu.stack_pointer -= 1;
}

// set pc to addr
pub fn jump_to_address(cpu: &mut Cpu, ins: &Opcode) {
    cpu.program_counter = (ins.address() - 2).into(); // subtract 2 to offset pc increment
}

// increment sp, push pc to stack, set pc to addr
pub fn call_subroutine(cpu: &mut Cpu, ins: &Opcode) {
    cpu.stack_pointer += 1;
    cpu.stack[cpu.stack_pointer] = cpu.program_counter as u16;
    cpu.program_counter = (ins.address() - 2).into(); // subtract 2 to offset pc increment
}

// if Vx == byte, skip next instruction
pub fn skip_if_equal(cpu: &mut Cpu, ins: &Opcode) {
    if cpu.register[ins.register_x()] == ins.byte() {
        cpu.program_counter += 2;
    }
}

// if Vx != byte, skip next instruction
pub fn skip_not_equal(cpu: &mut Cpu, ins: &Opcode) {
    if cpu.register[ins.register_x()] != ins.byte() {
        cpu.program_counter += 2;
    }
}

// if Vx == Vy, skip next instruction
pub fn skip_equal_registers(cpu: &mut Cpu, ins: &Opcode) {
    if cpu.register[ins.register_x()] == cpu.register[ins.register_y()] {
        cpu.program_counter += 2;
    }
}

// store byte in Vx
pub fn load_byte(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register[ins.register_x()] = ins.byte();
}

// Vx += byte
pub fn add_byte(cpu: &mut Cpu, ins: &Opcode) {
    let add_result = cpu.register[ins.register_x()].overflowing_add(ins.byte());

    if add_result.1 {
        cpu.register[0xf] = 1;
    } else {
        cpu.register[0xf] = 0;
    }

    cpu.register[ins.register_x()] = add_result.0;
}

// Vx = Vy
pub fn load_register(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register[ins.register_x()] = cpu.register[ins.register_y()];
}

// Vx |= Vy
pub fn bitwise_or(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register[ins.register_x()] |= cpu.register[ins.register_y()];
}

// Vx &= Vy
pub fn bitwise_and(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register[ins.register_x()] &= cpu.register[ins.register_y()];
}

// Vx ^= Vy
pub fn bitwise_xor(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register[ins.register_x()] ^= cpu.register[ins.register_y()];
}

// Vx += Vy, if the result is over 255, set VF to 1, otherwise to 0, store the lowest 8 bits
pub fn add_registers(cpu: &mut Cpu, ins: &Opcode) {
    let add_result = cpu.register[ins.register_x()].overflowing_add(cpu.register[ins.register_y()]);

    if add_result.1 {
        cpu.register[0xf] = 1;
    } else {
        cpu.register[0xf] = 0;
    }

    cpu.register[ins.register_x()] = add_result.0;
}

// Vx -= Vy, set VF = not borrow
// VF = Vx > Vy ? 1 : 0, then subtraction occurs
pub fn subtract_registers(cpu: &mut Cpu, ins: &Opcode) {
    let sub_result = cpu.register[ins.register_x()].overflowing_sub(cpu.register[ins.register_y()]);

    if sub_result.1 {
        cpu.register[0xf] = 0;
    } else {
        cpu.register[0xf] = 1;
    }

    cpu.register[ins.register_x()] = sub_result.0;
}

// shift Vx right, if the least significant bit is 1, Vf=1, otherwise 0
pub fn shift_right(cpu: &mut Cpu, ins: &Opcode) {
    // Vf = least significant bit
    cpu.register[0xf] = cpu.register[ins.register_x()] & 0x1;
    cpu.register[ins.register_x()] /= 2; // shift right
}

// Vx = Vy - Vx, VF = NOT borrow
pub fn subtract_negative(cpu: &mut Cpu, ins: &Opcode) {
    let sub_result = cpu.register[ins.register_y()].overflowing_sub(cpu.register[ins.register_x()]);

    if sub_result.1 {
        cpu.register[0xf] = 0;
    } else {
        cpu.register[0xf] = 1;
    }

    cpu.register[ins.register_x()] = sub_result.0;
}

// shift Vx left, if the most significant bit is 1, Vf=1, otherwise 0
pub fn shift_left(cpu: &mut Cpu, ins: &Opcode) {
    // Vf = most significant bit
    cpu.register[0xf] = (cpu.register[ins.register_x()] & 0x80) >> 7;
    cpu.register[ins.register_x()] <<= 1; // shift left
}

// if Vx != Vy increment pc by 2
pub fn skip_not_equal_registers(cpu: &mut Cpu, ins: &Opcode) {
    if cpu.register[ins.register_x()] != cpu.register[ins.register_y()] {
        cpu.program_counter += 2;
    }
}

// set I to addr
pub fn load_register_i(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register_i = ins.address();
}

// set pc to V0 + addr
pub fn jump_register_0(cpu: &mut Cpu, ins: &Opcode) {
    cpu.program_counter = (cpu.register[0] as u16 + ins.address()).into();
}

// random number between in [0,255], AND with byte, store in Vx
pub fn random_byte(cpu: &mut Cpu, ins: &Opcode) {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..=255);
    cpu.register[ins.register_x()] = random_number & ins.byte();
}

// draw a sprite starting at Vx, Vy with width=8 pixels and height=n pixels
// rows read as bit coded string starting from memory at location I
// xor bits onto display, vF is set to 1 if any pixels are flipped,
// vf=0 otherwise
pub fn draw(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register[0xf] = 0;

    let start_x: usize = cpu.register[ins.register_x()] as usize;
    let start_y: usize = cpu.register[ins.register_y()] as usize;
    let height: usize = ins.nibble() as usize;

    for y in 0..height {
        let value: u8 = cpu.memory[cpu.register_i as usize + y]; // value from memory
        for x in 0..8 {
            if value & (0x80 >> x) != 0 { // the xth bit in the byte == 1
            }
            let bit = (value & (0x80 >> x)) >> (7 - x); // the xth bit in the byte
                                                        //let coord: usize = (((start_y + y) % CHIP8_HEIGHT as usize) * CHIP8_WIDTH as usize
                                                        //    + ((start_x + x) % CHIP8_WIDTH as usize)) as usize;
            let coord: usize = (start_y + y) * CHIP8_WIDTH as usize + (start_x + x) as usize;
            let coord = coord % (CHIP8_WIDTH * CHIP8_HEIGHT) as usize;

            if bit == 1 {
                if cpu.display[coord] == 1 {
                    cpu.register[0xf] = 1; // collision
                }
                cpu.display[coord] ^= 1;
            }
        }
    }

    cpu.redraw_flag = true;
}

// skip next instruction if key Vx is pressed
pub fn skip_if_key(cpu: &mut Cpu, ins: &Opcode) {
    if cpu.keypad[cpu.register[ins.register_x()] as usize] {
        cpu.program_counter += 2;
    }
}

// skip next instruction if key Vx is not pressed
pub fn skip_if_not_key(cpu: &mut Cpu, ins: &Opcode) {
    if !cpu.keypad[cpu.register[ins.register_x()] as usize] {
        cpu.program_counter += 2;
    }
}

// set Vx = DT
pub fn set_register_to_delay_timer(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register[ins.register_x()] = cpu.delay_timer;
}

// wait for key, store the value in Vx
pub fn wait_for_key(cpu: &mut Cpu, ins: &Opcode) {
    cpu.keypad_waiting = true;
    cpu.keypad_waiting_register = ins.register_x() as u8;
}

// set DT = Vx
pub fn set_delay_timer(cpu: &mut Cpu, ins: &Opcode) {
    cpu.delay_timer = cpu.register[ins.register_x()];
}

// set ST = Vx
pub fn set_sound_timer(cpu: &mut Cpu, ins: &Opcode) {
    cpu.sound_timer = cpu.register[ins.register_x()];
}

// I += Vx
pub fn add_register_i(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register_i += cpu.register[ins.register_x()] as u16;
}

// set I to the location of the sprite of Vx in memory
pub fn set_register_i_to_sprite(cpu: &mut Cpu, ins: &Opcode) {
    cpu.register_i = cpu.register[ins.register_x()] as u16 * 5;
}

// store BCD representation of Vx at I->I+2 in memory
pub fn store_register_x(cpu: &mut Cpu, ins: &Opcode) {
    let value: u32 = cpu.register[ins.register_x()].into();

    cpu.memory[cpu.register_i as usize] = ((value % 1000) / 100) as u8;
    cpu.memory[(cpu.register_i + 1) as usize] = ((value % 100) / 10) as u8;
    cpu.memory[(cpu.register_i + 2) as usize] = (value % 10) as u8;
}

// store registers V0 to Vx in memory, starting at I
pub fn store_registers(cpu: &mut Cpu, ins: &Opcode) {
    for i in 0..=ins.register_x() {
        cpu.memory[cpu.register_i as usize + i] = cpu.register[i];
    }
}

// read V0 to Vx from memory, starting at I
pub fn read_registers(cpu: &mut Cpu, ins: &Opcode) {
    for i in 0..=ins.register_x() {
        cpu.register[i] = cpu.memory[cpu.register_i as usize + i];
    }
}

pub fn unknown_instruction(_cpu: &mut Cpu, instruction: &Opcode) {
    let bytes = instruction.bytes();
    panic!("Unknown instruction: {:02x?}{:02x?}", bytes.0, bytes.1);
}
