//! A Trait that repersents an iterator that can return None but still yeild
//! elements in nice way.
//!
//! ```rust
//! use fallible::prelude::*;
//!
//! #[derive(Clone)]
//! struct Meal(usize);
//! struct Food;
//!
//! enum FoodError {
//!     NoMore,
//!     Rotten,
//! }
//!
//! impl FallibleIterator for Meal {
//!     type SomeItem = Food;
//!     type Error = FoodError;
//!
//!     fn some_next(&mut self) -> Fallible<Self::SomeItem, Self::Error> {
//!         self.0 += 1;
//!         match self.0 {
//!             0..=2 => Thing(Food),
//!             3 => Nothing,
//!             4 => Thing(Food),
//!             5..=7 => Failure(FoodError::Rotten),
//!             8 => Thing(Food),
//!             _ => Failure(FoodError::NoMore),
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut dinner = Meal(0);
//!     for _food in dinner.as_iter() {
//!         println!("Good Food!");
//!     }
//! }
//! ```

#![no_std]
#![deny(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unused_imports,
    dead_code,
    unused_crate_dependencies
)]
// #![feature(never_type)]
// #![forbid(unsafe_code)]

extern crate alloc;

/// An iterator over [`Fallible`]s.
pub mod iter;
/// A collection of useful imports
pub mod prelude;

// /// A sum that repersents both a union between [`Result`] and [`Option`].
// pub trait FallibleExt<T, E> {
//     fn unwrap_both(self) -> T;
//     fn unwrap_none(self) -> Result<T, E>;
//     fn unwrap_err(self) -> Option<T>;
// }
//
// impl<T, E> FallibleExt<T, E> for Option<Result<T, E>> {
//     fn unwrap_both(self) -> T {
//         match self {
//             Some(Ok(t)) => t,
//             Some(Err(_)) => panic!("Called [`FallibleExt::unwrap_both`] on Some([`Result::Err`])."),
//             None => panic!("Called [`FallibleExt::unwrap_both`] on [`Option::None`]."),
//         }
//     }
//
//     fn unwrap_none(self) -> Result<T, E> {
//         self.expect("Called [`FallibleExt::unwrap_none`] on [`Option::None`]")
//     }
//
//     fn unwrap_err(self) -> Option<T> {
//         match self {
//             Some(Ok(t)) => Some(t),
//             Some(Err(_)) => panic!("Called [`FallibleExt::unwrap_err`] on Some([`Result::Err`])"),
//             None => None,
//         }
//     }
// }

// impl<T, E> Fallible<T, E> {
//     pub fn unwrap_or_default(self) -> T
//     where
//         T: Default,
//     {
//         match self {
//             Thing(t) => t,
//             Failure(_) => T::default(),
//             Nothing => T::default(),
//         }
//     }
// }
