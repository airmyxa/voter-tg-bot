pub static POINTS: &'static [&'static str; 8] = &[
    "0", "1", "2", "3", "5", "8", "13", "21"
];

pub static ACTIONS: &'static [&'static str; 2] = &[
    "Open",
    "Dismiss"
];

pub static LOGICAL_KEYBOARD_BLOCKS: (&'static [&'static str; 8], &'static [&'static str; 2]) = (
    POINTS, ACTIONS
);

pub static ROW_SIZE: usize = 5;
