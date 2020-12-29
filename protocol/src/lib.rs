#[macro_use]
extern crate serde_derive;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchScheduleEntry {
    pub max_actors: u32,
    pub board_height: u32,
    pub board_width: u32,
    pub max_turns: u32,
}