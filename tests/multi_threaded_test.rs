use lock_free_freelist::{FreeList, Reuse, Reusable};
use rand::prelude::*;
use std::thread;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Reusable)]
struct Container {
    a: i32,
    b: String,
}

impl Container {
    fn rand() -> Self {
        Self {
            a: random(),
            b: rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(10)
                .collect::<String>(),
        }
    }

    fn heap_alloc_rand<'a>(free_list: &'a FreeList<Box<Container>>) -> Reuse<'a, Box<Container>> {
        let rand_container = Self::rand();
        match free_list.reuse(rand_container) {
            Ok(reused) => {
                //println!("Reusing");
                reused
            },
            Err(rand_container) => {
                //println!("Allocating new");
                free_list.alloc(rand_container)
            },
        }
    }
}

lazy_static! {
    static ref FREE_LIST: FreeList<std::boxed::Box<Container>> = FreeList::<Box<Container>>::new();
}

#[test]
fn multi_threaded_test() {
    let thread_count = 4;
    let mut thread_handles = Vec::with_capacity(thread_count);

    let mut vec = Vec::new();
    for _ in 0..100 {
        vec.push(Container::heap_alloc_rand(&FREE_LIST));
    }
 
    for i in 0..thread_count {
        let builder = thread::Builder::new().name(format!("thread{}", i));

        let handle = builder.spawn(|| {
            for _ in 0..100 {
                let _container = Container::heap_alloc_rand(&FREE_LIST);
            }
        });

        thread_handles.push(handle.unwrap());
    }

    while !vec.is_empty() {
        vec.remove(vec.len() - 1);
    }

    for handle in thread_handles.into_iter() {
        handle.join().unwrap();
    }

    unsafe {
        FREE_LIST.clear();
    }
}
