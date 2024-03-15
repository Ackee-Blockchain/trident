# Fuzz Test Example for Trident Fuzzer

---

## Program and Errors Description

- ### Initialize
    In this function, we initialize an Escrow Account and lock the entered amount within the Escrow. The Receiver's PubKey is stored within the Account.

- ### Withdraw
    - The Receiver, meant to withdraw funds from the Escrow, can call this Instruction. It will close the Escrow Account and transfer all funds (locked amount + rent) to the Receiver.
    - ❗ **NOTE:** 🐛 We do not correctly perform a check to ensure that the **Signer** is the corresponding receiver in the Escrow Account, **resulting in the 🚨unauthorized withdrawal🚨**.

## Fuzz Test Checks
- ### ✔️Withdraw Instruction Check
We first check if the Escrow Account was initialized before the execution of the Instruction.
```rust
if let Some(escrow_pre) = pre_ix.escrow {
    //...
}
```
Then, we extract the Before/After balance of the Receiver (i.e., Signer).
```rust
let receiver = pre_ix.receiver;
let receiver_lamports_before = receiver.lamports();
let receiver_lamports_after = post_ix.receiver.lamports();
```
Finally, we perform a check to verify that the Receiver (i.e., Signer) and the stored Receiver within the Escrow Account do not match, and that the Signer's balance increased.
```rust
if receiver.key() != escrow_pre.receiver
    && receiver_lamports_before < receiver_lamports_after {
        //...
    }
```
If these conditions are met, we have identified the 🚨Error🚨.
```rust
return Err(FuzzingError::BalanceMismatch);
```

<u> Final Check </u>
```rust
if let Some(escrow_pre) = pre_ix.escrow {
    let receiver = pre_ix.receiver;
    let receiver_lamports_before = receiver.lamports();
    let receiver_lamports_after = post_ix.receiver.lamports();
    if receiver.key() != escrow_pre.receiver
        && receiver_lamports_before < receiver_lamports_after {
            return Err(FuzzingError::BalanceMismatch);
    }
}
```
