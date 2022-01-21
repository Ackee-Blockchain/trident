use trdelnik::*;

pub struct Turnstile;

impl Turnstile {
    pub const ID: Pubkey = Pubkey::new_from_array([216u8, 55u8, 200u8, 93u8, 189u8, 81u8, 94u8, 109u8, 14u8, 249u8, 244u8, 106u8, 68u8, 214u8,222u8, 190u8, 9u8, 25u8, 199u8, 75u8, 79u8, 230u8, 94u8, 137u8, 51u8, 187u8, 193u8, 48u8, 87u8, 222u8, 175u8, 163u8]);

    // let payer = reader.keypair("id").await?;
    // let state = reader.keypair("state").await?;
    // turnstile::accounts::Initialize { 
    //     state: state.pubkey(),
    //     user: payer_pubkey,
    //     system_program: System::id()
    // },
    // signers: Some(state),
    pub async fn initialize(
        payer: Keypair, 
        a_state: Pubkey, 
        a_user: Pubkey, 
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<Signature, ClientError> {
        Ok(Client::new(payer).send_instruction(
            Self::ID,
            turnstile::instruction::Initialize,
            turnstile::accounts::Initialize { 
                state: a_state,
                user: a_user,
                system_program: a_system_program,
            },
            signers,
        ).await?)
    }

    // let payer = reader.keypair("id").await?;
    // dummy_arg: "something".to_owned() 
    // state: reader.pubkey("state").await?
    // signers: None
    pub async fn coin(
        payer: Keypair,
        i_dummy_arg: String,
        a_state: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<Signature, ClientError> {
        Ok(Client::new(payer).send_instruction(
            Self::ID,
            turnstile::instruction::Coin { 
                dummy_arg: i_dummy_arg, 
            },
            turnstile::accounts::UpdateState { 
                state: a_state,
            },
            signers,
        ).await?)
    }

     // let payer = reader.keypair("id").await?;
     // state: reader.pubkey("state").await?
     // signers: None
    pub async fn push(
        payer: Keypair,
        a_state: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<Signature, ClientError> {
        Ok(Client::new(payer).send_instruction(
            Self::ID,
            turnstile::instruction::Push,
            turnstile::accounts::UpdateState { 
                state: a_state,
            },
            signers,
        ).await?)
    }
}
