# push-vec
A crate that exposes a push-only vector type. Useful for getting a reference to an element and pushing more elements simultaneously

https://crates.io/crates/push-vec

# Example
```
use push_vec::prelude::*;

let mut vec = push_vec![0, 1, 2];
for x in vec.iter_mut() {
    // This is now allowed in a normal Vec!
    vec.push(x + 4);
    *x += 1;
}
```
