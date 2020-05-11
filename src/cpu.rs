// Documentation: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5

#[allow(non_snake_case)]
struct CPU {
    /// CHIP-8 CPU

    /// 4K of RAM
    // Most programs start at 512 (0x200)
    memory: [u8; 4096],

    /// 16 general purpose 8-bit registers, named V0 - VF
    /// The VF register should not be touched as it is used a flag by some instructions
    V: [u8; 16],

    /// One 16-bit register to store memory addresses (the index register)
    /// Usually, only the lower 12 bits are used
    I: u16,

    /// Two special purpose 16-bit registers for the delay and sound timers
    /// While non-zero, decremented at a rate of 60hz

    /// The delay timer is active whenever the delay timer register (DT) is non-zero
    /// This timer does nothing more than subtract 1 from the value of DT at a rate of 60Hz
    /// When DT reaches 0, it deactivates
    DT: u16,
    /// The sound timer is active whenever the sound timer register (ST) is non-zero
    /// his timer also decrements at a rate of 60Hz, however, as long as ST's value is greater than zero, the Chip-8 buzzer will sound
    /// When ST reaches zero, the sound timer deactivates
    ST: u16,

    /// Program counter (PC) should be 16-bit
    PC: u16,

    /// Stack pointer can be 8-bit, used to indicate the topmost level of the stack
    SP: u8,

    /// Stack is an array of 16 16-bit values
    /// CHIP-8 allows for up to 16 levels of nested subroutines
    stack: [u16; 16],

    /// 16-key hexadecimal keyboard
    /// Not sure if this is the best way to implement this
    keyboard: [bool; 16],

    /// 64x32-pixel monochrome display
    display: [u8; 64 * 32],
}

impl CPU {
    /// All instructions are two bytes long and are stored most-significant-byte first
    /// In memory, the first byte of each instruction should be located at an even addresses
    fn read_opcode(&self) -> u16 {
        ((self.memory[self.PC as usize] as u16) << 8) | (self.memory[(self.PC + 1) as usize] as u16)
    }

    /// Executes the current cycle
    fn execute_cycle(&mut self) {
        let opcode = self.read_opcode();
        self.process_opcode(&mut self, opcode);
    }

    /// Processes the given opcode
    /// In these listings, the following variables are used:

