use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::views::commands::point_story::template;

pub struct KeyboardBuilder {}

impl KeyboardBuilder {
    pub fn make_keyboard() -> InlineKeyboardMarkup {
        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

        for points in template::POINTS.chunks(template::ROW_SIZE) {
            let row = points
                .iter()
                .map(|&point| {
                    InlineKeyboardButton::callback(point.to_owned(), point.to_owned())
                })
                .collect();

            keyboard.push(row);
        }

        for actions in template::ACTIONS.chunks(template::ROW_SIZE) {
            let row = actions
                .iter()
                .map(|&action| {
                    InlineKeyboardButton::callback(action.to_owned(), action.to_owned())
                })
                .collect();

            keyboard.push(row);
        }

        InlineKeyboardMarkup::new(keyboard)
    }
}