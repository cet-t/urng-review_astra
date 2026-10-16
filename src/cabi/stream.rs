//! Shared C ABI handles. The generator is the first field so exported pointer
//! types remain unchanged; handles must only be freed by their matching export.
use crate::{Rng, Word};

#[repr(C)]
struct Handle<R, W, const N: usize> {
    rng: R,
    pending: [W; N],
    pos: usize,
}

pub(super) fn new<R, W: Default + Copy, const N: usize>(rng: R) -> *mut R {
    Box::into_raw(Box::new(Handle {
        rng,
        pending: [W::default(); N],
        pos: N,
    }))
    .cast()
}

pub(super) unsafe fn free<R, W, const N: usize>(ptr: *mut R) {
    if !ptr.is_null() {
        // SAFETY: caller supplies a live handle from new with the same R/W/N.
        unsafe {
            drop(Box::from_raw(ptr.cast::<Handle<R, W, N>>()));
        }
    }
}

pub(super) struct Stream<'a, R, W, const N: usize, D> {
    handle: &'a mut Handle<R, W, N>,
    draw: D,
}
impl<R, W: Word, const N: usize, D: FnMut(&mut R) -> [W; N]> Rng for Stream<'_, R, W, N, D> {
    type Word = W;
    fn nextu(&mut self) -> W {
        if self.handle.pos == N {
            self.handle.pending = (self.draw)(&mut self.handle.rng);
            self.handle.pos = 0;
        }
        let word = self.handle.pending[self.handle.pos];
        self.handle.pos += 1;
        word
    }
}

pub(super) unsafe fn fill<R, W: Word, T, const N: usize, D, F>(
    ptr: *mut R,
    out: *mut T,
    count: usize,
    draw: D,
    mut sample: F,
) where
    D: FnMut(&mut R) -> [W; N],
    F: FnMut(&mut Stream<'_, R, W, N, D>) -> T,
{
    // Zero-length operations accept null and do not advance the generator.
    if count == 0
        || ptr.is_null()
        || out.is_null()
        || !out.is_aligned()
        || !ptr.is_aligned()
        || count > isize::MAX as usize / std::mem::size_of::<T>()
    {
        return;
    }
    // SAFETY: the exported unsafe contract requires valid, disjoint allocations,
    // exclusive access, and a handle allocated by the matching constructor.
    let handle = unsafe { &mut *ptr.cast::<Handle<R, W, N>>() };
    let buffer = unsafe { std::slice::from_raw_parts_mut(out, count) };
    let mut stream = Stream { handle, draw };
    for value in buffer {
        *value = sample(&mut stream);
    }
}
