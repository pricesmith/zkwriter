use rusqlite::{Connection, Statement, Result};
// use rusqlite::config::DbConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SQLiteContext {
    // pub config: Config,
    pub conn: &Connection, // Maybe initialize this in "new" method | maybe rename "new" to "init"...

    pub init_db_stmt:       Option<Statement>, // Don't really need this stmt-- just for practice, I guess.
    pub insert_zknote_stmt: Option<Statement>,
    pub insert_zklink_stmt: Option<Statement>,
}

impl SQLiteContext {
    pub fn new(conn: &Connection) -> Self {
        SQLiteContext {
            // config,
            conn,

            init_db_stmt:       None,
            insert_zknote_stmt: None,
            insert_zklink_stmt: None,
        }
    }
}
