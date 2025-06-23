use std::alloc;
use std::ptr;
use std::fmt;

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

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }

        unsafe { Some(&*self.data.add(index)) }
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.size {
            panic!("Index out of bounds.");
        }

        unsafe {
            ptr::write(self.data.add(index), value);
        }
    }

    pub fn drop_last(&mut self) {
        if self.is_empty() { return; }

        self.size -= 1;
        unsafe {
            ptr::drop_in_place(self.data.add(self.size));
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
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

impl<T: fmt::Debug> fmt::Debug for RawArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }
            unsafe {
                write!(f, "{:?}", &*self.data.add(i))?;
            }
        }
        write!(f, "]")
    }
}