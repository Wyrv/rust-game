
pub struct Letter{
    pub character: char,
    pub revealed: bool
}

pub enum GameProgress{
    InProgress,
    Won,
    Lost
}