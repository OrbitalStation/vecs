//!
//! Crate for convenient usage of OpenGL shaders-like vectors.
//!
//! Main feature - powerful constructor:
//!
//! ```rust
//!use vecs::*;
//!
//! fn main() {
//!     let v1 = vec3::new(1., 2., 3.);
//!     println!("v1 = {:?}", v1);
//!
//!     let v2 = vec4::new();
//!     println!("v2 = {:?}", v2);
//!
//!     let v3 = vec4::new(v1, 4.);
//!     println!("v3 = {:?}", v3);
//!
//!     let v4 = bvec1::new(true);
//!     println!("v4 = {:?}", v4);
//!
//!     let v5 = bvec4::new(v4, false, v4);
//!     println!("v5 = {:?}", v5);
//!
//!     let v6 = uvec3::new((2u32, 2u32), (), [1u32]);
//!     println!("v6 = {:?}", v6);
//! }
//! ```
//!

#![feature(const_trait_impl)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]
#![feature(specialization)]

#![no_std]

#![allow(non_camel_case_types)]
#![allow(incomplete_features)]

mod impls;
mod private;
mod init;
mod alias;

pub(crate) use private::*;

pub use alias::*;

#[derive(Copy, Clone)]
pub struct vec <T: Copy, const N: usize> ([T; N]);
