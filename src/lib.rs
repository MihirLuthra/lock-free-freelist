//! A fast lock free limited length free list.
//!
//! It uses bitmaps of type `usize` to keep track of the free list
//! and hence free list has a size equal to number of bits in `usize`. If `usize` is `8` bytes then `64`
//! and if `usize` is `4` bytes then `32`.

mod dump;
mod fbox;
mod free_list;
mod smart_pointer;

pub use fbox::FBox;
pub use free_list::FreeList;
pub use smart_pointer::SmartPointer;
