use std::{
    sync::atomic::AtomicUsize,
    ptr::null_mut,
};
use bit_fiddler::max_bits;

pub struct Dump<T> {
    reader_bitmap: AtomicUsize,
    writer_bitmap: AtomicUsize,
    dump: [*mut T; max_bits!(type = usize)],
}

unsafe impl<T: Send> Send for Dump<T> {}
unsafe impl<T: Sync> Sync for Dump<T> {}

impl<T> Dump<T> {
    pub fn new() -> Self {
        Dump {
            reader_bitmap: AtomicUsize::new(usize::MAX),
            writer_bitmap: AtomicUsize::new(usize::MAX),
            dump: [null_mut::<T>(); max_bits!(type = usize)],
        }
    }
}
