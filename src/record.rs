use crate::vinyl_format;

struct Record {
    name: String,
    year: u16,
    artist: String,
    country_of_origin: String,
    tags: Vec<String>,
    format: VinylFormat

}

impl Record {
    fn is_easy_listening(&self) -> bool {
        self.tags.iter().any(|x|  x == "jazz" )
    }

    // TODO: explore ordering
    fn is_before(&self, other: &Record) -> bool {
        self.year < other.year
    } 
}