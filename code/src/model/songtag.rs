#[derive(Debug, Clone)]
pub struct SongTag {
    pub file_path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub track: Option<i32>,
    pub year: Option<i32>,
}