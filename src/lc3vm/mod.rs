// Rust LC3 virtual machine

// To convert enum to/from u16
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const PROG_START: u16 = 0x3000;
const MEM_SIZE: usize = 1 << 16;
const NUM_REG: usize = 10;

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    RPC,  // program counter
    RCND, // conditional
}

#[derive(Debug)]
#[repr(u8)]
enum Condition {
    POS = 1 << 0,
    ZERO = 1 << 1,
    NEG = 1 << 2,
}

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
enum Opcode {
    BR = 0x0,
    ADD = 0x1,
    LD = 0x2,
    ST = 0x3,
    JSR = 0x4,
    AND = 0x5,
    LDR = 0x6,
    STR = 0x7,
    RTI = 0x8, // not implemented
    NOT = 0x9,
    LDI = 0xA,
    STI = 0xB,
    JMP = 0xC,
    RESERVED = 0xD,
    LEA = 0xE,
    TRAP = 0xF,
}

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
enum TrapVec {
    TGETC = 0x20, // reads a character from console to R0
    TPUTC,        // writes character in R0 to console
    TPUTS,        // print u16 string at R0 until it finds 0x0000
    TIN,          // like TGETC, but echos character to console
    TPUTSP,       // like TPUTS, but prints u8 string (two characters per word)
    THALT,        // halts execution
    TINU16,       // reads a u16 from console to R0
    TOUTU16,      // writes u16 from R0 to console
}

// Returns u8 view of u16 buffer
fn to_u8_slice(slice: &mut [u16]) -> &mut [u8] {
    let byte_len = slice.len() * 2;
    unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast::<u8>(), byte_len) }
}

// The virtual machine state.
pub struct Machine {
    mem: [u16; MEM_SIZE],
    reg: [u16; NUM_REG],
    running: bool,
}

impl Machine {
    // Create initialized machine
    pub fn new() -> Machine {
        Machine {
            mem: [0; MEM_SIZE],
            reg: [0; NUM_REG],
            running: false,
        }
    }

    // Load program from file
    pub fn load(&mut self, filename: &String) {
        let mut f = std::fs::File::open(filename).unwrap_or_else(|error| {
            panic!("failed to open file '{}': {:?}", filename, error);
        });

        println!("Loading '{}' at 0x{:04x}...", filename, PROG_START);
        let progslice = &mut self.mem[(PROG_START as usize)..];
        let progbuf = to_u8_slice(progslice);

        use std::io::Read;
        match f.read(progbuf) {
            Ok(bytes_read) => println!("Loaded {} bytes", bytes_read),
            Err(err) => panic!("failed to read to prog mem: {:?}", err),
        }
    }

    // Run
    pub fn run(&mut self) {
        let rpc = Register::RPC;

        self.regw(rpc, PROG_START);
        self.running = true;

        while self.running {
            self.dump_state();

            // Read instruction at PC, increment PC with wrap-around
            let pc = self.regr(rpc);
            let instr = self.memr(pc);
            self.regw(rpc, pc.overflowing_add(1).0);

            self.dispatch(instr);
        }
    }

    // Dump current machine state
    fn dump_state(&self) {
        // PC and condition register
        let pc = self.regr(Register::RPC);
        let cond = self.regr(Register::RCND);
        print!("pc=0x{:04x} cond=0b{:04b} ", pc, cond);

        // Register file
        let regs = [
            Register::R0,
            Register::R1,
            Register::R2,
            Register::R3,
            Register::R4,
            Register::R5,
            Register::R6,
            Register::R7,
        ];
        for reg in regs {
            print!("{:?}=0x{:04x} ", reg, self.regr(reg));
        }
        println!("");
    }

    // Dispatch instruction
    fn dispatch(&mut self, instr: u16) {
        let opcode = Machine::opcode(instr);
        println!("instr=0x{:04x}, opcode={:?}", instr, opcode);

        match opcode {
            Opcode::ADD => self.opc_add(instr),
            Opcode::AND => self.opc_and(instr),
            Opcode::LD => self.opc_ld(instr),
            Opcode::LDI => self.opc_ldi(instr),
            Opcode::LDR => self.opc_ldr(instr),
            Opcode::LEA => self.opc_lea(instr),
            Opcode::NOT => self.opc_not(instr),
            Opcode::ST => self.opc_st(instr),
            Opcode::STI => self.opc_sti(instr),
            Opcode::STR => self.opc_str(instr),
            Opcode::JMP => self.opc_jmp(instr),
            Opcode::JSR => self.opc_jsr(instr),
            Opcode::BR => self.opc_br(instr),
            Opcode::TRAP => self.opc_trap(instr),
            Opcode::RTI => self.opc_reserved(instr),
            Opcode::RESERVED => self.opc_reserved(instr),
        }
    }

