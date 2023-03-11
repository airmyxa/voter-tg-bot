use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

static POINTS: &'static [&'static str] = &["0", "1", "2", "3", "5", "8", "13", "21"];

static ACTIONS: &'static [&'static str] = &["Open", "Dismiss"];

static LOGICAL_KEYBOARD_BLOCKS: &'static [&'static [&'static str]] = &[POINTS, ACTIONS];

static ROW_SIZE: usize = 5;

pub struct KeyboardBuilder {}

impl KeyboardBuilder {
    pub fn make_keyboard() -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

        for block in LOGICAL_KEYBOARD_BLOCKS {
            for elems in block.chunks(ROW_SIZE) {
                let row = elems
                    .iter()
                    .map(|&elem| InlineKeyboardButton::callback(elem.to_owned(), elem.to_owned()))
                    .collect();

                keyboard.push(row);
            }
        }

        InlineKeyboardMarkup::new(keyboard)
    }
}
