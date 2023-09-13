#![no_std]
#[cfg(test)]
mod test;


use core::ops::RangeFull;
use core::ops::Index;
use core::slice::from_raw_parts;

use core::fmt::Formatter;
use core::fmt;
use core::fmt::Debug;

use core::ptr::NonNull;

pub struct Array<T, const CAP:usize> {
    len: usize,
    start: *mut T,
}


impl<T, const CAP:usize> Array<T, CAP> {
    pub fn new(base:*mut T) -> Array<T, CAP> {
        Self{
            len: 0,
            start: base
        }
    }
    
    pub fn append(&mut self, value:T) -> Result<(),()> {
        if self.len == CAP {
            return Err(());
        }
        let target = unsafe{NonNull::new_unchecked(self.start.add(self.len)).as_mut()};
        *target = value;
        self.len += 1;
        Ok(())
    }
    
    pub fn get(&self, index:usize) -> Result<NonNull<T>,()> {
        if index > self.len {
            return Err(());
        }
        Ok(unsafe{NonNull::new_unchecked(self.start.add(index))})
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
}


impl<T, const CAP:usize> Index<RangeFull> for Array<T, CAP> {
    type Output = [T];
    fn index(&self, _:RangeFull) -> &Self::Output {
        unsafe{from_raw_parts(self.start, self.len)}
    }
}


impl<T:Debug, const CAP:usize> Debug for Array<T, CAP> {
    
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Array")
           .field("len", &self.len)
           .field("cap", &CAP)
           .field("content", &&self[..])
           .finish()
    }
    
}
