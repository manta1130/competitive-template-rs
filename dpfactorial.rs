use crate::modint::*;
pub struct DPFactorial {
    normal: Vec<ModInt>,
    inv: Vec<ModInt>,
}
impl DPFactorial {
    #[allow(dead_code)]
    pub fn new(modulus: u32) -> DPFactorial {
        let mut obj = DPFactorial {
            normal: Vec::new(),
            inv: Vec::new(),
        };
        ModInt::set_modulus(modulus);
        obj.normal.push(ModInt::new(1));
        obj.inv.push(ModInt::new(1));
        obj
    }

    #[allow(dead_code)]
    pub fn get_factorial(&mut self, n: usize) -> ModInt {
        if n < self.normal.len() {
            return self.normal[n];
        }
        for z in self.normal.len()..n as usize + 1 {
            let buf = ModInt::new(z);
            let buf = buf * self.normal[z - 1];

            self.normal.push(buf);
        }
        self.normal[n]
    }

    #[allow(dead_code)]
    pub fn get_factorial_inv(&mut self, n: usize) -> ModInt {
        if n < self.inv.len() {
            return self.inv[n];
        }
        for z in self.inv.len()..n + 1 {
            let mut buf = ModInt::new(z);
            buf = buf.inv();
            let buf = buf * self.inv[z - 1];
            self.inv.push(buf);
        }
        self.inv[n]
    }

    #[allow(dead_code)]
    pub fn get_combination(&mut self, n: usize, r: usize) -> ModInt {
        if n < r {
            return ModInt::new(0);
        }
        self.get_factorial(n) * self.get_factorial_inv(n - r) * self.get_factorial_inv(r)
    }

    #[allow(dead_code)]
    pub fn get_permutation(&mut self, n: usize, r: usize) -> ModInt {
        if n < r {
            return ModInt::new(0);
        }
        self.get_factorial(n) * self.get_factorial_inv(n - r)
    }
}
