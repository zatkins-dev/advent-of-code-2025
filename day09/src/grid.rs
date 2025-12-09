use std::alloc::{self, Layout};
use std::any::type_name;
use std::fmt::{Debug, Display};
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;

pub struct Grid<T> {
    ptr: NonNull<T>,
    pub rows: usize,
    pub cols: usize,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
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
            min_x: 0,
            max_x: (cols - 1) as isize,
            min_y: 0,
            max_y: (rows - 1) as isize,
        }
    }

    pub fn new_with_bounds(min_x: isize, min_y: isize, max_x: isize, max_y: isize) -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        assert!(min_x < max_x + 1, "Grid must have positive size");
        assert!(min_y < max_y + 1, "Grid must have positive size");
        let cols = max_x.abs_diff(min_x) + 1;
        let rows = max_y.abs_diff(min_y) + 1;
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
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}
impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        assert!(
            y >= self.min_y && y <= self.max_y,
            "y = {} out of range [{}, {}]",
            y,
            self.min_y,
            self.max_y
        );
        assert!(
            x >= self.min_x && x <= self.max_x,
            "x = {} out of range [{}, {}]",
            x,
            self.min_x,
            self.max_x
        );
        let row = (y - self.min_y) as usize;
        let col = (x - self.min_x) as usize;
        unsafe { self.ptr.add(self.cols * row + col).as_ref() }
    }
}
impl<T> IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        assert!(
            y >= self.min_y && y <= self.max_y,
            "y = {} out of range [{}, {}]",
            y,
            self.min_y,
            self.max_y
        );
        assert!(
            x >= self.min_x && x <= self.max_x,
            "x = {} out of range [{}, {}]",
            x,
            self.min_x,
            self.max_x
        );
        let row = (y - self.min_y) as usize;
        let col = (x - self.min_x) as usize;
        unsafe { self.ptr.add(self.cols * row + col).as_mut() }
    }
}
impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.min_y..=self.max_y {
            for j in self.min_x..=self.max_x {
                match write!(f, "{}", self[(j, i)]) {
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
impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(
            f,
            "Grid<{}>(rows: {}, cols: {}):\n",
            type_name::<T>(),
            self.rows,
            self.cols
        ) {
            Ok(()) => (),
            Err(e) => return Err(e),
        };
        for i in self.min_y..=self.max_y {
            for j in self.min_x..=self.max_x {
                match write!(f, "{:?}", self[(j, i)]) {
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
