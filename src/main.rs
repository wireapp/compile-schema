use std::ffi::OsString;

use anyhow::{Context, Result};
use clap::Parser;
use rusqlite::functions::FunctionFlags;
use sha2::{Digest, Sha256};

/// Compile a SQL schema from a set of migrations
#[derive(Debug, Parser)]
struct Args {
    /// Path to the migrations directory
    migrations: OsString,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let migrations =
        refinery::load_sql_migrations(&args.migrations).context("loading migrations")?;
    let runner = refinery::Runner::new(&migrations);
    let mut sqlite = rusqlite::Connection::open_in_memory().context("opening in-memory sqlite")?;
    sqlite
        .create_scalar_function(
            "sha256_blob",
            1,
            FunctionFlags::SQLITE_DETERMINISTIC | FunctionFlags::SQLITE_INNOCUOUS,
            |ctx| {
                let input_blob = ctx.get::<Vec<u8>>(0)?;
                let mut hasher = Sha256::new();
                hasher.update(&input_blob);
                Ok(hex::encode(hasher.finalize()))
            },
        )
        .context("creating sha256_blob function")?;
    runner.run(&mut sqlite).context("running migrations")?;
    let mut stmt = sqlite
        .prepare(
            "SELECT sql FROM sqlite_schema WHERE sql IS NOT NULL AND name NOT LIKE 'refinery_%'",
        )
        .context("preparing schema query")?;
    for row in stmt
        .query_map([], |row| row.get::<_, String>("sql"))
        .context("executing query")?
    {
        let sql = row.context("loading row from query results")?;
        let formatted = sqlformat::format(
            &sql,
            &Default::default(),
            &sqlformat::FormatOptions {
                uppercase: Some(true),
                ..Default::default()
            },
        );
        println!("{formatted};\n");
    }
    Ok(())
}
