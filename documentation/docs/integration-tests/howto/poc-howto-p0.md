# Init Fixture

```rust
// <my_project>/trident-tests/poc_tests/tests/test.rs
// TODO: do not forget to add all necessary dependencies to the generated `trident-tests/poc_tests/Cargo.toml`
use program_client::my_instruction;
use trident_client::*;
use my_program;

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
  // create a test fixture
  let mut fixture = Fixture {
    client: Client::new(system_keypair(0)),
    // make sure to pass the correct name of your program
    program: anchor_keypair("my_program_name").unwrap(),
    state: keypair(42),
  };
  // deploy the program to test
  fixture.deploy().await?;
  // call instruction init
  my_instruction::initialize(
    &fixture.client,
    fixture.state.pubkey(),
    fixture.client.payer().pubkey(),
    System::id(),
    Some(fixture.state.clone()),
  ).await?;
  fixture
}

#[trident_test]
async fn test_happy_path(#[future] init_fixture: Result<Fixture>) {
  let fixture = init_fixture.await?;
  // call the instruction
  my_instruction::do_something(
    &fixture.client,
    "dummy_string".to_owned(),
    fixture.state.pubkey(),
    None,
  ).await?;
  // check the test result
  let state = fixture.get_state().await?;
  assert_eq!(state.something_changed, "yes");
}
```
