use anyhow::Error;
use fehler::throws;
use structopt::StructOpt;
use sled;

mod command;

#[derive(Debug, StructOpt)]
enum Opt {
    Init,
    Coin,
    Push,
    GetState,
}

#[throws]
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);

    let db = sled::open("./mocked_state").expect("failed to open DB 'mocked_state'");

    match opt {
        Opt::Init => command::init(db).await?,
        Opt::Coin => command::coin(db).await?,
        Opt::Push => command::push(db).await?,
        Opt::GetState => command::get_state(db).await?,
    }
}
