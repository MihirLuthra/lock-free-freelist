use bit_fiddler::{max_bits, set, unset};
use std::{
    cell::UnsafeCell,
    ptr::null_mut,
    sync::atomic::{AtomicUsize, Ordering},
};

/// In this struct,
/// max_bits!(reader_bitmap) == max_bits!(writer_bitmap) == dump.len()
///
/// The accesses to dump[] array are synchronized by reader_bitmap
/// and writer_bitmap.
pub struct Dump<T> {
    reader_bitmap: AtomicUsize,
    writer_bitmap: AtomicUsize,
    dump: UnsafeCell<[*mut T; max_bits!(type = usize)]>,
}

unsafe impl<T: Send> Send for Dump<T> {}
unsafe impl<T: Sync> Sync for Dump<T> {}

impl<T> Dump<T> {
    /// Returns a new Dump instance.
    ///
    /// ```ignore
    ///
    /// struct Example {
    ///     a: i32,
    ///     b: String,
    /// }
    ///
    /// let dump = Dump::<Example>::new();
    /// ```
    pub fn new() -> Self {
        Dump {
            reader_bitmap: AtomicUsize::new(0),
            writer_bitmap: AtomicUsize::new(0),
            dump: UnsafeCell::new([null_mut::<T>(); max_bits!(type = usize)]),
        }
    }

    /// Adds a new element to the dump. On success it returns
    /// () and on failure returns back the ptr indicating
    /// that it couldn't be stored.
    ///
    /// To synchronize this addition to the dump[] array, the following
    /// procedure is followed:
    ///
    /// 1) It checks `writer_bitmap` for unset bits (0 bits).
    /// 2) When it finds one, it atomically sets it.
    /// 3) We use this bit position as the index in `dump[]` to store the value.
    /// 4) Setting the bit in `writer_bitmap` ensures that no
    ///    other thread will write at that index.
    /// 5) After storing `raw` in the `dump[]`, we tell reader threads
    ///    that this index is available for read. To do this, we set this
    ///    same bit position in `reader_bitmap` atomically.
    pub fn throw(&self, raw: *mut T) -> Result<(), *mut T> {
        let mut old_writer_bitmap = self.writer_bitmap.load(Ordering::Relaxed);
        let mut first_empty_spot;

        loop {
            // basically returns the first bit which is 0
            first_empty_spot = old_writer_bitmap.trailing_ones();

            // occupy `first_empty_spot` in `old_writer_bitmap` and assign it to `new_writer_bitmap`
            let new_writer_bitmap = if first_empty_spot as usize == max_bits!(type = usize) {
                return Err(raw);
            } else {
                set!(old_writer_bitmap, usize, first_empty_spot)
            };

            match self.writer_bitmap.compare_exchange_weak(
                old_writer_bitmap,
                new_writer_bitmap,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(old) => old_writer_bitmap = old,
            };
        }

        let dump_ptr = self.dump.get();

        unsafe {
            (*dump_ptr)[first_empty_spot as usize] = raw;
        }

        let mut old_reader_bitmap = self.reader_bitmap.load(Ordering::Relaxed);

        loop {
            let new_reader_bitmap = set!(old_reader_bitmap, usize, first_empty_spot);

            /*
             * Memory order on success should be `Ordering::Release`.
             * If it was Ordering::Relaxed, it would become possible
             * that `recycle()` sees this bit as set in `reader_bitmap`
             * but doesn't see the newly updated value in `dump[]`.
             */
            match self.reader_bitmap.compare_exchange_weak(
                old_reader_bitmap,
                new_reader_bitmap,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(old) => old_reader_bitmap = old,
            };
        }

        Ok(())
    }

    /// Gets a value from the dump. On success it returns
    /// the value `*mut T` and on failure (). Failure indicates
    /// that dump is empty.
    ///
    /// To synchronize the retreival from the dump[] array, the following
    /// procedure is followed:
    ///
    /// 1) A set bit is searched in `reader_bitmap` and then we
    ///    atomically unset that bit in `reader_bitmap`.
    /// 2) Corresponding to the bit posn that we unset, we get the
    ///    `dump[bit_posn]`.
    /// 3) Then to allow writers to use this position for new writes,
    ///    we unset this bit from `writer_bitmap`.
    /// 4) Finally, we return `dump[bit_posn]`.
    pub fn recycle(&self) -> Result<*mut T, ()> {
        let mut old_reader_bitmap = self.reader_bitmap.load(Ordering::Relaxed);
        let mut first_set_spot;

        loop {
            // basically returns the first bit which is 1
            first_set_spot = old_reader_bitmap.trailing_zeros();

            // occupy `first_set_spot` in `old_reader_bitmap` and assign it to `new_reader_bitmap`
            let new_reader_bitmap = if first_set_spot as usize == max_bits!(type = usize) {
                return Err(());
            } else {
                unset!(old_reader_bitmap, usize, first_set_spot)
            };

            match self.reader_bitmap.compare_exchange_weak(
                old_reader_bitmap,
                new_reader_bitmap,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(old) => old_reader_bitmap = old,
            };
        }

        let dump_ptr = self.dump.get();

        let retval = unsafe { (*dump_ptr)[first_set_spot as usize] };

        let mut old_writer_bitmap = self.writer_bitmap.load(Ordering::Relaxed);

        loop {
            let new_writer_bitmap = unset!(old_writer_bitmap, usize, first_set_spot);

            match self.writer_bitmap.compare_exchange_weak(
                old_writer_bitmap,
                new_writer_bitmap,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(old) => old_writer_bitmap = old,
            };
        }

        Ok(retval)
    }

    /// This function is not thread safe and hence it takes
    /// a mutable reference to check at compile time that
    /// no other methods that take reference to dump are called.
    ///
    /// This executes closure `f` for every value in the dump
    /// and then clears the dump.
    pub fn for_each<F>(&mut self, f: F)
    where
        F: Fn(*mut T) -> (),
    {
        let mut reader_bitmap = self.reader_bitmap.load(Ordering::Relaxed);

        self.reader_bitmap.store(0, Ordering::Relaxed);
        self.writer_bitmap.store(0, Ordering::Relaxed);

        loop {
            // Fast if set bits are sparse which should generally be the case.
            let first_set_spot = reader_bitmap.trailing_zeros();

            if first_set_spot as usize == max_bits!(type = usize) {
                break;
            }

            unset!(in reader_bitmap, usize, first_set_spot);

            let dump_ptr = self.dump.get();
            let val_at_index = unsafe { (*dump_ptr)[first_set_spot as usize] };

            f(val_at_index);
        }
    }
}
