use super::{
    free_list::FreeList,
    smart_pointer::SmartPointer,
};

pub struct FBox<'a, T: SmartPointer> {
    smart_pointer: T,
    free_list: &'a FreeList<'a, T>,
}
