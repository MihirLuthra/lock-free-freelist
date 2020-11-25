pub trait SmartPointer {
    type Content;

    unsafe fn from_raw(raw: *mut Self::Content) -> Self;
    fn into_raw(smart_pointer: Self) -> *mut Self::Content;
    fn new(data: Self::Content) -> Self;
}
