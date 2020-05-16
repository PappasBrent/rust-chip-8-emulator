use chip_8::cpu::display::{HEIGHT, WIDTH};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
fn main() {
    // TODO: Check that arguments were passed correctly
    let game_path = std::env::args().nth(1).unwrap();

    let mut cpu = chip_8::cpu::CPU::new();
    cpu.reset();

    let rom = std::fs::read(std::path::Path::new(&game_path)).unwrap();

    cpu.load_rom(&rom);

    const COLOR: u32 = 0x00FFFF;
    const NO_COLOR: u32 = 0x333333;
    const SCALE: usize = 15;
    const SCREEN_WIDTH: usize = WIDTH * SCALE;
    const SCREEN_HEIGHT: usize = HEIGHT * SCALE;
    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "Chip 8 Emulator",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(5000)));
    window.limit_update_rate(Some(std::time::Duration::from_millis(1000 / 60)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Get input
        for &key in window.get_keys_released().unwrap_or(vec![]).iter() {
            let btn: usize = match key {
                Key::Key1 => 1,
                Key::Key2 => 2,
                Key::Key3 => 3,
                Key::Key4 => 0xC,
                Key::Q => 4,
                Key::W => 5,
                Key::F => 6,
                Key::P => 0xD,
                Key::A => 7,
                Key::R => 8,
                Key::S => 9,
                Key::T => 0xE,
                Key::Z => 0xA,
                Key::X => 0,
                Key::C => 0xB,
                Key::V => 0xF,
                _ => 16,
            };
            if btn <= 0xF_usize {
                cpu.keyboard.key_up(btn);
            }
        }

        for &key in window
            .get_keys_pressed(KeyRepeat::Yes)
            .unwrap_or(vec![])
            .iter()
        {
            let btn: usize = match key {
                Key::Key1 => 1,
                Key::Key2 => 2,
                Key::Key3 => 3,
                Key::Key4 => 0xC,
                Key::Q => 4,
                Key::W => 5,
                Key::F => 6,
                Key::P => 0xD,
                Key::A => 7,
                Key::R => 8,
                Key::S => 9,
                Key::T => 0xE,
                Key::Z => 0xA,
                Key::X => 0,
                Key::C => 0xB,
                Key::V => 0xF,
                _ => 16,
            };
            if btn <= 0xF_usize {
                println!("{}", btn);
                cpu.keyboard.key_down(btn);
            }
        }

        // Update game
        cpu.execute_cycle();
        cpu.decrement_timers();

        // Draw pixels
        for (i, &val) in cpu.display.screen_buffer().iter().enumerate() {
            for r in 0..SCALE {
                let row_offset = ((i / WIDTH) * SCALE + r) * SCREEN_WIDTH;
                let col_start = (i % WIDTH) * SCALE;
                let col_end = (i % WIDTH) * SCALE + SCALE;
                buffer[row_offset + col_start..row_offset + col_end].copy_from_slice(if val == 1 {
                    &[COLOR; SCALE]
                } else {
                    &[NO_COLOR; SCALE]
                });
            }
        }

        // NOTE: Keys assume QWERTY layout! Changing to Colemak doesn't change this!

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
