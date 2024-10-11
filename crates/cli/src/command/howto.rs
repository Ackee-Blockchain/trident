use anyhow::Error;
use fehler::throws;

use crate::show_howto;

#[throws]
pub fn howto() {
    show_howto();
}
