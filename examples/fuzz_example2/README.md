# Fuzz Test Example for Trdelnik Fuzzer

---

## Program and Errors Description

- ### Initialize
    Within this function, we Initialize an Escrow Account and lock the entered amount within the Escrow, Receiver PubKey is stored within the Account.
- ### Withdraw
    - Receiver meant to withdraw funds from Escrow can call this Instruction. It will close the Escrow Account and transfer all funds (locked amount + rent) to the Receiver.
    - ❗ **NOTE:** 🐛 We do not correctly perform a check that the **Signer** is the corresponding receiver in the Escrow Account, **resulting in the 🚨unauthorized withdrawal🚨**.

## Fuzz Test Checks
- ### ✔️Withdraw Instruction check
We first check if the Escrow Account was initialized before the Instruction execution
```rust
if let Some(escrow_pre) = pre_ix.escrow
```
if so we extract the Receiver from the Escrow Account and the Before/After balance of the Receiver (i.e. Signer).
 ```rust
let receiver = pre_ix.receiver.unwrap();
let receiver_lamports_before = receiver.lamports();
let receiver_lamports_after = post_ix.receiver.unwrap().lamports();
```
Lastly, we perform a check that the Receiver (i.e. Signer) and stored Receiver within the Escrow Account do not match and that the Signer balance increased.
```rust
if receiver.key() != escrow_pre.receiver.key()
    && receiver_lamports_before < receiver_lamports_after
```
If so, we found the 🚨Error🚨
```rust
return Err("Un-authorized withdrawal");
```
<u> Final Check </u>
```rust
if let Some(escrow_pre) = pre_ix.escrow {
    let receiver = pre_ix.receiver.unwrap();
    let receiver_lamports_before = receiver.lamports();
    let receiver_lamports_after = post_ix.receiver.unwrap().lamports();
    if receiver.key() != escrow_pre.receiver.key()
        && receiver_lamports_before < receiver_lamports_after{
            return Err("Un-authorized withdrawal");
    }
}
```
