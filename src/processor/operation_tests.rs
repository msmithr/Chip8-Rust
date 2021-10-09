use super::cpu::*;
use super::operation_map::function_from_instruction;
use super::operations::*;
use super::CHIP8_HEIGHT;
use super::CHIP8_WIDTH;

fn setup(bytes: u16) -> (Cpu, Opcode, fn(&mut Cpu, &Opcode)) {
    let empty_program = Vec::<u8>::new();
    let cpu: Cpu = match Cpu::new(&empty_program) {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    let instruction: Opcode = Opcode::_from_bytes(bytes);

    let function = function_from_instruction(&instruction);

    (cpu, instruction, function)
}

#[test]
fn test_clear_display() {
    let (mut cpu, instruction, function) = setup(0x00e0);

    for i in 0..CHIP8_HEIGHT * CHIP8_WIDTH {
        cpu.display[i as usize] = 0x5;
    }

    function(&mut cpu, &instruction);

    for i in 0..(CHIP8_HEIGHT * CHIP8_WIDTH) {
        assert_eq!(cpu.display[i as usize], 0x0);
    }
}

#[test]
fn test_return_from_subroutine() {
    let (mut cpu, instruction, function) = setup(0x00ee);
    cpu.program_counter = 0x32;
    cpu.stack_pointer = 6;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, cpu.stack[6] as usize);
    assert_eq!(cpu.stack_pointer, 5);
}

#[test]
fn test_jump_to_address() {
    let (mut cpu, instruction, function) = setup(0x1536);
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x534);
}

#[test]
fn test_call_subroutine() {
    let (mut cpu, instruction, function) = setup(0x2536);
    cpu.program_counter = 0x32;
    cpu.stack_pointer = 3;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.stack_pointer, 4);
    assert_eq!(cpu.stack[4], 0x32);
    assert_eq!(cpu.program_counter, 0x534);
}

#[test]
fn test_skip_if_equal() {
    let (mut cpu, instruction, function) = setup(0x3412);
    cpu.register[0x4] = 0x12;
    cpu.program_counter = 0x32;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x34);
    cpu.register[0x4] = 0x14;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x34);
}

#[test]
fn test_skip_not_equal() {
    let (mut cpu, instruction, function) = setup(0x4412);
    cpu.register[0x4] = 0x12;
    cpu.program_counter = 0x32;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x32);
    cpu.register[0x4] = 0x14;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x34);
}

#[test]
fn test_skip_equal_registers() {
    let (mut cpu, instruction, function) = setup(0x5270);
    cpu.register[0x2] = 0x3;
    cpu.register[0x7] = 0x3;
    cpu.program_counter = 0x32;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x34);
    cpu.register[0x7] = 0x4;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x34);
}

#[test]
fn test_load_byte() {
    let (mut cpu, instruction, function) = setup(0x6870);
    cpu.register[0x8] = 0x0;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x8], 0x70);
}

#[test]
fn test_add_byte() {
    let (mut cpu, instruction, function) = setup(0x7870);
    cpu.register[0x8] = 0x3;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x8], 0x73);
    assert_eq!(cpu.register[0xf], 0x0);
    cpu.register[0x8] = 0xf0;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x8], 0x60);
    assert_eq!(cpu.register[0xf], 0x1);
}

#[test]
fn test_load_register() {
    let (mut cpu, instruction, function) = setup(0x8150);
    cpu.register[0x1] = 0x3;
    cpu.register[0x5] = 0xa;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x1], 0xa);
    assert_eq!(cpu.register[0x5], 0xa);
}

#[test]
fn test_bitwise_or() {
    let (mut cpu, instruction, function) = setup(0x8151);
    cpu.register[0x1] = 0x3;
    cpu.register[0x5] = 0xa;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x1], 0xb);
    assert_eq!(cpu.register[0x5], 0xa);
}

#[test]
fn test_bitwise_and() {
    let (mut cpu, instruction, function) = setup(0x8152);
    cpu.register[0x1] = 0x3;
    cpu.register[0x5] = 0xa;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x1], 0x2);
    assert_eq!(cpu.register[0x5], 0xa);
}

#[test]
fn test_bitwise_xor() {
    let (mut cpu, instruction, function) = setup(0x8153);
    cpu.register[0x1] = 0x3;
    cpu.register[0x5] = 0xa;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x1], 0x9);
    assert_eq!(cpu.register[0x5], 0xa);
}

#[test]
fn test_add_registers() {
    let (mut cpu, instruction, function) = setup(0x8154);
    cpu.register[0x1] = 0x3;
    cpu.register[0x5] = 0xa;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x1], 0xd);
    assert_eq!(cpu.register[0x5], 0xa);
    assert_eq!(cpu.register[0xf], 0x0);
    cpu.register[0x1] = 0xf0;
    cpu.register[0x5] = 0xaa;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x1], 0x9a);
    assert_eq!(cpu.register[0x5], 0xaa);
    assert_eq!(cpu.register[0xf], 0x1);
}

#[test]
fn test_subtract_registers() {
    let (mut cpu, instruction, function) = setup(0x8155);
    cpu.register[0x1] = 0x3;
    cpu.register[0x5] = 0x5;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x1], 0xfe);
    assert_eq!(cpu.register[0x5], 0x5);
    assert_eq!(cpu.register[0xf], 0x0);
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x1], 0xf9);
    assert_eq!(cpu.register[0x5], 0x5);
    assert_eq!(cpu.register[0xf], 0x1);
}

