pub struct IntCode {
    mem: Vec<i64>,
    pc: usize, 
    input_buf: i64,
    input_available: bool,
    blocked_on_input: bool,
    output_buf: i64,
    output_available: bool,
    finished: bool
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
    const INST_HALT: i64 = 99;

    pub fn new( memsize: usize ) -> Self {
        IntCode {
            mem: vec![0; memsize],
            pc: 0,
            input_buf: 0,
            input_available: false,
            blocked_on_input: false,
            output_buf: 0,
            output_available: false,
            finished: false
        }
    }

    pub fn init(&mut self, data: Vec<i64>) {
        for (a, v) in data.iter().enumerate() {
            self.mem.insert(a, *v);
        }
    }

    pub fn set_mem(&mut self, addr: usize, val: i64) {
        self.mem.insert(addr, val);
    }

    pub fn get_mem(&mut self, addr: usize) -> i64 {
        self.mem[addr]
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
        match inst {
            IntCode::INST_ADD => self.add(mode_a, mode_b),
            IntCode::INST_MUL => self.mul(mode_a, mode_b),
            IntCode::INST_INPUT => self.input(),
            IntCode::INST_OUTPUT => self.output(mode_a),
            IntCode::INST_JUMP_IF_TRUE => self.jump_if_true(mode_a, mode_b),
            IntCode::INST_JUMP_IF_FALSE => self.jump_if_false(mode_a, mode_b),
            IntCode::INST_LESS_THAN => self.less_than(mode_a, mode_b),
            IntCode::INST_EQUALS => self.equals(mode_a, mode_b),
            IntCode::INST_HALT => self.finished = true,
            _ => println!("Invalid Instruction {} at PC = {}", inst, self.pc)
        }
    }

    pub fn run(&mut self) {
        while !self.finished {
            self.tick();
        }
    }

    fn get_val(&mut self, addr: usize, mode: i64) -> i64 {
        if mode == 0 {
            let a = self.mem[addr] as usize;
            return self.mem[a];
        }
        self.mem[addr]
    }

    fn add(&mut self, mode_a: i64, mode_b: i64) {
        let val_a = self.get_val(self.pc + 1, mode_a);
        let val_b = self.get_val(self.pc + 2, mode_b);
        let dest_addr = self.mem[self.pc + 3] as usize;
        let sum = val_a + val_b;
        self.mem[dest_addr] = sum;
        self.pc += 4;    
    }

    fn mul(&mut self, mode_a: i64, mode_b: i64) {
        let val_a = self.get_val(self.pc + 1, mode_a);
        let val_b = self.get_val(self.pc + 2, mode_b);
        let dest_addr = self.mem[self.pc + 3] as usize;
        let sum = val_a * val_b;
        self.mem[dest_addr] = sum;
        self.pc += 4;    
    }

    fn input(&mut self) {
        if !self.input_available {
            self.blocked_on_input = true;
        } else {
            let dest_addr = self.mem[self.pc + 1] as usize;
            self.mem[dest_addr] = self.input_buf;
            self.input_available = false;
            self.pc += 2;
        }
    }

    fn output(&mut self, mode: i64) {
        let val = self.get_val(self.pc + 1, mode);
        self.output_buf = val;
        self.output_available = true;
        self.pc += 2;
    }

    fn jump_if_true(&mut self, mode_a: i64, mode_b: i64) {
        let test_val = self.get_val(self.pc + 1, mode_a);
        let dest = self.get_val(self.pc + 2, mode_b);
        if test_val != 0 {
            self.pc = dest as usize;
        } else {
            self.pc += 3;
        }
    }

    fn jump_if_false(&mut self, mode_a: i64, mode_b: i64) {
        let test_val = self.get_val(self.pc + 1, mode_a);
        let dest = self.get_val(self.pc + 2, mode_b);
        if test_val == 0 {
            self.pc = dest as usize;
        } else {
            self.pc += 3;
        }
    }

    fn less_than(&mut self, mode_a: i64, mode_b: i64) {
        let val_a = self.get_val(self.pc + 1, mode_a);
        let val_b = self.get_val(self.pc + 2, mode_b);
        let dest_addr = self.mem[self.pc + 3] as usize;
        if val_a < val_b {
            self.mem[dest_addr] = 1;
        } else {
            self.mem[dest_addr] = 0;
        }
        self.pc += 4;
    }

    fn equals(&mut self, mode_a: i64, mode_b: i64) {
        let val_a = self.get_val(self.pc + 1, mode_a);
        let val_b = self.get_val(self.pc + 2, mode_b);
        let dest_addr = self.mem[self.pc + 3] as usize;
        if val_a == val_b {
            self.mem[dest_addr] = 1;
        } else {
            self.mem[dest_addr] = 0;
        }
        self.pc += 4;
    }

}