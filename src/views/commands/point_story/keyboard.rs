use crate::models::point_story::keyboard::{Keyboard, KeyboardBuilder};
use crate::models::vote::VoteState;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn make_keyboard(vote_state: VoteState) -> InlineKeyboardMarkup {
    let keyboard = KeyboardBuilder::new(vote_state).make_keyboard();
    let keyboard = to_tg_keyboard(&keyboard);
    return keyboard;
}

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
