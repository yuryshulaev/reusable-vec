# reusable-vec

A `Vec` wrapper that allows reusing contained values.
It’s useful when the values are expensive to initialize, but can be reused, for example heap-based containers.

```rust
pub struct ReusableVec<T> {
    vec: Vec<T>,
    len: usize,
}
```

Derefs to a slice of valid items, i. e. `&self.vec[..self.len]`.

## Example

```rust
struct Thing {
    cheap: u32,
    expensive: Vec<u32>,
}

fn main() {
    let mut things = reusable_vec::ReusableVec::<Thing>::new();

    for _ in 0..2 {
        let new_thing = Thing { cheap: 123, expensive: Vec::new() };

        if let Some(reused) = things.push_reuse() {
            // Reuse members with previously allocated heap storage
            let mut expensive = std::mem::take(&mut reused.expensive);
            // They may still contain something
            expensive.clear();
            // Assigning the whole struct safeguards against forgetting to assign new values
            // to some of the fields
            *reused = Thing { expensive, ..new_thing };
        } else {
            things.push(Thing { expensive: Vec::with_capacity(100), ..new_thing });
        }

        things.last_mut().unwrap().expensive.push(456);

        for thing in &things {
            println!("{} {:?}", thing.cheap, thing.expensive);
        }

        // Release all items: sets `len` to 0
        things.clear_reuse();
        // Drop all items: calls `vec.clear()`
        // things.clear_drop();
    }
}
```
