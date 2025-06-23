use std::alloc;
use std::ptr;
use std::mem;
use std::fmt;


#[derive(Debug)]
pub struct RawArray<T> {
    data: *mut T,
    size: usize,
    capacity: usize,
}

impl<T> RawArray<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        let layout = alloc::Layout::array::<T>(capacity).unwrap();

        let data = unsafe {
            alloc::alloc(layout) as *mut T
        };

        if data.is_null() {
            panic!("Failed allocating memory.");
        }

        Self {
            data,
            size: 0,
            capacity
        }
    }

    pub fn push(&mut self, value: T) {
        if self.size >= self.capacity {
            panic!("Capacity exceeded.")
        }

        unsafe {
            ptr::write(self.data.add(self.size), value);
        }

        self.size += 1;
    }

    pub fn get_size(&self) -> &usize {
        &self.size
    }
}

impl<T> Drop for RawArray<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.size {
                ptr::drop_in_place(self.data.add(i));
            }

            let layout = alloc::Layout::array::<T>(self.capacity).unwrap();
            alloc::dealloc(self.data as *mut u8, layout);
        }
    }
}