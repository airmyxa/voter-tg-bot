use crate::models::point_story::keyboard::Keyboard;
use crate::models::vote::VoteState;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

impl std::convert::From<Keyboard> for InlineKeyboardMarkup {
    fn from(value: Keyboard) -> Self {
        let mut result: Vec<Vec<InlineKeyboardButton>> = vec![];
        for row in &value.key_rows {
            let mut result_row: Vec<InlineKeyboardButton> = vec![];
            for key in row {
                result_row.push(InlineKeyboardButton::callback(key.clone(), key.clone()));
            }
            result.push(result_row);
        }
        return InlineKeyboardMarkup::new(result);
    }
}
