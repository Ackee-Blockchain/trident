use anyhow::Error;
use fehler::throws;
use sled::Db;

#[throws]
pub async fn coin(db: Db) {
    db.transaction::<_, _, sled::Error>(|tx_db| {
        tx_db.insert(b"locked", b"false")?;
        tx_db.insert(b"res", b"true")?;
        Ok(())
    })?;
}
