use rusqlite::{Connection, Result};

fn main() -> Result<()> {

    let connection = Connection::open("programs.db")?;

    let mut statement = connection.prepare("SELECT name, token FROM categories")?;

    let rows = statement.query_map([], |row| {
        Ok((
             row.get::<_, String>(0)?,
            row.get::<_, Option<String>>(1)?,
        ))
    })?;

    for row in rows {
        let (name, token) = row?;
        println!("{:?} {:?}", name, token );
    }

    Ok(())
}