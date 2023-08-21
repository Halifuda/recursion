//! Recursion abstraction.
//!
#![no_std]

use core::any::Any;
use core::mem::replace;
use core::marker::PhantomData;

struct Recursion<Data, Gnerator> {
    data: Option<Data>,
    next: fn(&Data) -> Option<Data>,
    _gen: PhantomData<Gnerator>,
}

impl<Data, Gnerator> Recursion<Data, Gnerator> {
    fn new(data: Option<Data>, next: fn(&Data) -> Option<Data>) -> Self {
        Self {
            data,
            next,
            _gen: PhantomData::<Gnerator>::default(),
        }
    }

    fn set(&mut self, data: Data) -> Option<Data> {
        replace(&mut self.data, Some(data))
    }

    fn clear(&mut self) -> Option<Data> {
        replace(&mut self.data, None)
    }
}

impl<Data> Iterator for Recursion<Data, CallTwice> {
    type Item = Data;
    fn next(&mut self) -> Option<Self::Item> {
        let new = self.data.as_ref().and_then(self.next);
        let old = replace(&mut self.data, new);
        old.as_ref().and_then(self.next)
    }
}

impl<Data> Iterator for Recursion<Data, CloneNew>
where
    Data: Clone,
{
    type Item = Data;
    fn next(&mut self) -> Option<Self::Item> {
        self.data = self.data.as_ref().and_then(self.next);
        self.data.as_ref().cloned()
    }
}

impl<Data> Iterator for Recursion<Data, CopyNew>
where
    Data: Copy,
{
    type Item = Data;
    fn next(&mut self) -> Option<Self::Item> {
        self.data = self.data.as_ref().and_then(self.next);
        self.data
    }
}

struct CallTwice;
new_recursive!(CallTwiceRecursion, Any, CallTwice);

struct CloneNew;
new_recursive!(CloneRecursion, Clone, CloneNew);

struct CopyNew;
new_recursive!(CopyRecursion, Copy, CloneNew);

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! new_recursive {
        ($name:tt, $w: tt, $g:ty) => {
            pub struct $name<Data: $w>(Recursion<Data, $g>);
            impl<Data> $name<Data> 
            where
                Data: $w {
                pub fn new(data: Option<Data>, next: fn(&Data) -> Option<Data>) -> Self {
                    Self(Recursion::<Data, $g>::new(data, next))
                }
                pub fn set(&mut self, data: Data) -> Option<Data> {
                    self.0.set(data)
                }
                pub fn clear(&mut self) -> Option<Data> {
                    self.0.clear()
                }
            }
            impl<Data> Iterator for $name<Data>
            where
                Data: $w,
            {
                type Item = Data;
                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next()
                }
            }
        };
    }
}
