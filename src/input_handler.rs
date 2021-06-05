use device_query::DeviceQuery;
use device_query::{DeviceState, Keycode};

use crate::api::APIHandle;

const XPOS_OFFSETS: &[usize] = &[0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x10];

const YPOS_OFFSETS: &[usize] = &[0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x14];

const ZPOS_OFFSETS: &[usize] = &[0x1001FA0, 0x260, 0x2E8, 0x318, 0x10, 0x28, 0x40, 0x18];

const XVEL_OFFSETS: &[usize] = &[0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x1C];

const YVEL_OFFSETS: &[usize] = &[0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x20];

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
    Keycode::Numpad0,
    Keycode::Numpad1,
    Keycode::Numpad2,
    Keycode::Numpad3,
    Keycode::Numpad4,
    Keycode::Numpad5,
    Keycode::Numpad6,
    Keycode::Numpad7,
    Keycode::Numpad8,
    Keycode::Numpad9,
    Keycode::Dot,
    Keycode::Comma,
    Keycode::Minus,
    Keycode::NumpadSubtract,
    Keycode::Enter,
    Keycode::N,
    Keycode::Backspace,
    Keycode::C,
    Keycode::P,
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

enum Parameter {
    Position,
    Velocity,
}

pub struct InputHandler {
    device_state: DeviceState,
    prev_keys: Vec<Keycode>,
    position_key: PositionKey,
    num_mode: NumMode,
    parameter: Parameter,
    input_text: String,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            device_state: DeviceState::new(),
            prev_keys: Vec::new(),
            position_key: PositionKey::X,
            num_mode: NumMode::Set,
            parameter: Parameter::Position,
            input_text: String::new(),
        }
    }

    pub fn get_next_key(&mut self) -> Option<Keycode> {
        let keys = self.device_state.get_keys();
        let relevant_keys: Vec<Keycode> =
            keys.into_iter().filter(|key| KEYS.contains(key)).collect();

        let filtered_keys: Vec<&Keycode> = relevant_keys
            .iter()
            .filter(|&key| !self.prev_keys.contains(key))
            .collect();

        self.prev_keys = relevant_keys.to_vec();
        if !filtered_keys.is_empty() {
            Some(filtered_keys[0].clone())
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

        let parameter = match self.parameter {
            Parameter::Position => "POS",
            Parameter::Velocity => "VEL",
        };

        let x_pos = api_handle.read_memory_f32(XPOS_OFFSETS);
        let y_pos = api_handle.read_memory_f32(YPOS_OFFSETS);
        let z_pos = api_handle.read_memory_f32(ZPOS_OFFSETS);
        let x_vel = api_handle.read_memory_f32(XVEL_OFFSETS);
        let y_vel = api_handle.read_memory_f32(YVEL_OFFSETS);

        text.clear();
        text.push_str(&format!(
            "X: {:.4}\nY: {:.4}\nZ: {:.4}\nXV: {:.4}\nYV: {:.4}\n{} ({}, {})> {}",
            x_pos, y_pos, z_pos, x_vel, y_vel, position_key, parameter, num_mode, self.input_text
        ));

        let key_pressed = self.get_next_key();

        if key_pressed.is_none() {
            return;
        }

        match key_pressed.unwrap() {
            Keycode::Key0 | Keycode::Numpad0 => self.input_text.push('0'),
            Keycode::Key1 | Keycode::Numpad1 => self.input_text.push('1'),
            Keycode::Key2 | Keycode::Numpad2 => self.input_text.push('2'),
            Keycode::Key3 | Keycode::Numpad3 => self.input_text.push('3'),
            Keycode::Key4 | Keycode::Numpad4 => self.input_text.push('4'),
            Keycode::Key5 | Keycode::Numpad5 => self.input_text.push('5'),
            Keycode::Key6 | Keycode::Numpad6 => self.input_text.push('6'),
            Keycode::Key7 | Keycode::Numpad7 => self.input_text.push('7'),
            Keycode::Key8 | Keycode::Numpad8 => self.input_text.push('8'),
            Keycode::Key9 | Keycode::Numpad9 => self.input_text.push('9'),
            Keycode::Minus | Keycode::NumpadSubtract => self.input_text.push('-'),
            Keycode::Dot | Keycode::Comma => self.input_text.push('.'),
            Keycode::X => self.position_key = PositionKey::X,
            Keycode::Y => self.position_key = PositionKey::Y,
            Keycode::Z => self.position_key = PositionKey::Z,
            Keycode::N => {
                self.num_mode = {
                    match self.num_mode {
                        NumMode::Increase => NumMode::Set,
                        NumMode::Set => NumMode::Increase,
                    }
                }
            }
            Keycode::P => {
                self.parameter = {
                    match self.parameter {
                        Parameter::Position => Parameter::Velocity,
                        Parameter::Velocity => Parameter::Position,
                    }
                }
            }
            Keycode::Backspace => {
                self.input_text.pop();
            }
            Keycode::C => self.input_text.clear(),
            Keycode::Enter => {
                let parsed_str = self.input_text.parse::<f32>();
                if let Ok(num) = parsed_str {
                    match self.position_key {
                        PositionKey::X => match self.parameter {
                            Parameter::Position => match self.num_mode {
                                NumMode::Increase => {
                                    api_handle.write_memory_f32(XPOS_OFFSETS, x_pos + num)
                                }
                                NumMode::Set => api_handle.write_memory_f32(XPOS_OFFSETS, num),
                            },
                            Parameter::Velocity => match self.num_mode {
                                NumMode::Increase => {
                                    api_handle.write_memory_f32(XVEL_OFFSETS, x_vel + num)
                                }
                                NumMode::Set => api_handle.write_memory_f32(XVEL_OFFSETS, num),
                            },
                        },
                        PositionKey::Y => match self.parameter {
                            Parameter::Position => match self.num_mode {
                                NumMode::Increase => {
                                    api_handle.write_memory_f32(YPOS_OFFSETS, y_pos + num)
                                }
                                NumMode::Set => api_handle.write_memory_f32(YPOS_OFFSETS, num),
                            },
                            Parameter::Velocity => match self.num_mode {
                                NumMode::Increase => {
                                    api_handle.write_memory_f32(YVEL_OFFSETS, y_vel + num)
                                }
                                NumMode::Set => api_handle.write_memory_f32(YVEL_OFFSETS, num),
                            },
                        },
                        PositionKey::Z => match self.parameter {
                            Parameter::Position => match self.num_mode {
                                NumMode::Increase => {
                                    api_handle.write_memory_f32(ZPOS_OFFSETS, z_pos + num)
                                }
                                NumMode::Set => api_handle.write_memory_f32(ZPOS_OFFSETS, num),
                            },
                            Parameter::Velocity => {}
                        },
                    }
                }
            }
            _ => (),
        }
    }
}
