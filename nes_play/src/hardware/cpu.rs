use super::bus;
use super::opcode;
use std::rc::Rc;
use std::cell::RefCell;

type Bus = bus::Bus;
type Opcode = opcode::Opcode;
type AddressingModes = opcode::AddressingModes;

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
    pub carry_flag: bool,
    pub zero_flag: bool,
    pub interrupt_disable_flag: bool,
    pub decimal_mode_flag: bool,
    pub break_command: bool,
    pub overflow_flag: bool,
    pub negative_flag: bool,
}

impl ProcessStatus {
    pub fn new() -> Self {
        ProcessStatus {
            carry_flag: false,
            zero_flag: false,
            interrupt_disable_flag: false,
            decimal_mode_flag: false,
            break_command: false,
            overflow_flag: false,
            negative_flag: false,
        }
    }

    pub fn set_carry_flag(&mut self) {
        self.carry_flag = true;
    }

    pub fn set_zero_flag(&mut self) {
        self.zero_flag = true;
    }

    pub fn set_interrupt_disable_flag(&mut self) {
        self.interrupt_disable_flag = true;
    }

    pub fn set_decimal_mode_flag(&mut self) {
        self.decimal_mode_flag = true;
    }

    pub fn set_break_command(&mut self) {
        self.break_command = true;
    }

    pub fn set_overflow_flag(&mut self) {
        self.overflow_flag = true;
    }

    pub fn set_negative_flag(&mut self) {
        self.negative_flag = true;
    }

    pub fn clear_carry_flag(&mut self) {
        self.carry_flag = false;
    }

    pub fn clear_set_zero_flag(&mut self) {
        self.zero_flag = false;
    }

    pub fn clear_interrupt_disable_flag(&mut self) {
        self.interrupt_disable_flag = false;
    }

    pub fn clear_decimal_mode_flag(&mut self) {
        self.decimal_mode_flag = false;
    }

    pub fn clear_break_command(&mut self) {
        self.break_command = false;
    }

    pub fn clear_overflow_flag(&mut self) {
        self.overflow_flag = false;
    }

    pub fn clear_negative_flag(&mut self) {
        self.negative_flag = false;
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

    fn inc_program_counter(&mut self) {
        self.program_counter += 1;
    }

    pub fn set_program_counter(&mut self, address: u16) {
        self.program_counter = address;
    }

    fn stack_push(&mut self) {
    
    }

    fn stack_pop(&mut self) {

    }

    pub fn execute_opcode(&mut self) {
        let code = self.bus.borrow_mut().read(self.program_counter);
        self.inc_program_counter();

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
            _ => println!("Error matching opcode with title {}.", opcode.title)
        }
    }

    fn get_address(&mut self, mode: &AddressingModes) -> Option<u16> {
        match mode {
            AddressingModes::Implied => None,
            
            AddressingModes::Accumulator => None,
            
            AddressingModes::Immediate => Some(self.bus.borrow_mut().read(self.program_counter) as u16),
            
            AddressingModes::ZeroPage => Some(self.bus.borrow_mut().read(self.program_counter) as u16),
            
            AddressingModes::ZeroPageX => {
                let base = self.bus.borrow_mut().read(self.program_counter) as u16;
                Some(base + self.register_x as u16)
            }

            AddressingModes::ZeroPageY => {
                let base = self.bus.borrow_mut().read(self.program_counter) as u16;
                Some(base + self.register_y as u16)
            },

            AddressingModes::Relative => Some(self.bus.borrow_mut().read(self.program_counter) as u16),

            AddressingModes::Absolute => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                Some((hi << 8) | lo)
            },

            AddressingModes::AbsoluteX => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                Some(((hi << 8) | lo) + self.register_x as u16)
            },

