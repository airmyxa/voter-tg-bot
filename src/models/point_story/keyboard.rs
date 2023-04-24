pub static POINTS: &'static [&'static str] = &["0", "1", "2", "3", "5", "8", "13", "21"];

pub static ACTIONS: &'static [&'static str] = &["Open", "Dismiss"];

pub static LOGICAL_KEYBOARD_BLOCKS: &'static [&'static [&'static str]] = &[POINTS, ACTIONS];

pub static ROW_SIZE: usize = 5;

type KeyRow = Vec<String>;

pub struct Keyboard {
    pub key_rows: Vec<KeyRow>,
}
