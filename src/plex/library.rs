#[derive(Debug, Clone)]
pub struct Library {
    pub id: String,
    pub name: String,
    pub library_type: String,

    pub duration_total: i64,
    pub storage_total: i64,
    pub item_count: i64,
}

pub fn is_library_directory_type(directory_type: &str) -> bool {
    matches!(directory_type, "movie" | "show" | "artist")
}
