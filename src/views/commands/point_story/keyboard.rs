use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::views::commands::point_story::template;

pub struct KeyboardBuilder {}

impl KeyboardBuilder {
    pub fn make_keyboard() -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

        for block in template::LOGICAL_KEYBOARD_BLOCKS {
            for elems in block.chunks(template::ROW_SIZE) {
                let row = elems
                    .iter()
                    .map(|&elem| {
                        InlineKeyboardButton::callback(elem.to_owned(), elem.to_owned())
                    })
                    .collect();

                keyboard.push(row);
            }
        }

        InlineKeyboardMarkup::new(keyboard)
    }

}