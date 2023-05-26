//! Grumpkin curve implementation in Rust
//! Code in this module adapted from https://github.com/jules/grumpkin

mod curve;
mod fq;
mod fr;

pub use curve::*;
pub use fq::*;
pub use fr::*;

#[cfg(all(feature = "prefetch", target_arch = "x86_64"))]
#[inline(always)]
pub fn prefetch<T>(data: &[T], offset: usize) {
    use core::arch::x86_64::_mm_prefetch;
    unsafe {
        _mm_prefetch(
            data.as_ptr().offset(offset as isize) as *const i8,
            core::arch::x86_64::_MM_HINT_T0,
        );
    }
}
