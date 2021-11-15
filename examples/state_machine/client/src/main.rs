use anyhow::Error;
use fehler::throws;
use structopt::StructOpt;

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
    println!("{:?}", opt);

    match opt {
        Opt::Init => command::init().await?,
        Opt::Coin => command::coin().await?,
        Opt::Push => command::push().await?,
        Opt::GetState => command::get_state().await?,
    }
}