    // Read value from memory
    fn memr(&self, address: u16) -> u16 {
        self.mem[address as usize]
    }

    // Write value to memory
    fn memw(&mut self, address: u16, value: u16) {
        self.mem[address as usize] = value;
    }

    // Read value from register
    fn regr(&self, reg: Register) -> u16 {
        self.reg[reg as usize]
    }

    // Write value to register
    fn regw(&mut self, reg: Register, value: u16) {
        self.reg[reg as usize] = value;
    }

    // Update condition register dependent on value of 'reg'
    fn update_rcnd(&mut self, reg: Register) {
        let value = self.regr(reg);
        if value == 0 {
            self.regw(Register::RCND, Condition::ZERO as u16);
        } else if (value >> 15) != 0 {
            self.regw(Register::RCND, Condition::NEG as u16);
        } else {
            self.regw(Register::RCND, Condition::POS as u16);
        }
    }

    // Returns opcode from instruction
    fn opcode(instr: u16) -> Opcode {
        Opcode::from_u16(instr >> 12).unwrap()
    }

    // Returns destination register from instruction
    fn dstreg(instr: u16) -> Register {
        Register::from_u16((instr >> 9) & 0x7).unwrap()
    }

    // Returns source register 1 from instruction
    fn srcreg1(instr: u16) -> Register {
        Register::from_u16((instr >> 6) & 0x7).unwrap()
    }

    // Returns source register 2 from instruction
    fn srcreg2(instr: u16) -> Register {
        Register::from_u16(instr & 0x7).unwrap()
    }

    // Returns trapvec from instruction
    fn trapvec(instr: u16) -> TrapVec {
        TrapVec::from_u16(instr & 0xff).unwrap()
    }

    // Gets a value indicating whether 'bit' is set in 'value'
    fn is_bit_set(value: u16, bit: u8) -> bool {
        ((value >> bit) & 0x1) != 0
    }

    // Returns sign-extended N-bit immediate from 'value'
    fn sign_extend_immediate(value: u16, bitsize: u8) -> u16 {
        let masked = value & ((1 << bitsize) - 1);
        assert_eq!(value, masked);
        match ((value >> (bitsize - 1)) & 1) != 0 {
            true => (value | (0xffff << bitsize)),
            false => value,
        }
    }

    // add: dr = sr1 + (sr2 | imm5)
    fn opc_add(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let value1 = self.regr(Machine::srcreg1(instr));
        let value2 = match Machine::is_bit_set(instr, 5) {
            true => Machine::sign_extend_immediate(instr & 0x1f, 5),
            false => self.regr(Machine::srcreg2(instr)),
        };

        let value = value1.overflowing_add(value2).0;
        self.regw(dr, value);
        self.update_rcnd(dr);
    }

    // and: dr = sr1 + (sr2 | imm5)
    fn opc_and(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let value1 = self.regr(Machine::srcreg1(instr));
        let value2 = match Machine::is_bit_set(instr, 5) {
            true => Machine::sign_extend_immediate(instr & 0x1f, 5),
            false => self.regr(Machine::srcreg2(instr)),
        };

        let value = value1 & value2;
        self.regw(dr, value);
        self.update_rcnd(dr);
    }

    // ld: dr = [rpc + offset9]
    fn opc_ld(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let offset9 = Machine::sign_extend_immediate(instr & 0x1ff, 9);
        let addr = self.regr(Register::RPC).overflowing_add(offset9).0;

        let value = self.memr(addr);
        self.regw(dr, value);
        self.update_rcnd(dr);
    }

    // ldi: dr = [[rpc + offset9]]
    fn opc_ldi(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let offset9 = Machine::sign_extend_immediate(instr & 0x1ff, 9);
        let addr = self.regr(Register::RPC).overflowing_add(offset9).0;

        let value = self.memr(self.memr(addr));
        self.regw(dr, value);
        self.update_rcnd(dr);
    }

    // ldr: dr = [sr1 + offset6]
    fn opc_ldr(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let sr1 = Machine::srcreg1(instr);
        let offset6 = Machine::sign_extend_immediate(instr & 0x3f, 6);
        let addr = self.regr(sr1).overflowing_add(offset6).0;

        let value = self.memr(addr);
        self.regw(dr, value);
        self.update_rcnd(dr);
    }

    // lea: dr = drpc + offset9
    fn opc_lea(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let offset9 = Machine::sign_extend_immediate(instr & 0x1ff, 9);
        let addr = self.regr(Register::RPC).overflowing_add(offset9).0;

        self.regw(dr, addr);
        self.update_rcnd(dr);
    }

    // not: dr = ~sr1
    fn opc_not(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let sr1 = Machine::srcreg1(instr);

        let value = !self.regr(sr1);
        self.regw(dr, value);
        self.update_rcnd(dr);
    }

