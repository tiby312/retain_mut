//! This crate provides trait `RetainMut` which
//! provides `retain_mut` method for `Vec` and `VecDeque`.
//!
//! `retain_mut` is basically the same as `retain` except that
//! it gives mutable reference of items to the predicate function.
//!
//! Since there is no reason `retain` couldn't have been designed this way,
//! this crate basically just copies the code from std with minor (1-line) change
//! to hand out mutable reference.
//! The code these impls are based on can be found in code comments of this crate.
//!
//! # Examples
//!
//! ## `Vec`
//!
//! ```
//! # use retain_mut::RetainMut;
//! let mut vec = vec![1, 2, 3, 4];
//! vec.retain_mut(|x| { *x *= 3; *x % 2 == 0 });
//! assert_eq!(vec, [6, 12]);
//! ```
//!



/// Trait that provides `retain_mut` method.
pub trait RetainMut<T> {
    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` such that `f(&e)` returns `false`.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool;

    fn retain_mut_unordered<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool;
}


impl<T> RetainMut<T> for Vec<T> {
    // The implementation is based on
    // https://github.com/rust-lang/rust/blob/0eb878d2aa6e3a1cb315f3f328681b26bb4bffdb/src/liballoc/vec.rs#L1072-L1093
    fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let len = self.len();
        let mut del = 0;
        {
            let v = &mut **self;

            for i in 0..len {
                if !f(&mut v[i]) {
                    del += 1;
                } else if del > 0 {
                    v.swap(i - del, i);
                }
            }
        }
        if del > 0 {
            self.truncate(len - del);
        }
    }

    
    fn retain_mut_unordered<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let len = self.len();
        let mut del = 0;
        {
            let v = &mut **self;

            let mut cursor=0;
            for _ in 0..len {
                if !f(&mut v[cursor]) {
                    v.swap(cursor,len-1-del);
                    del+=1;
                    
                }else{
                    cursor+=1;
                }
            }
        }
        if del > 0 {
            self.truncate(len-del);
        }
    }
}

