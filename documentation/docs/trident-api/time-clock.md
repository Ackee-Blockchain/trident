# Time & Clock Methods

These methods provide functionality for manipulating time and clock-related operations in the fuzzing environment, allowing you to test time-dependent program behavior.

## Time Manipulation Methods

### `warp_to_epoch`

Warps the clock to a specific epoch.

```rust
pub fn warp_to_epoch(&mut self, warp_epoch: u64)
```

**Parameters:**

- `warp_epoch` - The epoch to warp to

**Description:** Changes the current epoch to test how your program behaves at different epochs.

---

### `warp_to_slot`

Warps the clock to a specific slot.

```rust
pub fn warp_to_slot(&mut self, warp_slot: u64)
```

**Parameters:**

- `warp_slot` - The slot to warp to

**Description:** Changes the current slot to test how your program behaves at different slots.

---

### `warp_to_timestamp`

Warps the clock to a specific Unix timestamp.

```rust
pub fn warp_to_timestamp(&mut self, warp_timestamp: i64)
```

**Parameters:**

- `warp_timestamp` - The Unix timestamp to warp to

**Description:** Changes the current time to test how your program behaves at specific timestamps.

---

### `forward_in_time`

Advances the clock by the specified number of seconds.

```rust
pub fn forward_in_time(&mut self, seconds: i64)
```

**Parameters:**

- `seconds` - Number of seconds to advance the clock

**Description:** Moves time forward by the specified number of seconds to test time-dependent program behavior.

---

## Time Query Methods

### `get_current_timestamp`

Gets the current Unix timestamp from the clock sysvar.

```rust
pub fn get_current_timestamp(&self) -> i64
```

**Returns:** Current Unix timestamp.

**Description:** Gets the current timestamp that your program would see when executing.

---

## Example Usage

```rust
use trident_fuzz::*;

#[flow]
fn test_time_dependent_logic(&mut self) {
    // Get initial timestamp
    let start_time = self.get_current_timestamp();
    
    // Test program logic at current time
    // ... your program calls ...
    
    // Advance time by 1 hour
    self.forward_in_time(3600);
    
    // Test program logic after time advancement
    // ... your program calls ...
    
    // Warp to specific timestamp
    self.warp_to_timestamp(1640995200); // Jan 1, 2022
    
    // Test program logic at specific time
    // ... your program calls ...
    
    // Warp to specific epoch
    self.warp_to_epoch(500);
    
    // Test epoch-dependent logic
    // ... your program calls ...
}
```
