use super::operations;
use crate::processor::operations::Opcode;
use crate::Cpu;

pub fn function_from_instruction(ins: &Opcode) -> fn(&mut Cpu, &Opcode) {
    match ins.nibbles().0 {
        0x0 => FUNCTION_MAP_0[ins.nibbles().3 as usize],
        0x8 => FUNCTION_MAP_8[ins.nibbles().3 as usize],
        0xe => FUNCTION_MAP_E[ins.nibbles().3 as usize],
        0xf => FUNCTION_MAP_F[ins.bytes().1 as usize],
        _ => FUNCTION_MAP[ins.nibbles().0 as usize],
    }
}

const FUNCTION_MAP: [fn(&mut Cpu, &Opcode); 16] = [
    operations::unknown_instruction,
    operations::jump_to_address,
    operations::call_subroutine,
    operations::skip_if_equal,
    operations::skip_not_equal,
    operations::skip_equal_registers,
    operations::load_byte,
    operations::add_byte,
    operations::unknown_instruction,
    operations::skip_not_equal_registers,
    operations::load_register_i,
    operations::jump_register_0,
    operations::random_byte,
    operations::draw,
    operations::unknown_instruction,
    operations::unknown_instruction,
];

const FUNCTION_MAP_0: [fn(&mut Cpu, &Opcode); 15] = [
    operations::clear_display,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::return_from_subroutine,
];

const FUNCTION_MAP_8: [fn(&mut Cpu, &Opcode); 15] = [
    operations::load_register,
    operations::bitwise_or,
    operations::bitwise_and,
    operations::bitwise_xor,
    operations::add_registers,
    operations::subtract_registers,
    operations::shift_right,
    operations::subtract_negative,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::shift_left,
];

const FUNCTION_MAP_E: [fn(&mut Cpu, &Opcode); 15] = [
    operations::unknown_instruction,
    operations::skip_if_not_key,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::skip_if_key,
];

const FUNCTION_MAP_F: [fn(&mut Cpu, &Opcode); 102] = [
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::set_register_to_delay_timer,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::wait_for_key,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::set_delay_timer,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::set_sound_timer,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::add_register_i,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::set_register_i_to_sprite,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::store_register_x,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::store_registers,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::unknown_instruction,
    operations::read_registers,
];
