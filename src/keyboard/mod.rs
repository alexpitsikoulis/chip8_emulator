use crate::{Application, Message};
use relm4::{
    gtk::{gdk::Key, prelude::IsA, traits::WidgetExt, EventControllerKey, Inhibit, Widget},
    ComponentSender,
};
use std::sync::mpsc::Sender;

pub const HEX_KEYS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub struct Keyboard {
    keys: u16,
    sender: Sender<u16>,
}

impl Keyboard {
    pub fn new(sender: Sender<u16>) -> Self {
        return Self {
            keys: 0,
            sender: sender,
        };
    }

    pub fn change_key_state(&mut self, key: Key, pressed: bool) {
        if let Some(key_val) = key.to_unicode() {
            match key_val {
                '0'..='9' | 'a'..='f' => {
                    if let Some(pos) = HEX_KEYS.iter().position(|&x| x == key_val) {
                        let bin_pos = 15 - pos;

                        let mut lhs = self.keys >> bin_pos;

                        let mut rhs_mask = 0;
                        for _ in 0..bin_pos {
                            rhs_mask = rhs_mask << 1 | 1;
                        }
                        let rhs = self.keys & rhs_mask;

                        if pressed {
                            lhs = lhs | 1;
                        } else {
                            lhs = lhs >> 1 << 1;
                        }
                        self.keys = (lhs << bin_pos) | rhs;
                        if pressed {
                            self.sender.send(self.keys).unwrap();
                        }
                    };
                }
                _ => {}
            }
        };
    }

    pub fn register_keyboard_controller(
        &self,
        host: &impl IsA<Widget>,
        app_sender: ComponentSender<Application>,
    ) {
        let controller = EventControllerKey::new();
        let app_sender_ = app_sender.clone();
        controller.connect_key_pressed(move |_this, key, _code, _modifier| -> Inhibit {
            app_sender_.input(Message::KeyDown(key.to_lower()));
            return Inhibit(false);
        });
        let app_sender_ = app_sender.clone();
        controller.connect_key_released(move |_this, key, _code, _modifier| {
            app_sender_.input(Message::KeyUp(key));
        });
        host.add_controller(&controller);
    }
}
