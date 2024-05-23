struct Fuck {
    pub v: Vec<Vec<i32>>,
}

impl Fuck {
    pub fn change(&mut self) {
        for i in self.v.iter_mut() {
            if i.len() % 2 == 0 {
                let mut new = vec![1i32];
                self.v.push(new);
            };
        }
    }
}
