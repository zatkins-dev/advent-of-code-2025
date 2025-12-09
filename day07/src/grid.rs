use std::alloc::{self, Layout};
use std::fmt::Display;
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;

pub struct Grid<T> {
    ptr: NonNull<T>,
    pub rows: usize,
    pub cols: usize,
}

unsafe impl<T: Send> Send for Grid<T> {}
unsafe impl<T: Sync> Sync for Grid<T> {}
impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        assert!(rows.overflowing_mul(cols).1 == false, "Overflowing integer");
        let layout = Layout::array::<T>(rows * cols).unwrap();
        let ptr = unsafe { alloc::alloc_zeroed(layout) };

        Grid {
            ptr: match NonNull::new(ptr as *mut T) {
                Some(p) => p,
                None => alloc::handle_alloc_error(layout),
            },
            rows: rows,
            cols: cols,
        }
    }
}
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        unsafe { self.ptr.add(self.cols * index.0 + index.1).as_ref() }
    }
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        unsafe { self.ptr.add(self.cols * index.0 + index.1).as_mut() }
    }
}
impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { std::slice::from_raw_parts(self.ptr.add(self.cols * index).as_ptr(), self.cols) }
    }
}
impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.add(self.cols * index).as_ptr(), self.cols)
        }
    }
}
impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                match write!(f, "{}", self[(i, j)]) {
                    Ok(()) => (),
                    Err(e) => return Err(e),
                };
            }
            match write!(f, "\n") {
                Ok(()) => (),
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}
