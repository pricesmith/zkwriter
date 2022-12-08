use rusqlite::{Connection, Result};


pub fn init_mem_store() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open_in_memory();
    let &mut ctx = SQLiteContext::new(&conn);

    //
    // Do anything else here...
    //

    return ctx;
}

pub fn save_zklink() -> Result<i64, Box<dyn Error>> {

}