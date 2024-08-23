# chronit
Take some time to measure the time

## Example
```rust
use chronit::chronit;

fn main() {
    let closure = || {
        std::thread::sleep(std::time::Duration::new(1, 0));
        1 + 1
    };
    let v = chronit(closure);
    
    assert_eq!(v.value, 2);
    assert!(f32::abs(v.time - 1f32) < 0.1f32);
}
```