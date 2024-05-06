use rand::Rng;
pub struct Summon {
    gem_needed: u32,
}

impl Summon {
    pub fn new(gem_needed: u32) -> Self {
        Self { gem_needed }
    }

    pub fn get_gem_needed(&self) -> u32 {
        self.gem_needed
    }

    pub fn set_gem_needed(&mut self, gem_needed: u32) {
        self.gem_needed = gem_needed
    }

    pub fn generate_number(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..99)
    }
}