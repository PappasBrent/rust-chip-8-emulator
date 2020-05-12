pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { keys: [false; 16] }
    }
    pub fn key_pressed(&self, index: usize) -> bool {
        true
    }
    pub fn key_down(&mut self, i: usize) {
        self.keys[i] = true;
    }
    pub fn key_up(&mut self, i: usize) {
        self.keys[i] = false;
    }
}
