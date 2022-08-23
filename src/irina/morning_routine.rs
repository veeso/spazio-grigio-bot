//! # Morning routine
//!
//! Contains the video for a buongiorno minimalista

use rand::Rng;

pub struct MorningRoutine;

impl MorningRoutine {
    /// Get a random morning routine video
    pub fn get_random() -> &'static str {
        let mut rng = rand::thread_rng();
        MORNING_ROUTINE_VIDEOS[rng.gen_range(0..MORNING_ROUTINE_VIDEOS.len())]
    }
}

const MORNING_ROUTINE_VIDEOS: &[&str] = &[
    "https://www.youtube.com/watch?v=rRQP8PNEouo",
    "https://www.youtube.com/watch?v=zuJ6rWQ_2vE",
    "https://www.youtube.com/watch?v=-eOJWZCYJV0",
    "https://www.youtube.com/watch?v=ZEHVgvLAv6Q",
    "https://www.youtube.com/watch?v=tMZmKRk54bQ",
    "https://www.youtube.com/watch?v=5IDjxQKCUGY",
    "https://www.youtube.com/watch?v=iB5aW-csDiU",
    "https://www.youtube.com/watch?v=-eOJWZCYJV0",
    "https://www.youtube.com/watch?v=ZEHVgvLAv6Q",
    "https://www.youtube.com/watch?v=_uN7hvoZdmE",
    "https://www.youtube.com/watch?v=55RyTo4U818",
];
