use device_query::DeviceQuery;
use device_query::{DeviceState, Keycode};

use crate::api::APIHandle;

const XPOS_OFFSETS: &[usize] = &[0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x10];

const YPOS_OFFSETS: &[usize] = &[0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x14];

const ZPOS_OFFSETS: &[usize] = &[0x1001FA0, 0x260, 0x2E8, 0x318, 0x10, 0x28, 0x40, 0x18];

const KEYS: &[Keycode] = &[
    Keycode::X,
    Keycode::Y,
    Keycode::Z,
    Keycode::Key0,
    Keycode::Key1,
    Keycode::Key2,
    Keycode::Key3,
    Keycode::Key4,
    Keycode::Key5,
    Keycode::Key6,
    Keycode::Key7,
    Keycode::Key8,
    Keycode::Key9,
    Keycode::Dot,
    Keycode::Minus,
    Keycode::Enter,
    Keycode::N,
    Keycode::Backspace,
    Keycode::C,
];

enum PositionKey {
    X,
    Y,
    Z,
}
enum NumMode {
    Increase,
    Set,
}

pub struct InputHandler {
    device_state: DeviceState,
    prev_keys: Vec<Keycode>,
    position_key: PositionKey,
    num_mode: NumMode,
    input_text: String,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            device_state: DeviceState::new(),
            prev_keys: Vec::new(),
            position_key: PositionKey::X,
            num_mode: NumMode::Set,
            input_text: String::new(),
        }
    }

    pub fn get_next_key(&mut self) -> Option<Keycode> {
        let keys = self.device_state.get_keys();
        let relevant_keys: Vec<Keycode> = keys
            .into_iter()
            .filter(|key| KEYS.contains(key))
            .collect();

        let filtered_keys: Vec<&Keycode> = relevant_keys.iter().filter(|&key| !self.prev_keys.contains(key)).collect();

        self.prev_keys = relevant_keys.iter().map(|key| key.clone()).collect();
        if filtered_keys.len() > 0 {
            Some(filtered_keys[0].clone().clone())
        } else {
            None
        }
    }

    pub fn update(&mut self, text: &mut String, api_handle: &APIHandle) {
        let position_key = match self.position_key {
            PositionKey::X => "x",
            PositionKey::Y => "y",
            PositionKey::Z => "z",
        };

        let num_mode = match self.num_mode {
            NumMode::Increase => "INC",
            NumMode::Set => "SET",
        };

        let x_pos = api_handle.read_memory_f32(XPOS_OFFSETS);
        let y_pos = api_handle.read_memory_f32(YPOS_OFFSETS);
        let z_pos = api_handle.read_memory_f32(ZPOS_OFFSETS);

        text.clear();
        text.push_str(&format!("X: {}\nY: {}\nZ: {}\n{} ({})> {}", x_pos, y_pos, z_pos, position_key, num_mode, self.input_text));

        let key_pressed = self.get_next_key();
        match key_pressed {
            Some(Keycode::Key0) => self.input_text.push('0'),
            Some(Keycode::Key1) => self.input_text.push('1'),
            Some(Keycode::Key2) => self.input_text.push('2'),
            Some(Keycode::Key3) => self.input_text.push('3'),
            Some(Keycode::Key4) => self.input_text.push('4'),
            Some(Keycode::Key5) => self.input_text.push('5'),
            Some(Keycode::Key6) => self.input_text.push('6'),
            Some(Keycode::Key7) => self.input_text.push('7'),
            Some(Keycode::Key8) => self.input_text.push('8'),
            Some(Keycode::Key9) => self.input_text.push('9'),
            Some(Keycode::Minus) => self.input_text.push('-'),
            Some(Keycode::Dot) => self.input_text.push('.'),
            Some(Keycode::X) => self.position_key = PositionKey::X,
            Some(Keycode::Y) => self.position_key = PositionKey::Y,
            Some(Keycode::Z) => self.position_key = PositionKey::Z,
            Some(Keycode::N) => self.num_mode = {
                match self.num_mode {
                    NumMode::Increase => NumMode::Set,
                    NumMode::Set => NumMode::Increase,
                }
            },
            Some(Keycode::Backspace) => {self.input_text.pop(); ()},
            Some(Keycode::C) => self.input_text.clear(),
            Some(Keycode::Enter) => {
                let parsed_str = self.input_text.parse::<f32>();
                if let Ok(num) = parsed_str {
                    match self.position_key {
                        PositionKey::X => {
                            match self.num_mode {
                                NumMode::Increase => api_handle.write_memory_f32(XPOS_OFFSETS, x_pos + num),
                                NumMode::Set => api_handle.write_memory_f32(XPOS_OFFSETS, num),
                            }
                        },
                        PositionKey::Y => {
                            match self.num_mode {
                                NumMode::Increase => api_handle.write_memory_f32(YPOS_OFFSETS, y_pos + num),
                                NumMode::Set => api_handle.write_memory_f32(YPOS_OFFSETS, num),
                            }
                        },
                        PositionKey::Z => {
                            match self.num_mode {
                                NumMode::Increase => api_handle.write_memory_f32(ZPOS_OFFSETS, z_pos + num),
                                NumMode::Set => api_handle.write_memory_f32(ZPOS_OFFSETS, num),
                            }
                        },
                    }
                }
            }
            _ => (),
        }
    }
}
