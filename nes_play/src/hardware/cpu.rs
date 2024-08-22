use super::bus::Bus;
use super::opcode;
use super::opcode::Opcode;
use super::opcode::AddressingModes;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Cpu {
    program_counter: u16,
    stack_pointer: u8,
    register_a: u8,
    register_x: u8,
    register_y: u8,
    processor_status: ProcessStatus,
    bus: Rc<RefCell<Bus>>,
}

struct ProcessStatus {
    pub status: u8
}

impl ProcessStatus {
    pub fn new() -> Self {
        ProcessStatus {
            status: 0,
        }
    }

    pub fn set_carry_flag(&mut self) {
        self.status |= 0b00000001;
    }

    pub fn set_zero_flag(&mut self) {
        self.status |= 0b00000010;
    }

    pub fn set_interrupt_disable_flag(&mut self) {
        self.status |= 0b00000100;
    }

    pub fn set_decimal_mode_flag(&mut self) {
        self.status |= 0b00001000;
    }

    pub fn set_break_command(&mut self) {
        self.status |= 0b00010000;
    }

    pub fn set_overflow_flag(&mut self) {
        self.status |= 0b00100000;
    }

    pub fn set_negative_flag(&mut self) {
        self.status |= 0b01000000;
    }

    pub fn clear_carry_flag(&mut self) {
        self.status &= 0b11111110;
    }

    pub fn clear_zero_flag(&mut self) {
        self.status &= 0b11111101;
    }

    pub fn clear_interrupt_disable_flag(&mut self) {
        self.status &= 0b11111011;
    }

    pub fn clear_decimal_mode_flag(&mut self) {
        self.status &= 0b11110111;
    }

    pub fn clear_break_command(&mut self) {
        self.status &= 0b11101111;
    }

    pub fn clear_overflow_flag(&mut self) {
        self.status &= 0b11011111;
    }

    pub fn clear_negative_flag(&mut self) {
        self.status &= 0b10111111;
    }

    pub fn get_carry_flag(&mut self) -> u8 {
        self.status & 0b00000001
    }

    pub fn get_zero_flag(&mut self) -> u8 {
        self.status & 0b00000010
    }

    pub fn get_overflow_flag(&mut self) -> u8 {
        self.status & 0b00100000
    }

