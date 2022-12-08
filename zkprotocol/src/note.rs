use miniserde::{Serialize, Deserialize};

// ?: Why is it asking to convert to tuple struct here?
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZKNote {
    pub id:                 i64,            // ?: is uid?
    pub title:              String,
    pub content:            String,
    pub editable:           bool,
    pub editable_values:    bool,           // ?: <-- what is
    pub show_title:         bool,
    pub pub_id:             Option<String>,
    pub create_date:        i64,
    pub change_date:        i64,
    pub deleted:            bool,
    pub is_file:            bool,
    pub sys_ids:            Vec<i64>        // ?: hmm...
    // ?: what is usernote?
}

// ?: Does this work?
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZKLinkNote {             // ?: Is there a way to inherit these fields rather than redeclare?
    pub id:             i64,        // ZKNote::id,
    pub title:          String,     // ZKNote::title,
    pub is_file:        bool,       // ZKNote::is_file,
    pub create_date:    i64,        // ZKNote::create_date,
    pub change_date:    i64,        // ZKNote::change_date,
    pub sys_ids:        Vec<i64>,   // ZKNote::sys_ids,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Direction {
    From,
    To,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZKLink {
    pub from:   i64,
    pub to:     i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZkLinks {
    pub links: Vec<ZkLink>,
}

// Misc: Scion is a cool word to use for parent/child/descendant/etc relations.
// scion -- a descendant of a notable family -- GET IT?!
//
// Anyway, other words for parent/child/etc:
// -- descendant, successor, issue, offshoot
// -- precursor, source, origin, originator, root, predecessor, anticedent, radix (but that could get confusing).