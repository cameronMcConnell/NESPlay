use super::bus;
use super::opcode;
use std::rc::Rc;
use std::cell::RefCell;

type Bus = bus::Bus;

pub struct Cpu {
    program_counter: u16,
    stack_pointer: u8,
    register_a: u8,
    register_x: u8,
    register_y: u8,
    processor_status: u8,
    bus: Rc<RefCell<Bus>>,
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Cpu {
            program_counter: 0,
            stack_pointer: 0xFF,
            register_a: 0,
            register_x: 0,
            register_y: 0,
            processor_status: 0,
            bus,
        }
    }

    pub fn inc_program_counter(&mut self) {
        self.program_counter += 1;
    }

    pub fn set_program_counter(&mut self, address: u16) {
        self.program_counter = address;
    }

    pub fn dec_stack_pointer(&mut self) {
        self.stack_pointer -= 1;
    }

    fn match_opcode(&mut self) {
        // let byte = self.bus.read_cpu_ram(self.program_counter as usize);
        // self.inc_program_counter();

        // match on bytes here for opcode in CPU_OP_CODES
    }

    pub fn execute_opcode(&mut self) {
        // This is wrong and I am dumb
        let opcode = &opcode::CPU_OP_CODES[self.program_counter as usize];
        self.inc_program_counter();

        match opcode.title() {
            "ADC" => {

            }
            "AND" => {

            }
            "ASL" => {

            }
            "BCC" => {

            }
            "BCS" => {

            }
            "BEQ" => {

            }
            "BIT" => {

            }
            "BMI" => {

            }
            "BNE" => {

            }
            "BPL" => {

            }
            "BRK" => {

            }
            "BVC" => {

            }
            "BVS" => {

            }
            "CLC" => {

            }
            "CLD" => {

            }
            "CLI" => {

            }
            "CLV" => {

            }
            "CMP" => {

            }
            "CPX" => {

            }
            "CPY" => {

            }
            "DEC" => {

            }
            "DEX" => {

            }
            "DEY" => {

            }
            "EOR" => {

            }
            "INC" => {

            }
            "INX" => {
                
            }
            "INY" => {

            }
            "JMP" => {

            }
            "JSR" => {

            }
            "LDA" => {

            }
            "LDX" => {

            }
            "LDY" =>{

            }
            "LSR" => {

            }
            "NOP" => {

            }
            "ORA" => {

            }
            "PHA" => {

            }
            "PHP" => {

            }
            "PLA" => {

            }
            "PLP" => {

            }
            "ROL" => {

            }
            "ROR" => {

            }
            "RTI" => {

            }
            "RTS" => {

            }
            "SBC" => {

            }
            "SEC" => {

            }
            "SED" => {

            }
            "SEI" => {

            }
            "STA" => {

            }
            "STX" => {

            }
            "STY" => {

            }
            "TAX" => {

            }
            "TAY" => {

            }
            "TSX" => {

            }
            "TXA" => {

            }
            "TXS" => {

            }
            "TYA" => {
                
            }
            _ => println!("Error matching opcode with title {}.", opcode.title())
        }
    }
}