use rusqlite::{Connection, Result};

use yle::categories::{
    RIKOS,
    NUORET,
    DRAAMASARJAT,
    REALITY,
    KOMEDIASARJAT,
    SEKALAISET,
    DOKUMENTIT,
    LUONTO,
    HISTORIA,
};

fn main() -> Result<()> {

    let connection = Connection::open("programs.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            token TEXT DEFAULT NULL
        )",
        (),
    )?;

    insert_categories(&connection, RIKOS)?;
    insert_categories(&connection, NUORET)?;
    insert_categories(&connection, DRAAMASARJAT)?;
    insert_categories(&connection, REALITY)?;
    insert_categories(&connection, KOMEDIASARJAT)?;
    insert_categories(&connection, SEKALAISET)?;
    insert_categories(&connection, DOKUMENTIT)?;
    insert_categories(&connection, LUONTO)?;
    insert_categories(&connection, HISTORIA)?;

    Ok(())
}

fn insert_categories(connection: &Connection, categories: &[(&str, &str)]) -> Result<()> {
    for (name, _) in categories {
        connection.execute(
            "INSERT INTO categories (name, token) VALUES (?1, NULL)",
            [name],
        )?;
    }
    Ok(())
}