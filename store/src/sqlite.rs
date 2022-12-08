use std::fs;
use std::error::Error;
use rusqlite::{Connection, Statement, Result, params};
// use rusqlite::config::DbConfig;



#[derive(Debug)]
pub struct SQLiteContext<'a> {
    // pub config: Config,
    pub conn: &'a Connection, // Maybe initialize this in "new" method | maybe rename "new" to "init"...

    pub init_db_stmt:       Option<Statement<'a>>, // Don't really need this stmt-- just for practice, I guess.
    pub insert_zknote_stmt: Option<Statement<'a>>,
    pub insert_zklink_stmt: Option<Statement<'a>>,

    pub select_note_by_name_and_string_stmt: Option<Statement<'a>>,
}

impl<'a> SQLiteContext<'a> {
    pub fn new(conn: &'a Connection) -> Result<Self> {
        Ok(
            SQLiteContext {
                // config,
                conn,

                init_db_stmt:       None,
                insert_zknote_stmt: None,
                insert_zklink_stmt: None,

                select_note_by_name_and_string_stmt: None,
            }
        )
    }
}

// ?: Make sure errors are properly handled here...
impl<'a> SQLiteContext<'a> {
    /**
     * Context Initialization
     */
    pub fn init(&mut self) -> Result<()> {

        /* config stuff */

        if let None = &self.init_db_stmt {
            let sql_file_contents = read_sql_from_file("src/statements/init_db.sql");
            let stmt = self.conn.prepare(&sql_file_contents)?;

            self.init_db_stmt = Some(stmt);
        }

        self.init_db_stmt.as_mut().unwrap().execute([])?;

        Ok(())
    }

    /* --------- */
    /* Inserting */
    /* --------- */

    /**
     * insert_zknote
     */
    pub fn insert_zknote() -> ! {
        todo!()
    }

    /**
     * insert_zklink
     */
    pub fn insert_zklink(from_id: i64, to_id: i64, link_zknote: Option<i64>) -> ! {
        todo!()
    }


    /* --------- */
    /* Selecting */
    /* --------- */

    /**
     * Note Selection
     */
    pub fn select_note_by_name_and_string(&mut self, name: &str, title: &str) -> Result<i64, Box<dyn Error>> {
        if let None = &self.select_note_by_name_and_string_stmt {
            let sql_file_contents = read_sql_from_file("src/statements/select_note_by_name_and_string.sql");
            let stmt = self.conn.prepare(&sql_file_contents)?;

            self.select_note_by_name_and_string_stmt = Some(stmt);
        }

        let id: i64 = self.select_note_by_name_and_string_stmt.as_mut().unwrap().query_row(
            params![name, title], |row| Ok(row.get(0)?),
        )?;

        Ok(id)
    }
}

/* ---------------- */
/* Helper Functions */
/* ---------------- */

/**
 * read_sql_from_file
 */
// ?: Should "path" be std::path::Path or &str?
// ?: For now, we're not going to be "reading as buffer" because these SQL files are small.
fn read_sql_from_file(path: &str) -> String {
    fs::read_to_string(path).expect(r#"Failed to read SQL file."#)
}