    pub fn get_negative_flag(&mut self) -> u8 {
        self.status & 0b01000000
    }
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Cpu {
            program_counter: 0,
            stack_pointer: 0xFF,
            register_a: 0,
            register_x: 0,
            register_y: 0,
            processor_status: ProcessStatus::new(),
            bus,
        }
    }

    fn inc_program_counter(&mut self, amount: u16) {
        self.program_counter += amount;
    }

    pub fn set_program_counter(&mut self, address: u16) {
        self.program_counter = address;
    }

    fn stack_push(&mut self, data: u8) {
        let address = self.stack_pointer as u16 | 0x0100;
        self.bus.borrow_mut().write(address, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let address = self.stack_pointer as u16 | 0x0100;
        self.bus.borrow_mut().read(address)
    }

    pub fn execute_opcode(&mut self) {
        let code = self.bus.borrow_mut().read(self.program_counter);
        self.inc_program_counter(1);

        let opcode = opcode::CPU_OP_CODES.iter().find(|opcode| opcode.code == code).unwrap();

        match opcode.title {
            "ADC" => self.add_with_carry(opcode),
            "AND" => self.logical_and(opcode),
            "ASL" => self.arithmetic_shift_left(opcode),
            "BCC" => self.branch_if_carry_clear(opcode),
            "BCS" => self.branch_if_carry_set(opcode),
            "BEQ" => self.branch_if_equal(opcode),
            "BIT" => self.bit_test(opcode),
            "BMI" => self.branch_if_minus(opcode),
            "BNE" => self.branch_if_not_equal(opcode),
            "BPL" => self.branch_if_positive(opcode),
            "BRK" => self.force_interrupt(opcode),
            "BVC" => self.branch_if_overflow_clear(opcode),
            "BVS" => self.branch_if_overflow_set(opcode),
            "CLC" => self.clear_carry_flag(opcode),
            "CLD" => self.clear_decimal_mode(opcode),
            "CLI" => self.clear_interrupt_disable(opcode),
            "CLV" => self.clear_overflow_flag(opcode),
            "CMP" => self.compare(opcode),
            "CPX" => self.compare_x_register(opcode),
            "CPY" => self.compare_y_register(opcode),
            "DEC" => self.decrement_memory(opcode),
            "DEX" => self.decrement_x_register(opcode),
            "DEY" => self.decrement_y_register(opcode),
            "EOR" => self.exclusive_or(opcode),
            "INC" => self.increment_memory(opcode),
            "INX" => self.increment_x_register(opcode),
            "INY" => self.increment_y_register(opcode),
            "JMP" => self.jump(opcode),
            "JSR" => self.jump_to_subroutine(opcode),
            "LDA" => self.load_accumulator(opcode),
            "LDX" => self.load_x_register(opcode),
            "LDY" => self.load_y_register(opcode),
            "LSR" => self.logical_shift_right(opcode),
            "NOP" => self.no_operation(opcode),
            "ORA" => self.logical_inclusive_or(opcode),
            "PHA" => self.push_accumulator(opcode),
            "PHP" => self.push_processor_status(opcode),
            "PLA" => self.pull_accumulator(opcode),
            "PLP" => self.pull_processor_status(opcode),
            "ROL" => self.rotate_left(opcode),
            "ROR" => self.rotate_right(opcode),
            "RTI" => self.return_from_interrupt(opcode),
            "RTS" => self.return_from_subroutine(opcode),
            "SBC" => self.subtract_with_carry(opcode),
            "SEC" => self.set_carry_flag(opcode),
            "SED" => self.set_decimal_flag(opcode),
            "SEI" => self.set_interrupt_disable(opcode),
            "STA" => self.store_accumulator(opcode),
            "STX" => self.store_x_register(opcode),
            "STY" => self.store_y_register(opcode),
            "TAX" => self.transfer_accumulator_to_x(opcode),
            "TAY" => self.transfer_accumulator_to_y(opcode),
            "TSX" => self.transfer_stack_pointer_to_x(opcode),
            "TXA" => self.transfer_x_to_accumulator(opcode),
            "TXS" => self.transfer_x_to_stack_pointer(opcode),
            "TYA" => self.transfer_y_to_accumulator(opcode),
            _ => panic!("Error matching opcode with title {}.", opcode.title)
        }
    }

    fn get_address(&mut self, mode: &AddressingModes) -> Option<u16> {
        match mode {
            AddressingModes::Immediate => {
                let address = self.program_counter;
                self.inc_program_counter(1);
                Some(address)
            }

            AddressingModes::ZeroPage => {
                let address = self.bus.borrow_mut().read(self.program_counter) as u16;
                self.inc_program_counter(1);
                Some(address)
            }
            
            AddressingModes::ZeroPageX => {
                let base = self.bus.borrow_mut().read(self.program_counter) as u16;
                self.inc_program_counter(1);
                Some(base.wrapping_add(self.register_x as u16) & 0x00FF)
            }

            AddressingModes::ZeroPageY => {
                let base = self.bus.borrow_mut().read(self.program_counter) as u16;
                self.inc_program_counter(1);
                Some(base.wrapping_add(self.register_y as u16) & 0x00FF)
            },

            AddressingModes::Relative => {
                let offset = self.bus.borrow_mut().read(self.program_counter) as i8;
                self.inc_program_counter(1);
                Some((self.program_counter as i16 + offset as i16) as u16)
            }

            AddressingModes::Absolute => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                self.inc_program_counter(2);
                Some((hi << 8) | lo)
            },

            AddressingModes::AbsoluteX => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                self.inc_program_counter(2);
                Some(((hi << 8) | lo).wrapping_add(self.register_x as u16))
            },

            AddressingModes::AbsoluteY => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                self.inc_program_counter(2);
                Some(((hi << 8) | lo).wrapping_add(self.register_y as u16))
            },

            AddressingModes::Indirect => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                self.inc_program_counter(2);
                let address = (hi << 8) | lo;

                // apparently this is a bug in the 6502
                let lo_indirect = self.bus.borrow_mut().read(address) as u16;
                let hi_indirect = self.bus.borrow_mut().read((address & 0xFF00) | ((address + 1) & 0x00FF)) as u16;

                Some((hi_indirect << 8) | lo_indirect)
            }

            AddressingModes::IndirectX => {
                let base = self.bus.borrow_mut().read(self.program_counter);
                self.inc_program_counter(1);
                let address = (base.wrapping_add(self.register_x)) as u16;
                let lo = self.bus.borrow_mut().read(address) as u16;
                let hi = self.bus.borrow_mut().read((address + 1) & 0x00FF) as u16;
                Some((hi << 8) | lo)
            },

            AddressingModes::IndirectY => {
                let base = self.bus.borrow_mut().read(self.program_counter) as u16;
                self.inc_program_counter(1);
                let lo = self.bus.borrow_mut().read(base) as u16;
                let hi = self.bus.borrow_mut().read((base + 1) & 0x00FF) as u16;
                Some(((hi << 8) | lo).wrapping_add(self.register_y as u16))
            },

            _ => None
        }
    }

    fn add_with_carry(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address) + self.processor_status.get_carry_flag();
                let sum = self.register_a as u16 + memory as u16;

                if sum > 0xFF {
                    self.processor_status.set_carry_flag();
                }
                else {
                    self.processor_status.clear_carry_flag();
                }

                let result = self.register_a.wrapping_add(memory);

                if (self.register_a ^ result) & (memory ^ result) & 0b10000000 != 0 {
                    self.processor_status.set_overflow_flag();
                }
                else {
                    self.processor_status.clear_overflow_flag();
                }

                self.register_a = result;

                if self.register_a == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_a & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode ADC.")
        }
    }

    fn logical_and(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);
                
                self.register_a &= memory;

                if self.register_a == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_a & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode AND.")
        }
    }

    fn arithmetic_shift_left(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                if memory & 0b10000000 != 0 {
                    self.processor_status.set_carry_flag();
                }
                else {
                    self.processor_status.clear_carry_flag();
                }

                let result = memory << 1;
                self.bus.borrow_mut().write(address, result);

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => match &opcode.addressing_mode {
                AddressingModes::Accumulator => {
                    if self.register_a & 0b10000000 != 0 {
                        self.processor_status.set_carry_flag();
                    }
                    else {
                        self.processor_status.clear_carry_flag();
                    }

                    self.register_a <<= 1;

                    if self.register_a == 0 {
                        self.processor_status.set_zero_flag();
                    }
                    else {
                        self.processor_status.clear_zero_flag();
                    }

                    if self.register_a & 0b10000000 != 0 {
                        self.processor_status.set_negative_flag();
                    }
                    else {
                        self.processor_status.clear_negative_flag();
                    }
                },

                _ => panic!("Unsupported addressing mode for opcode ASL.")
            }
        }
    }

    fn branch_if_carry_clear(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                if self.processor_status.get_carry_flag() != 0 {
                    return;
                }

                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode BCC.")
        } 
    }

    fn branch_if_carry_set(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                if self.processor_status.get_carry_flag() != 1 {
                    return;
                }

                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode BCS.")
        } 
    }

    fn branch_if_equal(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                if self.processor_status.get_zero_flag() == 0 {
                    return;
                }

                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode BEQ.")
        } 
    }

    fn bit_test(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                if memory & 0b01000000 != 0 {
                    self.processor_status.set_overflow_flag();
                }
                else {
                    self.processor_status.clear_overflow_flag();
                }

                if memory & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }

                let result = self.register_a & memory;

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode BIT.")
        }
    }

    fn branch_if_minus(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                if self.processor_status.get_negative_flag() == 0 {
                    return;
                }

                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode BMI.")
        }
    }

    fn branch_if_not_equal(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                if self.processor_status.get_zero_flag() != 0 {
                    return;
                }

                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode BNE.")
        }
    }

    fn branch_if_positive(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                if self.processor_status.get_negative_flag() != 0 {
                    return;
                }

                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode BPL.")
        }
    }

    fn force_interrupt(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                let hi_pc = ((self.program_counter & 0xFF00) >> 8) as u8;
                let lo_pc = (self.program_counter & 0x00FF) as u8;

                self.stack_push(hi_pc);
                self.stack_push(lo_pc);

                self.stack_push(self.processor_status.status | 0b00010000);

                let hi_interrupt = self.bus.borrow_mut().read(0xFFFF);
                let lo_interrupt = self.bus.borrow_mut().read(0xFFFE);

                let target = ((hi_interrupt as u16) << 8) | lo_interrupt as u16;

                self.program_counter = target;
            },

            _ => panic!("Unsupported addressing mode for opcode BRK.")
        }
    }

    fn branch_if_overflow_clear(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                if self.processor_status.get_overflow_flag() != 0 {
                    return;
                }

                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode BVC.")
        }
    }

    fn branch_if_overflow_set(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                if self.processor_status.get_overflow_flag() == 0 {
                    return;
                }

                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode BVC.")
        }
    }

    fn clear_carry_flag(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.processor_status.clear_carry_flag();
            },

            _ => panic!("Unsupported addressing mode for opcode CLC.")
        }
    }

    fn clear_decimal_mode(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.processor_status.clear_decimal_mode_flag();
            },

            _ => panic!("Unsupported addressing mode for opcode CLD.")
        }
    }

    fn clear_interrupt_disable(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.processor_status.clear_interrupt_disable_flag();
            },

            _ => panic!("Unsupported addressing mode for opcode CLI.")
        }
    }

    fn clear_overflow_flag(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.processor_status.clear_overflow_flag();
            },

            _ => panic!("Unsupported addressing mode for opcode CLV.")
        }
    }

    fn compare(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                let result = self.register_a.wrapping_sub(memory);

                if self.register_a >= memory {
                    self.processor_status.set_carry_flag();
                }
                else {
                    self.processor_status.clear_carry_flag();
                }

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode CMP.")
        }
    }

    fn compare_x_register(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                let result = self.register_x.wrapping_sub(memory);

                if self.register_x >= memory {
                    self.processor_status.set_carry_flag();
                }
                else {
                    self.processor_status.clear_carry_flag();
                }

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode CPX.")
        }
    }

    fn compare_y_register(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                let result = self.register_y.wrapping_sub(memory);

                if self.register_y >= memory {
                    self.processor_status.set_carry_flag();
                }
                else {
                    self.processor_status.clear_carry_flag();
                }

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode CPY.")
        }
    }

    fn decrement_memory(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                let result = memory.wrapping_sub(1);

                self.bus.borrow_mut().write(address, result);

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode DEC.")
        }
    }

    fn decrement_x_register(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_x = self.register_x.wrapping_sub(1);

                if self.register_x == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_x & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode DEX.")
        }
    }

    fn decrement_y_register(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_y = self.register_y.wrapping_sub(1);

                if self.register_y == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_y & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode DEY.")
        }
    }

    fn exclusive_or(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                self.register_a ^= memory;

                if self.register_a == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_a & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode EOR.")
        }
    }

    fn increment_memory(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                let result = memory.wrapping_add(1);

                self.bus.borrow_mut().write(address, result);

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode INC.")
        }
    }

    fn increment_x_register(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_x = self.register_x.wrapping_add(1);

                if self.register_x == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_x & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode INX.")
        }
    }

    fn increment_y_register(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_y = self.register_y.wrapping_add(1);

                if self.register_y == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_y & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode INX.")
        }
    }

    fn jump(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode JMP.")
        }
    }

    fn jump_to_subroutine(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let hi_pc = ((self.program_counter.wrapping_sub(1) & 0xFF00) >> 8) as u8;
                let lo_pc = (self.program_counter.wrapping_sub(1) & 0x00FF) as u8;

                self.stack_push(hi_pc);
                self.stack_push(lo_pc);

                self.program_counter = address;
            },

            None => panic!("Unsupported addressing mode for opcode JSR.")
        }
    }

    fn load_accumulator(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                self.register_a = memory;

                if self.register_a == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_a & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode LDA.")
        }
    }

    fn load_x_register(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                self.register_x = memory;

                if self.register_x == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_x & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode LDX.")
        }
    }

    fn load_y_register(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                self.register_y = memory;

                if self.register_y == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_y & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode LDY.")
        }
    }

    fn logical_shift_right(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                if memory & 0b00000001 != 0 {
                    self.processor_status.set_carry_flag();
                }
                else {
                    self.processor_status.clear_carry_flag();
                }

                let result = memory >> 1;

                self.bus.borrow_mut().write(address, result);

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => match &opcode.addressing_mode {
                AddressingModes::Accumulator => {
                    if self.register_a & 0b00000001 != 0 {
                        self.processor_status.set_carry_flag();
                    }
                    else {
                        self.processor_status.clear_carry_flag();
                    }

                    self.register_a = self.register_a >> 1;

                    if self.register_a == 0 {
                        self.processor_status.set_zero_flag();
                    }
                    else {
                        self.processor_status.clear_zero_flag();
                    }

                    if self.register_a & 0b10000000 != 0 {
                        self.processor_status.set_negative_flag();
                    }
                    else {
                        self.processor_status.clear_negative_flag();
                    }
                },

                _ => panic!("Unsupported addressing mode for opcode LSR.")
            }
        }
    }

    fn no_operation(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                return;
            },

            _ => panic!("Unsupported addressing mode for opcode NOP.")
        }
    }

    fn logical_inclusive_or(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                self.register_a |= memory;

                if self.register_a == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_a & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => panic!("Unsupported addressing mode for opcode ORA.")
        }
    }

    fn push_accumulator(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.stack_push(self.register_a);
            },

            _ => panic!("Unsupported addressing mode for opcode PHA.")
        }
    }

    fn push_processor_status(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.stack_push(self.processor_status.status);
            },

            _ => panic!("Unsupported addressing mode for opcode PHP.")
        }
    }

    fn pull_accumulator(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_a = self.stack_pop();

                if self.register_a == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_a & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode PLA.")
        }
    }

    fn pull_processor_status(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.processor_status.status = self.stack_pop();
            },

            _ => panic!("Unsupported addressing mode for opcode PLP.")
        }
    }

    fn rotate_left(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                let result = (memory << 1) | self.processor_status.get_carry_flag();

                if memory & 0b10000000 != 0 {
                    self.processor_status.set_carry_flag();
                }
                else {
                    self.processor_status.clear_carry_flag();
                }

                self.bus.borrow_mut().write(address, result);

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => match &opcode.addressing_mode {
                AddressingModes::Accumulator => {
                    let result = (self.register_a << 1) | self.processor_status.get_carry_flag();

                    if self.register_a & 0b10000000 != 0 {
                        self.processor_status.set_carry_flag();
                    }
                    else {
                        self.processor_status.clear_carry_flag();
                    }

                    self.register_a = result;

                    if result == 0 {
                        self.processor_status.set_zero_flag();
                    }
                    else {
                        self.processor_status.clear_zero_flag();
                    }

                    if result & 0b10000000 != 0 {
                        self.processor_status.set_negative_flag();
                    }
                    else {
                        self.processor_status.clear_negative_flag();
                    }
                },

                _ => panic!("Unsupported addressing mode for opcode ROL.")
            }
        }
    }

    fn rotate_right(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                let result = (memory >> 1) | (self.processor_status.get_carry_flag() << 7);

                if memory & 0b00000001 != 0 {
                    self.processor_status.set_carry_flag();
                }
                else {
                    self.processor_status.clear_carry_flag();
                }

                self.bus.borrow_mut().write(address, result);

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            None => match &opcode.addressing_mode {
                AddressingModes::Accumulator => {
                    let result = (self.register_a >> 1) | (self.processor_status.get_carry_flag() << 7);
    
                    if self.register_a & 0b00000001 != 0 {
                        self.processor_status.set_carry_flag();
                    }
                    else {
                        self.processor_status.clear_carry_flag();
                    }
    
                    self.register_a = result;
    
                    if result == 0 {
                        self.processor_status.set_zero_flag();
                    }
                    else {
                        self.processor_status.clear_zero_flag();
                    }
    
                    if result & 0b10000000 != 0 {
                        self.processor_status.set_negative_flag();
                    }
                    else {
                        self.processor_status.clear_negative_flag();
                    }
                },

                _ => panic!("Unsupported addressing mode for opcode ROR.")
            }
        }
    }

    fn return_from_interrupt(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                let status = self.stack_pop();

                self.processor_status.status = status;

                let lo_pc = self.stack_pop() as u16;
                let hi_pc = (self.stack_pop() as u16) << 8;

                self.program_counter = hi_pc | lo_pc;
            },

            _ => panic!("Unsupported addressing mode for opcode RTI.")
        }
    }

    fn return_from_subroutine(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                let lo_pc = self.stack_pop() as u16;
                let hi_pc = (self.stack_pop() as u16) << 8;

                self.program_counter = (hi_pc | lo_pc).wrapping_add(1);
            },

            _ => panic!("Unsupported addressing mode for opcode RTS.")
        }
    }

    fn subtract_with_carry(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                let memory = self.bus.borrow_mut().read(address);

                let result = self.register_a.wrapping_sub(memory).wrapping_sub(1 - self.processor_status.get_carry_flag());

                if self.register_a >= memory + (1 - self.processor_status.get_carry_flag()) {
                    self.processor_status.set_carry_flag();
                }
                else {
                    self.processor_status.clear_carry_flag();
                }

                if result == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if result & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }

                if (self.register_a ^ result) & (memory ^ result) & 0b10000000 != 0 {
                    self.processor_status.set_overflow_flag();
                }
                else {
                    self.processor_status.clear_overflow_flag();
                }

                self.register_a = result;
            },

            None => panic!("Unsupported addressing mode for opcode SBC.")
        }
    }

    fn set_carry_flag(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.processor_status.set_carry_flag();
            },

            _ => panic!("Unsupported addressing mode for opcode SEC.")
        }
    }

    fn set_decimal_flag(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.processor_status.set_decimal_mode_flag();
            },

            _ => panic!("Unsupported addressing mode for opcode SED.")
        }
    }

    fn set_interrupt_disable(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.processor_status.set_interrupt_disable_flag();
            },

            _ => panic!("Unsupported addressing mode for opcode SEI.")
        }
    }

    fn store_accumulator(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                self.bus.borrow_mut().write(address, self.register_a);
            },

            None => panic!("Unsupported addressing mode for STA.")
        }
    }

    fn store_x_register(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                self.bus.borrow_mut().write(address, self.register_x);
            },

            None => panic!("Unsupported addressing mode for STX.")
        }
    }

    fn store_y_register(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.addressing_mode) {
            Some(address) => {
                self.bus.borrow_mut().write(address, self.register_y);
            },

            None => panic!("Unsupported addressing mode for STY.")
        }
    }

    fn transfer_accumulator_to_x(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_x = self.register_a;

                if self.register_x == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_x & 0b1000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode TAX.")
        }
    }

    fn transfer_accumulator_to_y(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_y = self.register_a;

                if self.register_y == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_y & 0b1000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode TAY.")
        }
    }

    fn transfer_stack_pointer_to_x(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_x = self.stack_pointer;

                if self.register_x == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_x & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode TSX.")
        }
    }

    fn transfer_x_to_accumulator(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_a = self.register_x;

                if self.register_a == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_a & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode TXA.")
        }
    }

    fn transfer_x_to_stack_pointer(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.stack_pointer = self.register_x;
            },

            _ => panic!("Unsupported addressing mode for opcode TXS.")
        }
    }

    fn transfer_y_to_accumulator(&mut self, opcode: &Opcode) {
        match &opcode.addressing_mode {
            AddressingModes::Implied => {
                self.register_a = self.register_y;

                if self.register_a == 0 {
                    self.processor_status.set_zero_flag();
                }
                else {
                    self.processor_status.clear_zero_flag();
                }

                if self.register_a & 0b10000000 != 0 {
                    self.processor_status.set_negative_flag();
                }
                else {
                    self.processor_status.clear_negative_flag();
                }
            },

            _ => panic!("Unsupported addressing mode for opcode TYA.")
        }
    }
}