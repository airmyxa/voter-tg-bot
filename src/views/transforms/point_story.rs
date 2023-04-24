use crate::models::point_story::keyboard::Keyboard;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn to_tg_keyboard(keyboard: &Keyboard) -> InlineKeyboardMarkup {
    let mut result: Vec<Vec<InlineKeyboardButton>> = vec![];
    for row in &keyboard.key_rows {
        let mut result_row: Vec<InlineKeyboardButton> = vec![];
        for key in row {
            result_row.push(InlineKeyboardButton::callback(key.clone(), key.clone()));
        }
        result.push(result_row);
    }
    return InlineKeyboardMarkup::new(result);
}
