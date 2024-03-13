# Fuzz Test Example for Trident Fuzzer

---

## Program and Errors Description

- ### Initialize
    - In this function, we initialize a new common State, under which we can later register new Subjects, such as Projects.
    - An important variable inside the State account is:
        - `registrations_round`: this option indicates if the Registration round is still open.
        - ❗ **NOTE:** 🐛 It is incorrectly set in the Initialization, implicitly set as **FALSE**.

- ### Register
    - This function registers a Project under a specified State.
    - ❗ **NOTE:** 🐛 We fail to properly check if the registration window is open.

- ### End Registrations
    - This function halts Project Registrations for a given State, essentially setting **registrations_round** to false.

- ### Invest
    - Participants can invest in a Project of their choice. We perform a check to see if `registrations_round` is still open:
    ```rust
    require!(
        !state.registrations_round,
        CustomError::RegistrationRoundOpen
    );
    ```

## Fuzz Test Checks
- ### ✔️Register Instruction Check
We first verify if the State is initialized before the Register instruction call.
```rust
if let Some(state) = pre_ix.state {
    // ...
}
```
Then, we check if the Project was successfully created.
```rust
if let Some(_project) = post_ix.project {
    // ...
}
```
Finally, if both previous checks have passed, we check if **registrations_round** within the State was false.
```rust
if !registrations_round {
    // ...
}
```
If this condition is met, we have identified the 🚨Error🚨.
```rust
return Err("We successfully registered a new project even though registrations are not open");
```

<u> Final Check </u>
```rust
if let Some(state) = pre_ix.state {
    if let Some(_project) = post_ix.project {
        let registrations_round = state.registrations_round;
        if !registrations_round {
            return Err("We successfully registered a new project even though registrations are not open");
        }
    }
}
```
