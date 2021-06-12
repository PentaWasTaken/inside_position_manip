use device_query::DeviceQuery;
use device_query::{DeviceState, Keycode};

use crate::api::APIHandle;

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
    Keycode::Minus,
    Keycode::NumpadSubtract,
    Keycode::Enter,
    Keycode::Q,
    Keycode::N,
    Keycode::Backspace,
    Keycode::C,
    Keycode::P,
    Keycode::B,
    Keycode::L,
];

#[derive(Clone, Copy)]
enum NumMode {
    Increase,
    Set,
}

pub struct Parameter {
    name: String,
    address: Vec<usize>,
    pub locked: Option<f32>,
}

pub struct InputHandler {
    device_state: DeviceState,
    prev_keys: Vec<Keycode>,
    num_mode: NumMode,
    pub parameters: Vec<Parameter>,
    current_param: String,
    input_text: String,
}

impl InputHandler {
    pub fn new() -> Self {
        let mut parameters = Vec::new();
        parameters.push(Parameter {
            name: "x_pos".to_string(),
            address: vec![0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x10],
            locked: None,
        });

        parameters.push(Parameter {
            name: "y_pos".to_string(),
            address: vec![0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x14],
            locked: None,
        });

        parameters.push(Parameter {
            name: "z_pos".to_string(),
            address: vec![0x1001FA0, 0x260, 0x2E8, 0x318, 0x10, 0x28, 0x40, 0x18],
            locked: None,
        });

        parameters.push(Parameter {
            name: "x_vel".to_string(),
            address: vec![0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x1C],
            locked: None,
        });

        parameters.push(Parameter {
            name: "y_vel".to_string(),
            address: vec![0xF92610, 0x4C0, 0x10, 0x98, 0x670, 0x0, 0x58, 0x70, 0x20],
            locked: None,
        });

        parameters.push(Parameter {
            name: "breath".to_string(),
            address: vec![0xF92610, 0x18, 0xE0, 0x98, 0x508, 0x20, 0x28, 0x104],
            locked: None,
        });

        let current_param = parameters[0].name.clone();

        InputHandler {
            device_state: DeviceState::new(),
            prev_keys: Vec::new(),
            num_mode: NumMode::Set,
            parameters,
            current_param,
            input_text: String::new(),
        }
    }

    pub fn get_param(&self, name: &str) -> Option<&Parameter> {
        self.parameters
            .iter()
            .find(|x| x.name == name)
    }

    pub fn get_param_mut(&mut self, name: &str) -> Option<&mut Parameter> {
        self.parameters.iter_mut().find(|x| x.name == name)
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
        let num_mode = match self.num_mode {
            NumMode::Increase => "INC",
            NumMode::Set => "SET",
        };

        text.clear();

        for parameter in &self.parameters {
            let value = api_handle.read_memory_f32(&parameter.address);
            text.push_str(&format!("{}: {:.4}\n", parameter.name, value));
            if let Some(val) = parameter.locked {
                api_handle.write_memory_f32(&parameter.address, val);
            }
        }

        text.push_str(&format!(
            "{} ({}) > {}",
            self.current_param, num_mode, self.input_text
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
            Keycode::Dot => self.input_text.push('.'),
            Keycode::X => {
                if self.current_param.ends_with("pos") {
                    self.current_param = "x_pos".to_string();
                } else if self.current_param.ends_with("vel") {
                    self.current_param = "x_vel".to_string();
                } else {
                    self.current_param = "x_pos".to_string();
                }
            }
            Keycode::Y => {
                if self.current_param.ends_with("pos") {
                    self.current_param = "y_pos".to_string();
                } else if self.current_param.ends_with("vel") {
                    self.current_param = "y_vel".to_string();
                } else {
                    self.current_param = "y_pos".to_string();
                }
            }
            Keycode::Z => {
                if self.current_param.ends_with("pos") {
                    self.current_param = "z_pos".to_string();
                } else if self.current_param.ends_with("vel") {
                    self.current_param = "z_vel".to_string();
                } else {
                    self.current_param = "z_pos".to_string();
                }
            }
            Keycode::B => self.current_param = "breath".to_string(),
            Keycode::N => {
                self.num_mode = {
                    match self.num_mode {
                        NumMode::Increase => NumMode::Set,
                        NumMode::Set => NumMode::Increase,
                    }
                }
            }
            Keycode::P => {
                if self.current_param.ends_with("pos") {
                    self.current_param = self.current_param.replace("pos", "vel");
                } else if self.current_param.ends_with("vel") {
                    self.current_param = self.current_param.replace("vel", "pos");
                }
            }
            Keycode::Backspace => {
                self.input_text.pop();
            }
            Keycode::C => self.input_text.clear(),
            Keycode::L => {
                let p = self.get_param_mut(&self.current_param.clone());

                if let Some(parameter) = p {
                    if parameter.locked.is_none() {
                        parameter.locked = Some(api_handle.read_memory_f32(&parameter.address));
                    } else {
                        parameter.locked = None;
                    }
                }
            }
            Keycode::Enter | Keycode::Q => {
                let parsed_str = self.input_text.parse::<f32>();
                if let Ok(num) = parsed_str {
                    let n = self.num_mode;
                    
                    let p = self.get_param_mut(&self.current_param.clone());

                    if let Some(parameter) = p {
                        match n {
                            NumMode::Set => {
                                api_handle.write_memory_f32(&parameter.address, num);
                                if parameter.locked.is_some() {
                                    parameter.locked = Some(num);
                                }
                            },
                            NumMode::Increase => {
                                let prev_value = api_handle.read_memory_f32(&parameter.address);
                                api_handle.write_memory_f32(&parameter.address, num + prev_value);
                                if parameter.locked.is_some() {
                                    parameter.locked = Some(num + prev_value);
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