#[test]
fn test_shift_right() {
    let (mut cpu, instruction, function) = setup(0x8306);
    cpu.register[0x3] = 0x3e;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x3], 0x1f);
    assert_eq!(cpu.register[0xf], 0x0);
    cpu.register[0x3] = 0x1;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x3], 0x0);
    assert_eq!(cpu.register[0xf], 0x1);
}

#[test]
fn test_subtract_negative() {
    let (mut cpu, instruction, function) = setup(0x83a7);
    cpu.register[0x3] = 0x5;
    cpu.register[0xa] = 0x3;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x3], 0xfe);
    assert_eq!(cpu.register[0xa], 0x3);
    assert_eq!(cpu.register[0xf], 0x0);
    cpu.register[0x3] = 0x3;
    cpu.register[0xa] = 0x5;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x3], 0x2);
    assert_eq!(cpu.register[0xa], 0x5);
    assert_eq!(cpu.register[0xf], 0x1);
}

#[test]
fn test_shift_left() {
    let (mut cpu, instruction, function) = setup(0x830e);
    cpu.register[0x3] = 0x5;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x3], 0xa);
    assert_eq!(cpu.register[0xf], 0x0);
    cpu.register[0x3] = 0x85;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x3], 0xa);
    assert_eq!(cpu.register[0xf], 0x1);
}

#[test]
fn test_skip_not_equal_registers() {
    let (mut cpu, instruction, function) = setup(0x9270);
    cpu.register[0x2] = 0x3;
    cpu.register[0x7] = 0x3;
    cpu.program_counter = 0x32;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x32);
    cpu.register[0x7] = 0x4;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x34);
}

#[test]
fn test_load_i() {
    let (mut cpu, instruction, function) = setup(0xa87a);
    cpu.register_i = 0x320;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register_i, 0x87a);
}

#[test]
fn test_jump_register_0() {
    let (mut cpu, instruction, function) = setup(0xB87a);
    cpu.register[0x0] = 0x8;
    cpu.program_counter = 0x30;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.program_counter, 0x882);
}

#[test]
fn test_random_byte() {
    let (mut cpu, instruction, function) = setup(0xC87a);
    cpu.register[0x8] = 0x3;
    function(&mut cpu, &instruction);
    assert_ne!(cpu.register[0x8], 0x3);
}

#[test]
fn test_draw() {
    // todo
}

#[test]
fn test_skip_if_key() {
    // todo
}

#[test]
fn test_skip_if_not_key() {
    // todo
}

#[test]
fn test_set_register_to_delay_timer() {
    let (mut cpu, instruction, function) = setup(0xf807);
    cpu.delay_timer = 0x66;
    cpu.register[0x8] = 0x3;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x8], 0x66);
    assert_eq!(cpu.delay_timer, 0x66);
}

#[test]
fn test_wait_for_key() {
    // todo
}

#[test]
fn test_set_delay_timer() {
    let (mut cpu, instruction, function) = setup(0xf815);
    cpu.delay_timer = 0x66;
    cpu.register[0x8] = 0x3;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x8], 0x3);
    assert_eq!(cpu.delay_timer, 0x3);
}

#[test]
fn test_set_sound_timer() {
    let (mut cpu, instruction, function) = setup(0xf818);
    cpu.sound_timer = 0x66;
    cpu.register[0x8] = 0x3;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x8], 0x3);
    assert_eq!(cpu.sound_timer, 0x3);
}

#[test]
fn test_add_register_i() {
    let (mut cpu, instruction, function) = setup(0xf81e);
    cpu.register_i = 0x50;
    cpu.register[0x8] = 0x30;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register_i, 0x80);
    assert_eq!(cpu.register[0x8], 0x30);
}

#[test]
fn test_set_register_i_to_sprite() {
    // todo
}

#[test]
fn test_store_register_x() {
    let (mut cpu, instruction, function) = setup(0xf733);
    cpu.register_i = 0x50;
    cpu.register[0x7] = 0x88; // 136
    function(&mut cpu, &instruction);
    assert_eq!(cpu.memory[0x50], 0x1);
    assert_eq!(cpu.memory[0x51], 0x3);
    assert_eq!(cpu.memory[0x52], 0x6);
}

#[test]
fn test_store_registers() {
    let (mut cpu, instruction, function) = setup(0xf555);
    cpu.register_i = 0x50;
    cpu.register[0x0] = 0x3;
    cpu.register[0x1] = 0x8;
    cpu.register[0x2] = 0x9;
    cpu.register[0x3] = 0xe;
    cpu.register[0x4] = 0x1;
    cpu.register[0x5] = 0xa;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.memory[0x50], 0x3);
    assert_eq!(cpu.memory[0x51], 0x8);
    assert_eq!(cpu.memory[0x52], 0x9);
    assert_eq!(cpu.memory[0x53], 0xe);
    assert_eq!(cpu.memory[0x54], 0x1);
    assert_eq!(cpu.memory[0x55], 0xa);
}

#[test]
fn test_read_registers() {
    let (mut cpu, instruction, function) = setup(0xf565);
    cpu.register_i = 0x50;
    cpu.memory[0x50] = 0x3;
    cpu.memory[0x51] = 0x8;
    cpu.memory[0x52] = 0x9;
    cpu.memory[0x53] = 0xe;
    cpu.memory[0x54] = 0x1;
    cpu.memory[0x55] = 0xa;
    function(&mut cpu, &instruction);
    assert_eq!(cpu.register[0x0], 0x3);
    assert_eq!(cpu.register[0x1], 0x8);
    assert_eq!(cpu.register[0x2], 0x9);
    assert_eq!(cpu.register[0x3], 0xe);
    assert_eq!(cpu.register[0x4], 0x1);
    assert_eq!(cpu.register[0x5], 0xa);
}
