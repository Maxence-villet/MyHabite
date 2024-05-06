pub struct DailyQuest {
    name: &str,
    description: &str,
    value: u32,
    completed: bool,
}

impl DailyQuest {
    pub fn new(name: &str, description: &str, value: u32, completed: bool) -> Self {
        Self {  
                name,
                description,
                value,
                completed 
            }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name;
    }

    pub fn get_description(&self) -> &str {
        self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description;
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    }

    pub fn get_completed(&self) -> bool {
        self.completed
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
    
}

