use crate::models::keyboard;
use crate::models::{keyboard::Keyboard, vote::VoteState};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

impl From<Keyboard> for InlineKeyboardMarkup {
    fn from(value: Keyboard) -> Self {
        let mut result: Vec<Vec<InlineKeyboardButton>> = vec![];
        for row in &keyboard.rows {
            let mut result_row: Vec<InlineKeyboardButton> = vec![];
            for key in row {
                result_row.push(InlineKeyboardButton::callback(key.clone(), key.clone()));
            }
            result.push(result_row);
        }
        return InlineKeyboardMarkup::new(result);
    }
}
