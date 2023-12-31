use bitflags::bitflags;
use winit::keyboard::{KeyCode, ModifiersState};
use winit_input_helper::WinitInputHelper;
use rustc_hash::FxHashMap as HashMap;

bitflags! {
    pub struct InputState: u8 {
        const Released = 0b0001;
        const Pressed = 0b0010;
        const Held = 0b0100;
        const HeldRepeat = 0b1000;
    }
}

pub struct InputManager {
    inputs: HashMap<String, (KeyCode, ModifiersState, InputState)>,
}

impl InputManager {
    pub fn new() -> Self {
        Self { inputs: HashMap::default() }
    }

    pub fn register_input(&mut self, name: &str, key: KeyCode, modifiers: ModifiersState, input_state: InputState) {
         self.inputs.insert(name.to_string(), (key, modifiers, input_state));
    }

    pub fn set_input(&mut self, name: &str, key: KeyCode, modifiers: ModifiersState) {
        if let Some(a) = self.inputs.get_mut(name) {
            a.0 = key;
            a.1 = modifiers;
        }
    }

    pub fn get_input(&self, name: &str, input_helper: &WinitInputHelper) -> bool {
        if let Some(input) = self.inputs.get(name) {
            let input_state = (input.2.contains(InputState::Held) && input_helper.key_held(input.0)) || 
                (input.2.contains(InputState::HeldRepeat) && input_helper.key_pressed_os(input.0)) || 
                (input.2.contains(InputState::Released) && input_helper.key_released(input.0)) ||
                (input.2.contains(InputState::Pressed) && input_helper.key_pressed(input.0));

            let mut modifier_value = ModifiersState::empty();
            modifier_value.set(ModifiersState::SHIFT, input_helper.held_shift());
            modifier_value.set(ModifiersState::ALT, input_helper.held_alt());
            modifier_value.set(ModifiersState::CONTROL, input_helper.held_control());
            modifier_value.set(ModifiersState::SUPER, input_helper.key_held(KeyCode::SuperLeft) || input_helper.key_held(KeyCode::SuperRight));

            let modifier_state = modifier_value.contains(input.1);

            return input_state && modifier_state;
        }
        false
    }
}
