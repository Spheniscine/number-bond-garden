use std::ops::RangeInclusive;

// colors assigned to each number, and the contrasting text color. 
pub const ORB_COLORS: [(&str, &str); 11] = [
    ("#aaaaaa", "#000"), // 0, unused
    ("#da1918", "#fff"), // 1
    ("#f4641d", "#000"), // 2
    ("#e7e52c", "#000"), // 3
    ("#31b32b", "#fff"), // 4
    ("#21b19b", "#000"), // 5
    ("#1f87ff", "#fff"), // 6
    ("#a020f0", "#fff"), // 7
    ("#f570ce", "#000"), // 8
    ("#cccccc", "#000"), // 9
    ("#966919", "#fff"), // 10
];

pub const BOARD_RADIUS: u32 = 5;
pub const NUM_DUPES: usize = 6;
pub const NUM_ORBS: usize = 9 * NUM_DUPES + 1;
pub const INITIAL_FREE_ORB_RANGE: RangeInclusive<usize> = 4..=15;