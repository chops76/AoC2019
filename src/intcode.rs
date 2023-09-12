pub struct IntCode {
    mem: Vec<i64>,
    pc: usize, 
    finished: bool
}

impl IntCode {
    pub fn new( memsize: usize ) -> Self {
        IntCode {
            mem: vec![0; memsize],
            pc: 0,
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

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn tick(&mut self) {
        if self.finished {
            return;
        }
        if self.mem[self.pc] == 1 {
            let addr1 = self.mem[self.pc + 1] as usize;
            let addr2 = self.mem[self.pc + 2] as usize;
            let dest_addr = self.mem[self.pc + 3] as usize;
            let sum = self.mem[addr1] + self.mem[addr2];
            self.mem[dest_addr] = sum;
            self.pc += 4;
        } else if self.mem[self.pc] == 2 {
            let addr1 = self.mem[self.pc + 1] as usize;
            let addr2 = self.mem[self.pc + 2] as usize;
            let dest_addr = self.mem[self.pc + 3] as usize;
            let prod = self.mem[addr1] * self.mem[addr2];
            self.mem[dest_addr] = prod;
            self.pc += 4;
        } else if self.mem[self.pc] == 99 {
            self.finished = true;
        }
    }

    pub fn run(&mut self) {
        while !self.finished {
            self.tick();
        }
    }
}