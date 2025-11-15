use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;

use crate::trident::Trident;

pub struct AddressStorage {
    addresses: Vec<Pubkey>,
}

pub struct PdaSeeds<'a> {
    pub seeds: &'a [&'a [u8]],
    pub program_id: Pubkey,
}

impl<'a> PdaSeeds<'a> {
    pub fn new(seeds: &'a [&'a [u8]], program_id: Pubkey) -> Self {
        Self { seeds, program_id }
    }
}

fn derive_pda(seeds: &[&[u8]], program_id: &Pubkey) -> Option<Pubkey> {
    if let Some((address, _)) = Pubkey::try_find_program_address(seeds, program_id) {
        Some(address)
    } else {
        None
    }
}

impl Default for AddressStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressStorage {
    fn new() -> Self {
        let addresses: Vec<Pubkey> = Vec::new();
        Self { addresses }
    }

    pub fn insert(&mut self, trident: &mut Trident, seeds: Option<PdaSeeds>) -> Pubkey {
        let address = self.get_or_create_address(seeds, trident);
        self.addresses.push(address);
        address
    }

    pub fn insert_with_address(&mut self, address: Pubkey) {
        self.addresses.push(address);
    }

    pub fn get(&self, trident: &mut Trident) -> Pubkey {
        let accounts_num = self.addresses.len();

        let account_id = trident.random_from_range(0..accounts_num);
        self.addresses[account_id]
    }

    pub fn is_empty(&self) -> bool {
        self.addresses.is_empty()
    }
    pub fn len(&self) -> usize {
        self.addresses.len()
    }

    fn get_or_create_address(&self, seeds: Option<PdaSeeds>, trident: &mut Trident) -> Pubkey {
        match seeds {
            Some(seeds) => {
                if let Some(pubkey) = derive_pda(seeds.seeds, &seeds.program_id) {
                    pubkey
                } else {
                    let mut secret = [0; 32];
                    trident.random_bytes(&mut secret);
                    solana_sdk::signer::keypair::Keypair::new_from_array(secret).pubkey()
                }
            }
            None => {
                let mut secret = [0; 32];
                trident.random_bytes(&mut secret);
                solana_sdk::signer::keypair::Keypair::new_from_array(secret).pubkey()
            }
        }
    }
}