            AddressingModes::AbsoluteY => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                Some(((hi << 8) | lo) + self.register_y as u16)
            },

            AddressingModes::Indirect => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                let address = (hi << 8) | lo;
                Some(self.bus.borrow_mut().read(address) as u16)
            }

            AddressingModes::IndirectX => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                let address = ((hi << 8) | lo) + self.register_x as u16;
                Some(self.bus.borrow_mut().read(address) as u16)
            },

            AddressingModes::IndirectY => {
                let lo = self.bus.borrow_mut().read(self.program_counter) as u16;
                let hi = self.bus.borrow_mut().read(self.program_counter + 1) as u16;
                let address = (hi << 8) | lo;
                Some(self.bus.borrow_mut().read(address) as u16 + self.register_y as u16)
            }
        }
    }

    fn add_with_carry(&mut self, opcode: &Opcode) {
        match self.get_address(&opcode.adressing_mode) {
            Some(address) => {
                
                }
            },
            None => panic!("Unsupported addressing mode for opcode ADC.")
        }
    }

    fn logical_and(&mut self, opcode: &Opcode) {

    }

    fn arithmetic_shift_left(&mut self, opcode: &Opcode) {

    }

    fn branch_if_carry_clear(&mut self, opcode: &Opcode) {
        
    }

    fn branch_if_carry_set (&mut self, opcode: &Opcode) {
        
    }

    fn branch_if_equal(&mut self, opcode: &Opcode) {
        
    }

    fn bit_test(&mut self, opcode: &Opcode) {
        
    }

    fn branch_if_minus(&mut self, opcode: &Opcode) {
        
    }

    fn branch_if_not_equal(&mut self, opcode: &Opcode) {
        
    }

    fn branch_if_positive(&mut self, opcode: &Opcode) {
        
    }

    fn force_interrupt(&mut self, opcode: &Opcode) {
        
    }

    fn branch_if_overflow_clear(&mut self, opcode: &Opcode) {
        
    }

    fn branch_if_overflow_set(&mut self, opcode: &Opcode) {
        
    }

    fn clear_carry_flag(&mut self, opcode: &Opcode) {
        
    }

    fn clear_decimal_mode(&mut self, opcode: &Opcode) {
        
    }

    fn clear_interrupt_disable(&mut self, opcode: &Opcode) {
        
    }

    fn clear_overflow_flag(&mut self, opcode: &Opcode) {
        
    }

    fn compare(&mut self, opcode: &Opcode) {
        
    }

    fn compare_x_register(&mut self, opcode: &Opcode) {
        
    }

    fn compare_y_register(&mut self, opcode: &Opcode) {
        
    }

    fn decrement_memory(&mut self, opcode: &Opcode) {
        
    }

    fn decrement_x_register(&mut self, opcode: &Opcode) {
        
    }

    fn decrement_y_register(&mut self, opcode: &Opcode) {
        
    }

    fn exclusive_or(&mut self, opcode: &Opcode) {
        
    }

    fn increment_memory(&mut self, opcode: &Opcode) {
        
    }

    fn increment_x_register(&mut self, opcode: &Opcode) {
        
    }

    fn increment_y_register(&mut self, opcode: &Opcode) {
        
    }

    fn jump(&mut self, opcode: &Opcode) {
        
    }

    fn jump_to_subroutine(&mut self, opcode: &Opcode) {
        
    }

    fn load_accumulator(&mut self, opcode: &Opcode) {
        
    }

    fn load_x_register(&mut self, opcode: &Opcode) {
        
    }

    fn load_y_register(&mut self, opcode: &Opcode) {
        
    }

    fn logical_shift_right(&mut self, opcode: &Opcode) {
        
    }

    fn no_operation(&mut self, opcode: &Opcode) {
        
    }

    fn logical_inclusive_or(&mut self, opcode: &Opcode) {
        
    }

    fn push_accumulator(&mut self, opcode: &Opcode) {
        
    }

    fn push_processor_status(&mut self, opcode: &Opcode) {
        
    }

    fn pull_accumulator(&mut self, opcode: &Opcode) {
        
    }

    fn pull_processor_status(&mut self, opcode: &Opcode) {
        
    }

    fn rotate_left(&mut self, opcode: &Opcode) {
        
    }

    fn rotate_right(&mut self, opcode: &Opcode) {
        
    }

    fn return_from_interrupt(&mut self, opcode: &Opcode) {
        
    }

    fn return_from_subroutine(&mut self, opcode: &Opcode) {
        
    }

    fn subtract_with_carry(&mut self, opcode: &Opcode) {
        
    }

    fn set_carry_flag(&mut self, opcode: &Opcode) {
        
    }

    fn set_decimal_flag(&mut self, opcode: &Opcode) {
        
    }

    fn set_interrupt_disable(&mut self, opcode: &Opcode) {
        
    }

    fn store_accumulator(&mut self, opcode: &Opcode) {
        
    }

    fn store_x_register(&mut self, opcode: &Opcode) {
        
    }

    fn store_y_register(&mut self, opcode: &Opcode) {
        
    }

    fn transfer_accumulator_to_x(&mut self, opcode: &Opcode) {
        
    }

    fn transfer_accumulator_to_y(&mut self, opcode: &Opcode) {
        
    }

    fn transfer_stack_pointer_to_x(&mut self, opcode: &Opcode) {
        
    }

    fn transfer_x_to_accumulator(&mut self, opcode: &Opcode) {
        
    }

    fn transfer_x_to_stack_pointer(&mut self, opcode: &Opcode) {
        
    }

    fn transfer_y_to_accumulator(&mut self, opcode: &Opcode) {
        
    }
}