# lock-free-freelist

A fast lock free limited length free list for multiple producer and consumer. It is meant for cases where consumer is as fast as producer and hence the limited length of the free list doesn't matter.

It uses bitmaps of type usize to keep track of the free list and hence free list has a size equal to number of bits in usize. If usize is 8 bytes then 64 and if usize is 4 bytes then 32.

A free list can store free pointers for one type only. For example,

```rust
let free_list = FreeList::<Box<i32>>::new(); // a free list for Box<i32>
```

# Example

```rust
use lock_free_freelist::{FreeList, Reusable};
use std::{thread, iter};

#[macro_use]
extern crate lazy_static;

#[derive(Reusable)]
struct MyType {
    name: String,
    age: u32,
}

// from https://crates.io/crates/lazy_static
lazy_static! {
    static ref FREE_LIST: FreeList<Box<MyType>> = FreeList::<Box<MyType>>::new();
}

fn main() {
    // Spawn 4 threads
    // Each thread will allocate elements of type `MyType` using FREE_LIST
    let threads = iter::repeat(0).take(100)
        .map(|_| {
            thread::spawn(|| { for i in 0..1000 {
                let my_type = MyType { name: "Jane".to_string(), age: 30 };

                let mut my_type_on_heap = FREE_LIST.reuse_or_alloc(my_type);

                // This is similar to:
                //      let mut my_type_on_heap = Box::new(my_type);
                // But when using FREE_LIST.reuse_or_alloc(), the dropped
                // memory will be reused.

                my_type_on_heap.name.push_str(" Doe");
                my_type_on_heap.age = i;
            }})
        })
        .collect::<Vec<_>>();

    for handle in threads {
        handle.join().unwrap()
    }
}
```

API docs: https://docs.rs/lock-free-freelist
