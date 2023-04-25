use crate::models::point_story::keyboard::{Keyboard, LOGICAL_KEYBOARD_BLOCKS, ROW_SIZE};
use crate::models::vote::VoteState;

pub struct KeyboardBuilder {
    state: VoteState,
}

impl KeyboardBuilder {
    pub fn new(state: VoteState) -> Self {
        KeyboardBuilder { state }
    }

    pub fn make_keyboard(&self) -> Keyboard {
        return match self.state {
            VoteState::Init => self.build_init_keyboard(),
            VoteState::InProcess => self.build_init_keyboard(),
        };
    }

    fn build_init_keyboard(&self) -> Keyboard {
        let mut keyboard = Keyboard { key_rows: vec![] };

        for block in LOGICAL_KEYBOARD_BLOCKS {
            for elems in block.chunks(ROW_SIZE) {
                let row = elems
                    .iter()
                    .map(|&elem| elem.to_string())
                    //.map(|&elem| InlineKeyboardButton::callback(elem.to_owned(), elem.to_owned()))
                    .collect();

                keyboard.key_rows.push(row);
            }
        }
        return keyboard;
    }
}
