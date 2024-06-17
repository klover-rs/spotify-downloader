use rusqlite::{params, Connection};
use anyhow::{anyhow, Result};

use super::{get_current_user, get_database_path};

use bcrypt::{DEFAULT_COST, hash, verify};
use keyring::Entry;

pub fn store_creds(username: &str, password: &str) -> Result<()> {

    let db_path = get_database_path()?;

    let mut conn = Connection::open(db_path)?;

    let trans = conn.transaction()?;

    trans.execute("CREATE TABLE IF NOT EXISTS creds (
        key INTEGER PRIMARY KEY,
        username TEXT,
        password TEXT
    )", [])?;

    let hashed_pw = hash(&password, DEFAULT_COST)?;

    trans.execute("INSERT OR REPLACE INTO creds (
        username,
        password
    ) VALUES (?1, ?2)", [&username, &hashed_pw.as_str()])?;

    store_in_keyring(&password)?;
    
    trans.commit()?;

    Ok(())
    
}

pub fn get_creds() -> Result<(String, String)> {
    let db_path = get_database_path()?;

    let conn = Connection::open(db_path)?;

    conn.execute("CREATE TABLE IF NOT EXISTS creds (
        key INTEGER PRIMARY KEY,
        username TEXT,
        password TEXT
    )", [])?;

    let mut stmt = conn.prepare("SELECT * FROM creds")?;
    
    let rows = stmt.query_map(params![], |row| {
        Ok((
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    let mut all_rows = Vec::new();

    for row in rows {
        all_rows.push(row?);
    }

    if all_rows.is_empty() {
        return Err(anyhow!("no rows were returned from the db"));
    } else {
        let (username, password_hash) = &all_rows[0];

        let password = get_pw_from_keyring()?;

        let is_valid = verify(&password, &password_hash)?;

        if is_valid {
            return Ok((username.to_string(), password));
        } else {
            return Err(anyhow!("the stored password in the keyring does not match the hash of the pw in sqlite"))
        }

    }

    
}



fn store_in_keyring(password: &str) -> Result<()> {

    let current_user = get_current_user()?;

    let entry = Entry::new("spotify-dler", &current_user)?;

    entry.set_password(&password)?;

    Ok(())
}

fn get_pw_from_keyring() -> Result<String> {
    let current_user = get_current_user()?;

    let entry = Entry::new("spotify-dler", &current_user)?;

    let pw = entry.get_password()?;

    Ok(pw)
}
