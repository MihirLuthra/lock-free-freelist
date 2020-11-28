//! A fast lock free limited length free list for multiple producer and consumer.
//! It is meant for cases where consumer is as fast as producer and hence the
//! limited length of the free list doesn't matter.
//!
//! It uses bitmaps of type `usize` to keep track of the free list
//! and hence free list has a size equal to number of bits in `usize`. If `usize` is `8` bytes then `64`
//! and if `usize` is `4` bytes then `32`.

mod dump;
mod reuse;
mod free_list;
mod smart_pointer;
mod reusable;

pub use reuse::Reuse;
pub use free_list::FreeList;
pub use smart_pointer::SmartPointer;
pub use reusable::Reusable;
