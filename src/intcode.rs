use std::collections::HashMap;

pub struct IntCode {
    mem: Vec<i64>,
    ext_mem: HashMap<usize, i64>,
    pc: usize, 
    base: i64,
    input_buf: i64,
    input_available: bool,
    blocked_on_input: bool,
    output_buf: i64,
    output_available: bool,
    finished: bool,
    debug: bool
}

impl IntCode {
    const INST_ADD: i64 = 1;
    const INST_MUL: i64 = 2;
    const INST_INPUT: i64 = 3;
    const INST_OUTPUT: i64 = 4;
    const INST_JUMP_IF_TRUE: i64 = 5;
    const INST_JUMP_IF_FALSE: i64 = 6;
    const INST_LESS_THAN: i64 = 7;
    const INST_EQUALS: i64 = 8;
    const INST_ADJUST_BASE: i64 = 9;
    const INST_HALT: i64 = 99;

    pub fn new( memsize: usize ) -> Self {
        IntCode {
            mem: vec![0; memsize],
            ext_mem: HashMap::new(),
            pc: 0,
            base: 0,
            input_buf: 0,
            input_available: false,
            blocked_on_input: false,
            output_buf: 0,
            output_available: false,
            finished: false,
            debug: false
        }
    }

    pub fn init(&mut self, data: Vec<i64>) {
        for (a, v) in data.iter().enumerate() {
            self.mem.insert(a, *v);
        }
    }

    pub fn set_input(&mut self, input: i64) {
        self.input_buf = input;
        self.input_available = true;
        self.blocked_on_input = false;
    }

    pub fn blocked_on_input(self) -> bool {
        self.blocked_on_input
    }

