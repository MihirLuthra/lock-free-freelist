use super::smart_pointer::{InitializableSmartPointer, SmartPointer};

impl<T> SmartPointer for Box<T> {
    type Content = T;

    unsafe fn from_raw(raw: *mut Self::Content) -> Self {
        Box::from_raw(raw)
    }

    fn into_raw(smart_pointer: Self) -> *mut Self::Content {
        Box::into_raw(smart_pointer)
    }
}

impl<T> InitializableSmartPointer for Box<T> {
    fn new(data: <Self as SmartPointer>::Content) -> Self {
        Box::new(data)
    }
}
