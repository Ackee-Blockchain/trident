1. Use Linux or Mac or Windows WSL
1. Install Python
1. Install pip
1. Install Node.js
1. Install Yarn
1. Install Rust
1. Install Solana CLI
1. `export PATH=$HOME/.local/bin:$PATH`
1. `yarn`
1. `pip install hypothesis`
1. `pip install pytest`
1. `anchor build`
1. `solana-test-validator -C ./config.yml` (add `-r` for reset)
1. `solana airdrop -C ./config.yml 1 ./keys/id.json`
1. `solana program deploy -C ./config.yml --program-id ./keys/program.json ./target/deploy/turnstile.so`
1. `ANCHOR_WALLET=./keys/id.json node ./js_client/init.js`
1. `cargo build` in `./client`
1. `pytest turnstile.py`
