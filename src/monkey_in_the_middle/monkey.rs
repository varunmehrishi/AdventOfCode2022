pub struct Monkey {
    pub index: usize,
    pub inspections: u64,
    pub items: Vec<u64>,
    pub op: Box<dyn Fn(u64) -> u64>,
    pub div: u64,
    pub ti: usize,
    pub fi: usize,
}

impl Monkey {
    pub fn inspect_items(&mut self) {
        let f = &self.op;
        self.items.iter_mut().for_each(|i| *i = f(*i));
    }

    pub fn inspect_items_2(&mut self, div: u64) {
        let f = &self.op;
        self.items.iter_mut().for_each(|i| *i = f(*i) % div);
    }

    pub fn decrease_worry(&mut self) {
        self.items.iter_mut().for_each(|i| *i /= 3);
    }

    pub fn test(&mut self) -> (Vec<u64>, Vec<u64>) {
        let mut t = vec![];
        let mut f = vec![];
        for &i in &self.items {
            self.inspections += 1;
            if i % self.div == 0 {
                t.push(i);
            } else {
                f.push(i)
            }
        }

        self.items.clear();

        (t, f)
    }
}
