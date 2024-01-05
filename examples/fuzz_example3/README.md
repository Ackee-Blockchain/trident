# Fuzz Test Example for Trdelnik Fuzzer

---

## Program and Errors Description

- ### Initialize
    - Within the Initialize instruction, we initialize a new Escrow Transaction. The entered amount (as instruction input) is locked within the Escrow Token Account, for a specified amount of time (also as instruction input). Later, this amount can be partially unlocked or fully unlocked based on the current time during Withdrawal.

- ### Withdraw
    - Within the Withdrawal function, we can unlock the locked amount. The condition is that the eligible recipient stored as Recipient PubKey within the Escrow Account has to sign the Transaction.
    - ❗ **NOTE:** 🐛 One issue is that within the **amount_unlocked function**, we do not correctly compute the unlocked amount, resulting in:
        - In some cases the receiver **🚨can withdraw less🚨** than it was initially meant, based on the fact that integer arithmetic is used, so everything is rounded down.
        - As multiple Escrows share one Token Account per Mint, it is also **🚨possible to withdraw more🚨** than it was initially meant to.
        - The consequence of the previous point is if the **unlocked_amount** is greater than the initially locked amount and the Token Account has insufficient balance. **🚨The Recipient will not be able to withdraw any funds🚨**.
        ```rust
        todo!()
        // finish fuzz test where hacker can steal funds
        // example inside anchor test
        ```


## Fuzz Test Checks
- ### ✔️Withdraw Instruction checks
We first check that the Escrow Account was initialized before the Instruction call, if so we can read the Recipient Public Key from the Account.
```rust
if let Some(escrow) = pre_ix.escrow
let recipient = pre_ix.recipient.unwrap();
```
Next, we check that the Recipient Token Account was already initialized before the Instruction call and also unwrap the Token Account after the Instruction.
```rust
if let Some(recepient_token_account_pre) = pre_ix.recipient_token_account
if let Some(recepient_token_account_post) = post_ix.recipient_token_account
```
In order to check Account lamports, we need to also check that the Signer corresponds to the Recipient stored within the Escrow Account.
```rust
if escrow.recipient == *recipient.key
```
- #### Branch 1️⃣
We check that the Recipient's balance within his Token Account did not change.
```rust
if recepient_token_account_pre.amount
    == recepient_token_account_post.amount
```
If so, he was not able to withdraw the locked amount thus we found the 🚨Error🚨
```rust
return Err("Recipient was not able to withdraw any funds");
```
- #### Branch 2️⃣
The second branch means that the Recipient was actually able to withdraw something, but not the expected amount.
```rust
else if recepient_token_account_pre.amount + escrow.amount
    != recepient_token_account_post.amount
```
- ##### Option🅰️
We check if he actually withdrew **Less**
```rust
if recepient_token_account_pre.amount + escrow.amount
> recepient_token_account_post.amount
```
If so, we found the 🚨Error🚨
```rust
// ...
// "Amount Mismatch (Recipient withdrawn LESS) by: ...
// ...
return Err("Transfered amount mismatch");
```
- ##### Option🅱️
On the other hand, if the previous check did not pass, he was able to actually withdraw **MORE** than initially meant to, so we again found the 🚨Error🚨.
```rust
// ...
// "Amount Mismatch (Recipient withdrawn MORE) by: ...
// ...
return Err("Transfered amount mismatch");
```
<u> Final Check </u>
```rust
if let Some(escrow) = pre_ix.escrow {

    let recipient = pre_ix.recipient.unwrap();

    if let Some(recepient_token_account_pre) = pre_ix.recipient_token_account {

        if let Some(recepient_token_account_post) = post_ix.recipient_token_account {

            if escrow.recipient == *recipient.key {

                if recepient_token_account_pre.amount == recepient_token_account_post.amount{

                    return Err("Recipient was not able to withdraw any funds");

                } else if recepient_token_account_pre.amount + escrow.amount != recepient_token_account_post.amount{
                    if recepient_token_account_pre.amount + escrow.amount > recepient_token_account_post.amount{

                        eprintln!("Amount Mismatch (Recipient withdrawn LESS) by: {}",(recepient_token_account_pre.amount + escrow.amount) - recepient_token_account_post.amount);

                    } else {

                        eprintln!("Amount Mismatch (Recipient withdrawn MORE) by: {}",recepient_token_account_post.amount - (recepient_token_account_pre.amount + escrow.amount));

                    }

                    eprintln!("Before: {}", recepient_token_account_pre.amount);
                    eprintln!("After: {}", recepient_token_account_post.amount);
                    eprintln!("Expected: {}",recepient_token_account_pre.amount + escrow.amount);
                    return Err("Transfered amount mismatch");
                }
            }
        }
    }
}
```