    pub fn get_output(&mut self) -> Option<i64> {
        if self.output_available {
            self.output_available = false;
            return Some(self.output_buf);
        }
        None
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn tick(&mut self) {
        if self.finished {
            return;
        }
        let inst = self.mem[self.pc] % 100;
        let mode_a = self.mem[self.pc] / 100 % 10;
        let mode_b = self.mem[self.pc] / 1000 % 10;
        let mode_c = self.mem[self.pc] / 10000 % 10;
        if self.debug {
            print!("{}: ", self.pc);
        }
        match inst {
            IntCode::INST_ADD => self.add(mode_a, mode_b, mode_c),
            IntCode::INST_MUL => self.mul(mode_a, mode_b, mode_c),
            IntCode::INST_INPUT => self.input(mode_a),
            IntCode::INST_OUTPUT => self.output(mode_a),
            IntCode::INST_JUMP_IF_TRUE => self.jump_if_true(mode_a, mode_b),
            IntCode::INST_JUMP_IF_FALSE => self.jump_if_false(mode_a, mode_b),
            IntCode::INST_LESS_THAN => self.less_than(mode_a, mode_b, mode_c),
            IntCode::INST_EQUALS => self.equals(mode_a, mode_b, mode_c),
            IntCode::INST_ADJUST_BASE => self.adjust_base(mode_a),
            IntCode::INST_HALT => self.finished = true,
            _ => println!("Invalid Instruction {} at PC = {}", inst, self.pc)
        }
    }

    pub fn run(&mut self) {
        while !self.finished && !self.blocked_on_input {
            self.tick();
        }
    }

    pub fn run_until_output(&mut self) {
        while !self.finished && !self.blocked_on_input && !self.output_available {
            self.tick();
        }        
    }

    pub fn get_mem(&mut self, addr: usize) -> i64 {
        if addr < self.mem.len() {
            return self.mem[addr];
        }
        if self.ext_mem.contains_key(&addr) {
            return self.ext_mem[&addr];
        }
        self.ext_mem.insert(addr, 0);
        0
    }

    pub fn set_mem(&mut self, addr: usize, val: i64) {
        if addr < self.mem.len() {
            self.mem[addr] = val;
        } else {
            self.ext_mem.insert(addr, val);
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    fn mode_char(&self, mode: i64) -> char {
        match mode {
            0 => 'P',
            1 => 'A',
            2 => 'R',
            _ => '?'
        }
    }

    fn get_val(&mut self, addr: usize, mode: i64) -> i64 {
        if mode == 0 {
            let a = self.get_mem(addr) as usize;
            return self.get_mem(a);
        } else if mode == 2 {
            let tmp = self.get_mem(addr) as i64;
            let a = (tmp + self.base) as usize;
            return self.get_mem(a);
        }
        self.mem[addr]
    }

    fn set_val(&mut self, addr: usize, mode: i64, val: i64) {
        let a;
        if mode == 0 {
            a = self.get_mem(addr) as usize;
        } else {
            let tmp = self.get_mem(addr) as i64;
            a = (tmp + self.base) as usize;
        }
        self.set_mem(a, val);
    }

    fn add(&mut self, mode_a: i64, mode_b: i64, mode_c: i64) {
        let val_a = self.get_val(self.pc + 1, mode_a);
        let val_b = self.get_val(self.pc + 2, mode_b);
        if self.debug {
            println!("ADD {}[{}]({}) {}[{}]({}) {}[{}]",
                    self.get_mem(self.pc + 1), self.mode_char(mode_a), val_a,
                    self.get_mem(self.pc + 2), self.mode_char(mode_b), val_b,
                    self.get_mem(self.pc + 3), self.mode_char(mode_c));
        }
        let sum = val_a + val_b;
        self.set_val(self.pc + 3, mode_c, sum);
        self.pc += 4;    
    }

    fn mul(&mut self, mode_a: i64, mode_b: i64, mode_c: i64) {
        let val_a = self.get_val(self.pc + 1, mode_a);
        let val_b = self.get_val(self.pc + 2, mode_b);
        if self.debug {
            println!("MUL {}[{}]({}) {}[{}]({}) {}[{}]",
                    self.get_mem(self.pc + 1), self.mode_char(mode_a), val_a,
                    self.get_mem(self.pc + 2), self.mode_char(mode_b), val_b,
                    self.get_mem(self.pc + 3), self.mode_char(mode_c));
        }
        let prod = val_a * val_b;
        self.set_val(self.pc + 3, mode_c, prod);
        self.pc += 4;    
    }

    fn input(&mut self, mode: i64) {
        if self.debug {
            println!("INP {}[{}]",
                    self.get_mem(self.pc + 1), self.mode_char(mode));
        }
        if !self.input_available {
            self.blocked_on_input = true;
        } else {
            self.set_val(self.pc + 1, mode, self.input_buf);
            self.input_available = false;
            self.pc += 2;
        }
    }

    fn output(&mut self, mode: i64) {
        let val = self.get_val(self.pc + 1, mode);
        if self.debug {
            println!("OUT {}[{}]({})",
                    self.get_mem(self.pc + 1), self.mode_char(mode), val);
        }
        self.output_buf = val;
        self.output_available = true;
        self.pc += 2;
    }

    fn jump_if_true(&mut self, mode_a: i64, mode_b: i64) {
        let test_val = self.get_val(self.pc + 1, mode_a);
        if self.debug {
            println!("JIT {}[{}]({}) {}[{}]",
                    self.get_mem(self.pc + 1), self.mode_char(mode_a), test_val,
                    self.get_mem(self.pc + 2), self.mode_char(mode_b));
        }
        let dest = self.get_val(self.pc + 2, mode_b);
        if test_val != 0 {
            self.pc = dest as usize;
        } else {
            self.pc += 3;
        }
    }

    fn jump_if_false(&mut self, mode_a: i64, mode_b: i64) {
        let test_val = self.get_val(self.pc + 1, mode_a);
        if self.debug {
            println!("JIF {}[{}]({}) {}[{}]",
                    self.get_mem(self.pc + 1), self.mode_char(mode_a), test_val,
                    self.get_mem(self.pc + 2), self.mode_char(mode_b));
        }
        let dest = self.get_val(self.pc + 2, mode_b);
        if test_val == 0 {
            self.pc = dest as usize;
        } else {
            self.pc += 3;
        }
    }

    fn less_than(&mut self, mode_a: i64, mode_b: i64, mode_c: i64) {
        let val_a = self.get_val(self.pc + 1, mode_a);
        let val_b = self.get_val(self.pc + 2, mode_b);
        if self.debug {
            println!("LT  {}[{}]({}) {}[{}]({}) {}[{}]",
                    self.get_mem(self.pc + 1), self.mode_char(mode_a), val_a,
                    self.get_mem(self.pc + 2), self.mode_char(mode_b), val_b,
                    self.get_mem(self.pc + 3), self.mode_char(mode_c));
        }
        let result;
        if val_a < val_b {
            result = 1;
        } else {
            result = 0;
        }
        self.set_val(self.pc + 3, mode_c, result);
        self.pc += 4;
    }

    fn equals(&mut self, mode_a: i64, mode_b: i64, mode_c: i64) {
        let val_a = self.get_val(self.pc + 1, mode_a);
        let val_b = self.get_val(self.pc + 2, mode_b);
        if self.debug {
            println!("EQ  {}[{}]({}) {}[{}]({}) {}[{}]",
                    self.get_mem(self.pc + 1), self.mode_char(mode_a), val_a,
                    self.get_mem(self.pc + 2), self.mode_char(mode_b), val_b,
                    self.get_mem(self.pc + 3), self.mode_char(mode_c));
        }
        let result;
        if val_a == val_b {
            result = 1;
        } else {
            result = 0;
        }
        self.set_val(self.pc + 3, mode_c, result);
        self.pc += 4;
    }

    fn adjust_base(&mut self, mode: i64) {
        let val = self.get_val(self.pc + 1, mode);
        if self.debug {
            println!("BAS {}[{}]({})",
                    self.get_mem(self.pc + 1), self.mode_char(mode), val);
        }
        self.base += val;
        self.pc += 2;
    }

}