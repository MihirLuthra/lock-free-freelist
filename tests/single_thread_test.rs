use lock_free_freelist::FreeList;

#[test]
fn single_thread_test() {
    let free_list = FreeList::<Box<String>>::new();
    let string = String::from("hello");

    // free list is empty so should be Err
    if let Ok(_reuse) = free_list.reuse("abc".to_string()) {
        panic!("Free list should be empty at this point");
    };

    {
        // will be dropped at the end giving the pointer away to free list
        let _new_reuse = free_list.alloc(string);
    }

    if let Ok(reuse) = free_list.reuse("hello".to_string()) {
        // to check if it correctly derefs
        let _string: &String = &reuse;

        assert_eq!("hello".to_string(), **reuse);
    } else {
        panic!();
    };
}