    /// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction     _nnn
    /// n or nibble - A 4-bit value, the lowest 4 bits of the instruction       ___n
    /// x - A 4-bit value, the lower 4 bits of the high byte of the instruction _x__
    /// y - A 4-bit value, the upper 4 bits of the low byte of the instruction  __y_
    /// kk or byte - An 8-bit value, the lowest 8 bits of the instruction       __kk
    fn process_opcode(&mut self, opcode: u16) {
        // Break up opcode
        let nnn = opcode & 0x0FFF;
        let n = opcode & 0x000F;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let vx = self.V[x];
        let vy = self.V[y];

        // Increment program counter
        // Remember! Opcodes are two bytes but memory is byte addressed
        self.PC += 2;

        match opcode {
            // 0nnn - SYS addr
            // Jump to a machine code routine at nnn.
            // This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
            0x0000..=0x0FFF => (),

            // 00E0 - CLS
            // Clear the display.
            0x00E0 => self.display = [0u8; 64 * 32],

            // 00EE - RET
            // Return from a subroutine.
            // DO THIS BACKWARDS
            // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
            0x00EE => {
                self.SP -= 1;
                self.PC = self.stack[self.SP as usize];
            }

            //1nnn - JP addr
            //Jump to location nnn.
            //The interpreter sets the program counter to nnn.
            0x1000..=0x1FFF => self.PC = nnn,

            // 2nnn - CALL addr
            // Call subroutine at nnn.
            // DO THIS BACKWARDS
            // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            0x2000..=0x2FFF => {
                self.stack[self.SP as usize] = self.PC;
                self.SP += 1;
                self.PC = nnn;
            }

            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx = kk.
            // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
            0x3000..=0x3FFF => {
                if vx == kk {
                    self.PC += 2;
                }
            }

            // 4xkk - SNE Vx, byte
            // Skip next instruction if Vx != kk.
            // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
            0x4000..=0x4FFF => {
                if vx != kk {
                    self.PC += 2;
                }
            }

            // 5xy0 - SE Vx, Vy
            // Skip next instruction if Vx = Vy.
            // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
            0x5000..=0x5FFF => {
                if vx == vy {
                    self.PC += 2;
                }
            }
            // 6xkk - LD Vx, byte
            // Set Vx = kk.
            // The interpreter puts the value kk into register Vx.
            0x6000..=0x6FFF => {
                self.V[x] = kk as u8;
            }
            // 7xkk - ADD Vx, byte
            // Set Vx = Vx + kk.
            // Adds the value kk to the value of register Vx, then stores the result in Vx.
            0x7000..=0x7FFF => {
                self.V[x] += kk as u8;
            }

            0x8000..=0x8FFF => {
                // TODO
            }

            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy.
            // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
            0x9000..=0x9FFF => {
                if vx == vy {
                    self.PC += 2;
                }
            }

            // Annn - LD I, addr
            // Set I = nnn.
            // The value of register I is set to nnn.
            0xA000..=0xAFFF => self.I = nnn,
            // Bnnn - JP V0, addr
            // Jump to location nnn + V0.
            // The program counter is set to nnn plus the value of V0.
            0xB000..=0xBFFF => self.PC = self.V[0usize] + nnn,
            // Cxkk - RND Vx, byte
            // Set Vx = random byte AND kk.
            // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
            0xC000..=0xCFFF => {
                // TODO
            }

            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
            0xD000..=0xDFFF => {
                // TODO
            }

            0xE000..=0xEFFF => {
                match kk {
                    // Ex9E - SKP Vx
                    // Skip next instruction if key with the value of Vx is pressed.
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                    0x9E => {
                        // TODO
                    }

                    // ExA1 - SKNP Vx
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
                    // Skip next instruction if key with the value of Vx is not pressed.
                    0xA1 => {
                        // TODO
                    }

                    _ => (),
                }
            }

            0xF000..=0xFFFF => {
                match kk {
                    // Fx07 - LD Vx, DT
                    // Set Vx = delay timer value.
                    // The value of DT is placed into Vx.
                    0x07 => self.V[x] = (self.DT as u8),

                    // Fx0A - LD Vx, K
                    // Wait for a key press, store the value of the key in Vx.
                    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                    0x08 => {
                        // TODO
                    }

                    // Fx15 - LD DT, Vx
                    // Set delay timer = Vx.
                    // DT is set equal to the value of Vx.
                    0x15 => self.DT = (vx as u16),

                    // Fx18 - LD ST, Vx
                    // Set sound timer = Vx.
                    // ST is set equal to the value of Vx.
                    0x18 => self.ST = (vx as u16),

                    // Fx1E - ADD I, Vx
                    // Set I = I + Vx.
                    // The values of I and Vx are added, and the results are stored in I.
                    0x1E => self.I += (vx as u16),

                    // Fx29 - LD F, Vx
                    // Set I = location of sprite for digit Vx.
                    // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
                    0x29 => {
                        // TODO
                    }

                    // Fx33 - LD B, Vx
                    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                    0x33 => {
                        let hundreds = (vx / 100) % 10;
                        let tens = (vx / 10) % 10;
                        let ones = vx % 10;
                        self.memory[self.I as usize] = hundreds;
                        self.memory[(self.I + 1) as usize] = tens;
                        self.memory[(self.I + 2) as usize] = ones;
                    }

                    // Fx55 - LD [I], Vx
                    // Store registers V0 through Vx in memory starting at location I.
                    // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                    0x55 => {
                        // TODO
                    }

                    // Fx65 - LD Vx, [I]
                    // Read registers V0 through Vx from memory starting at location I.
                    // The interpreter reads values from memory starting at location I into registers V0 through Vx.
                    0x65 => {
                        // TODO
                    }
                }
            }
        }
    }
}
