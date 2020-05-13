const WIDTH: usize = 64;
const HEIGHT: usize = 32;
pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
}

pub fn byte_index(byte: u8, index: usize) -> u8 {
    (byte & (1 << index)) >> index
}

impl Display {
    /// Returns a new, cleared display instance
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }
    /// Clears the display
    pub fn cls(&mut self) {
        self.screen = [0; WIDTH * HEIGHT];
    }
    /// The interpreter reads n bytes from memory, starting at the address stored in I.
    /// These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
    /// VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it
    /// is outside the coordinates of the display, it wraps around to the opposite side of
    /// the screen.
    /// Returns true if a collision was detected, false otherwise
    pub fn draw_sprite(&mut self, memory: &[u8], n: usize, I: usize, vx: usize, vy: usize) -> bool {
        const BYTE_WIDTH: usize = 8;
        let mut res = false;
        for (r, &byte) in memory[I..I + n].iter().enumerate() {
            for bit_index in 0..BYTE_WIDTH {
                // I think this may actually be right...
                // ... not sure how to test though
                let row = ((vy + r) * WIDTH) % (HEIGHT * WIDTH);
                let col = (vx + bit_index) % WIDTH;
                let pixel = byte_index(byte, BYTE_WIDTH - bit_index - 1);
                let screen_index = row + col;
                self.screen[screen_index] ^= pixel;
                res = res || (self.screen[screen_index] == 0);
            }
        }
        res
    }
}
