# System Program Methods

The System Program methods provide functionality for working with Solana's system program in your fuzz tests. These methods allow you to create accounts, allocate space, assign ownership, and transfer SOL.

!!! warning "Signature Verification and PDAs"

    Trident does not perform signature verification for performance reasons and relies on the specified account metas. This means that operations that would normally require a signature (like `allocate`, `assign`, or `transfer`) will succeed in Trident even when called on Program Derived Addresses (PDAs), which do not have private keys and cannot sign transactions off-chain.
    
    In production, these operations would fail when attempted on PDAs outside of an on-chain program invocation. When testing, be aware that Trident's behavior differs from on-chain execution in this regard.

## Account Creation and Management

### `create_account`

Creates a new account with specified space and owner.

```rust
pub fn create_account(
    &mut self,
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
) -> Instruction
```

**Parameters:**

- `from_pubkey` - The public key of the account funding the new account
- `to_pubkey` - The public key of the new account to create
- `lamports` - The number of lamports to transfer to the new account
- `space` - The number of bytes to allocate for the account data
- `owner` - The program that will own the new account

**Returns:** An `Instruction` that needs to be executed with `process_transaction`.

**Description:** Generates a system program create_account instruction to allocate space and assign ownership of a new account in a single transaction.

---

### `allocate`

Allocates space for an existing account.

```rust
pub fn allocate(&mut self, address: &Pubkey, space: u64) -> Instruction
```

**Parameters:**

- `address` - The public key of the account to allocate space for
- `space` - The number of bytes to allocate

**Returns:** An `Instruction` that needs to be executed with `process_transaction`.

**Description:** Generates a system program allocate instruction to allocate the specified number of bytes for an account's data. The account must be owned by the system program and have sufficient lamports to be rent-exempt at the new size.

!!! warning "PDA Behavior"

    This method will succeed on PDAs in Trident, but would fail on-chain when called outside of a program invocation, since PDAs cannot sign transactions.

---

### `assign`

Assigns an account to a program (changes ownership).

```rust
pub fn assign(&mut self, address: &Pubkey, owner: &Pubkey) -> Instruction
```

**Parameters:**

- `address` - The public key of the account to assign
- `owner` - The public key of the program that will own the account

**Returns:** An `Instruction` that needs to be executed with `process_transaction`.

**Description:** Generates a system program assign instruction to change the owner of an account to the specified program. The account must be owned by the system program before assignment.

!!! warning "PDA Behavior"

    This method will succeed on PDAs in Trident, but would fail on-chain when called outside of a program invocation, since PDAs cannot sign transactions.

---

## SOL Transfer

### `transfer`

Transfers SOL from one account to another.

```rust
pub fn transfer(&mut self, from: &Pubkey, to: &Pubkey, amount: u64) -> Instruction
```

**Parameters:**

- `from` - The public key of the account to transfer from
- `to` - The public key of the account to transfer to
- `amount` - The number of lamports to transfer

**Returns:** An `Instruction` that needs to be executed with `process_transaction`.

**Description:** Generates a system program transfer instruction to move the specified amount of lamports from the source to destination account.

!!! warning "PDA Behavior"

    This method will succeed when transferring from a PDA in Trident, but would fail on-chain when called outside of a program invocation, since PDAs cannot sign transactions.

---

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn test_system_program(&mut self) {
    let payer = self.payer().pubkey();
    let new_account = Keypair::new();
    let program_id = Pubkey::new_unique();
    
    // Create a new account with 1000 lamports and 100 bytes of space
    let ix = self.create_account(
        &payer,
        &new_account.pubkey(),
        1000,
        100,
        &program_id,
    );
    let result = self.process_transaction(&[ix], Some("create_account"));
    assert!(result.is_success());
    
    // Transfer SOL to another account
    let recipient = Pubkey::new_unique();
    let ix = self.transfer(&payer, &recipient, 500);
    let result = self.process_transaction(&[ix], Some("transfer"));
    assert!(result.is_success());
    
    // Allocate more space for an existing account
    let ix = self.allocate(&new_account.pubkey(), 200);
    let result = self.process_transaction(&[ix], Some("allocate"));
    assert!(result.is_success());
    
    // Assign account to a different program
    let new_owner = Pubkey::new_unique();
    let ix = self.assign(&new_account.pubkey(), &new_owner);
    let result = self.process_transaction(&[ix], Some("assign"));
    assert!(result.is_success());
}
```