    // st: [rpc + offset9] = dr
    fn opc_st(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let offset9 = Machine::sign_extend_immediate(instr & 0x1ff, 9);
        let addr = self.regr(Register::RPC).overflowing_add(offset9).0;

        let value = self.regr(dr);
        self.memw(addr, value);
    }

    // sti: [[rpc + offset9]] = dr
    fn opc_sti(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let offset9 = Machine::sign_extend_immediate(instr & 0x1ff, 9);
        let addr = self.regr(Register::RPC).overflowing_add(offset9).0;

        let value = self.regr(dr);
        let addri = self.memr(addr);
        self.memw(addri, value);
    }

    // str: [sr1 + offset6] = dr
    fn opc_str(&mut self, instr: u16) {
        let dr = Machine::dstreg(instr);
        let sr1 = Machine::srcreg1(instr);
        let offset6 = Machine::sign_extend_immediate(instr & 0x3f, 6);
        let addr = self.regr(sr1).overflowing_add(offset6).0;

        let value = self.regr(dr);
        self.memw(addr, value);
    }

    // jmp: rpc = sr1
    fn opc_jmp(&mut self, instr: u16) {
        let sr1 = Machine::srcreg1(instr);

        let addr = self.regr(sr1);
        self.regw(Register::RPC, addr);
    }

    // jsr: r7 = rpc, rpc = sr1 | (rpc + offset11)
    fn opc_jsr(&mut self, instr: u16) {
        // Store return address in R7
        let pc = self.regr(Register::RPC);
        self.regw(Register::R7, pc);

        let addr = match Machine::is_bit_set(instr, 11) {
            true => Machine::sign_extend_immediate(instr & 0x7ff, 11),
            false => self.regr(Machine::srcreg1(instr)),
        };
        self.regw(Register::RPC, addr);
    }

    // br: rpc = rpc + offset9 iff condition is met
    fn opc_br(&mut self, instr: u16) {
        let branch_cond = (instr >> 9) & 0x7;
        let offset9 = Machine::sign_extend_immediate(instr & 0x1ff, 9);

        let cond = self.regr(Register::RCND);
        if (cond & branch_cond) != 0 {
            let rpc = Register::RPC;
            let addr = self.regr(rpc).overflowing_add(offset9).0;
            self.regw(rpc, addr);
        }
    }

    // trap:
    fn opc_trap(&mut self, instr: u16) {
        let trapvec = Machine::trapvec(instr);
        match trapvec {
            TrapVec::TGETC => self.trap_tgetc(),
            TrapVec::TPUTC => self.trap_tputc(),
            TrapVec::TPUTS => self.trap_tputs(),
            TrapVec::THALT => self.trap_thalt(),
            TrapVec::TINU16 => self.trap_tinu16(),
            TrapVec::TOUTU16 => self.trap_toutu16(),
            _ => self.trap_reserved(trapvec),
        }
    }

    // opcode reserved/not implemented:
    fn opc_reserved(&mut self, instr: u16) {
        println!("opcode reserved/not implemented: {:02x}", instr);
    }

    // tgetc: read character to R0
    fn trap_tgetc(&mut self) {
        use std::io::Write;
        print!("Enter char: ");
        std::io::stdout().flush().unwrap();

        // Read char from user
        let mut value = [0];

        use std::io::Read;
        std::io::stdin()
            .read(&mut value)
            .expect("failed to read u8");

        self.regw(Register::R0, value[0] as u16);
    }

    // tputc: write character from R0
    fn trap_tputc(&self) {
        let value = self.regr(Register::R0) as u8;
        println!("0x{:02x}", value);
    }

    // tputs: prints u16 string at R0, until it finds 0x0000
    fn trap_tputs(&self) {
        let mut addr = self.regr(Register::R0);
        loop {
            let value = self.memr(addr);
            if value == 0x0000 {
                break;
            }

            print!("{}", char::from_u32(value as u32).unwrap());
            addr += 1;
        }
        println!("");
    }

    // thalt: halt machine
    fn trap_thalt(&mut self) {
        println!("THALT");
        self.running = false;
    }

    // tinu16: read U16 to R0
    fn trap_tinu16(&mut self) {
        use std::io::Write;
        print!("Enter u16: 0x");
        std::io::stdout().flush().unwrap();

        // Read line from user
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .expect("failed to read u16 string");

        match u16::from_str_radix(&s.replace("\n", ""), 16) {
            Ok(value) => self.regw(Register::R0, value),
            Err(err) => println!("failed to parse '{}': {:?}", s, err),
        }
    }

    // toutu16: write U16 from R0
    fn trap_toutu16(&self) {
        let value = self.regr(Register::R0);
        println!("0x{:04x}", value);
    }

    // trap reserved/not implemented
    fn trap_reserved(&mut self, trapvec: TrapVec) {
        println!("trapvec reserved/not implemented: {:?}", trapvec);
    }
}
