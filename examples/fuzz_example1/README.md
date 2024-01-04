# Fuzz Test Example for Trdelnik Fuzzer

---

## Program and Errors Description

- ### Initialize
    - Within this function we initialize a new common State, under which we can later register new Subjects, let's say Projects.
    - Important variables inside the State account are:

        - registrations_round: this option tells us if the Registration round is still open.
        - **NOTE:** However it is not correctly set inside the Initialization, so it is implicitly set as **FALSE**.

- ### Register
    - Register Project under specified State.
    - **NOTE:** It is important to notice here that we are not correctly checking if the registration window is still open.

- ### End Registrations
    - Halts Project Registrations, meaning, flips **registrations_round** to false.

- ### Invest
    - Participants can Invest in a Project of their choice. Even though we performed a check if registration_round is still open:
    ```rust
    require!(
        !state.registrations_round,
        CustomError::RegistrationRoundOpen
    );
    ```
    - **NOTE:** Fuzz Test can discover the instruction sequence where **Invest** was successfully performed even though **End Registration** was not called before.


## Fuzz Test Checks
- ### Register Instruction check
We first check if the State is initialized before the Register instruction call
 ```rust
 if let Some(state) = pre_ix.state
 ```
 if so we then check if the Project was successfully created
 ```rust
 if let Some(_project) = post_ix.project
 ```
 Lastly, if both previous checks have passed, we then check if **registrations_round** within the State was false
```rust
if !registrations_round
```
If so, we found the Error
```rust
return Err("We succesfully registered new project even though registrations are not open");
```
Final Check
```rust
if let Some(state) = pre_ix.state {
    if let Some(_project) = post_ix.project {
        let registrations_round = state.registrations_round;
        if !registrations_round {
            return Err("We succesfully registered new project even though registrations are not open");
        }
    }
}
```

---

- ### Invest Instruction check
We first check if the Project was already registered before the Invest instruction call
```rust
if let Some(project_pre) = pre_ix.project
```
if so, we then check if the State was already initialized
```rust
if let Some(state) = pre_ix.state
```
Lastly, we check if the **invested amount before and after the instruction call has changed**
```rust
if !state.registrations_round && project_pre.invested_amount + ix_data.amount == project_post.invested_amount
```
If so, we found the Error
```rust
return Err("Registration round was not terminated, however investor was able to invest inside registration window");
```
Final Check
```rust
if let Some(project_pre) = pre_ix.project {
    let project_post = post_ix.project.unwrap();
    if let Some(state) = pre_ix.state {
        if !state.registrations_round
            && project_pre.invested_amount + ix_data.amount
            == project_post.invested_amount{
            return Err("Registration round was not terminated, however investor was able to invest inside registration window");
        }
    }
}
```
