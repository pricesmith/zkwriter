// ?: Why do I not have to redeclare miniserde here?
use crate::note::{ZKNote, ZKLinkNote};

// ?: If these are the same, do they need to be declared twice?

#[derive(Serialize, Debug, Clone)]
pub struct ZKNoteSearchResult {
    pub notes: Vec<ZKNote>,
    // offset,
    // what,
}

#[derive(Serialize, Debug, Clone)]
pub struct ZKLinkNoteSearchResult {
    pub notes: Vec<ZKLinkNote>,
    // offset,
    // what,
}

trait SearchZKNote {
    fn search_zknote(query: String) -> ZKNoteSearchResult;
}

trait SearchZKLinkNote {
    fn search_zklink_note(query: String) -> ZKLinkNoteSearchResult;
}