use anyhow::Error;
use fehler::throws;
use sled::Db;

#[throws]
pub async fn get_state(db: Db) {
    db.transaction::<_, _, sled::Error>(|tx_db| {
        let locked = tx_db.get(b"locked")?.unwrap_or_default() == b"true";
        let res = tx_db.get(b"res")?.unwrap_or_default() == b"true";
        println!("{}\n{}", locked, res);
        Ok(())
    })?;
}
