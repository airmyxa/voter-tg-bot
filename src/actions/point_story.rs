use crate::controller::point_story::keyboard::KeyboardBuilder;
use crate::models::vote::VoteState;
use crate::views::transforms;
use teloxide::types::InlineKeyboardMarkup;

pub fn make_keyboard(vote_state: VoteState) -> InlineKeyboardMarkup {
    let keyboard = KeyboardBuilder::new(vote_state).make_keyboard();
    let keyboard = transforms::point_story::to_tg_keyboard(&keyboard);
    return keyboard;
}
