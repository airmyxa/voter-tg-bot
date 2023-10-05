use crate::models::vote::VoteState;

pub static POINTS: &'static [&'static str] = &["0", "1", "2", "3", "5", "8", "13", "21"];

pub static ACTIONS: &'static [&'static str] = &["Open", "Dismiss"];

pub static LOGICAL_KEYBOARD_BLOCKS: &'static [&'static [&'static str]] = &[POINTS, ACTIONS];

pub static ROW_SIZE: usize = 5;

type KeyRow = Vec<String>;

pub struct Keyboard {
    pub key_rows: Vec<KeyRow>,
}

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
