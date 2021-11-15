use anyhow::Error;
use fehler::throws;
use sled::Db;

#[throws]
pub async fn push(db: Db) {
    db.transaction::<_, _, sled::Error>(|tx_db| {
        if tx_db.get(b"locked")?.unwrap_or_default() == b"true" {
            tx_db.insert(b"locked", b"true")?;
            tx_db.insert(b"res", b"false")?;
        } else {
            tx_db.insert(b"locked", b"true")?;
            tx_db.insert(b"res", b"true")?;
        }
        Ok(())
    })?;
}
