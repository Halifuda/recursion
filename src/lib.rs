//! Recursion abstraction.
//!
#![no_std]

use core::any::Any;
use core::marker::PhantomData;

struct Recursion<Data, Gnerator> {
    /// The inner data. The recursion saves an instance
    /// of currently arrived iter step.
    data: Option<Data>,
    /// The generator function pointer. It receives the
    /// previous data, and produce the next iter. `None`
    /// may be produced if the recursion reaches end.
    next: fn(&Data) -> Option<Data>,
    /// A marker of generator.
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
        self.data.replace(data)
    }

    fn clear(&mut self) -> Option<Data> {
        self.data.take()
    }
}

impl<Data> Iterator for Recursion<Data, CallTwice> {
    type Item = Data;
    fn next(&mut self) -> Option<Self::Item> {
        let new = self.data.as_ref().and_then(self.next);
        let old = core::mem::replace(&mut self.data, new);
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
struct CloneNew;
struct CopyNew;

/// Recursion leverages [`CallTwice`] iterator strategy. From one
/// old data `&old` to two instances of new data (in struct and
/// return value), it calls the generator twice. This may be unsafe
/// if the generator has side-affect.
pub struct CallTwiceRecursion<Data: Any>(Recursion<Data, CallTwice>);
impl_recursive!(CallTwiceRecursion, Any, CallTwice);

/// Recursion leverages [`CloneNew`] iterator strategy. It uses
/// the generator to generate a new data from one old data, then
/// clone the new data to produce the in-struct and return value
/// both. This requires the `Data` to be [`Clone`].
pub struct CloneRecursion<Data: Clone>(Recursion<Data, CloneNew>);
impl_recursive!(CloneRecursion, Clone, CloneNew);

/// Recursion leverages [`CopyNew`] iterator strategy. It uses
/// the generator to generate a new data from one old data, then
/// copy the new data to produce the in-struct and return value
/// both. This requires the `Data` to be [`Copy`].
pub struct CopyRecursion<Data: Copy>(Recursion<Data, CopyNew>);
impl_recursive!(CopyRecursion, Copy, CopyNew);

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! impl_recursive {
        ($name:tt, $w: tt, $g:ty) => {
            impl<Data> $name<Data>
            where
                Data: $w,
            {
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
