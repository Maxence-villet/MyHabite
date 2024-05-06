pub struct Player {
    gem: u32,
    wallet: f64,
}

impl Player {
    pub fn new() -> Self {
        Self {
            gem: 0,
            wallet: 0.0,
        }
    }
    
    pub fn get_gem(&self) -> u32 {
        self.gem
    }

    pub fn get_wallet(&self) -> f64 {
        self.wallet
    }

    pub fn add_gem(&mut self, gem: u32) {
        self.gem += gem;
    }

    pub fn add_wallet(&mut self, wallet: f64) {
        self.wallet += wallet;
    }

    pub fn remove_gem(&mut self, gem: u32) {
        self.gem -= gem;
    }

    pub fn remove_wallet(&mut self, wallet: f64) {
        self.wallet -= wallet;
    }
}
