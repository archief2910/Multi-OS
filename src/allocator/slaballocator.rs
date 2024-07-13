use core::mem::MaybeUninit;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::Ordering;
use core::{marker::PhantomData, ptr::NonNull, result::Result, sync::atomic::AtomicU8};

fn atomic_bts(arr: &[AtomicU8], idx: usize) -> Option<bool> {
    let el = arr.get(idx / 8)?;
    let bit = (idx % 8) as u8;

    let val = el.fetch_or(1 << bit, Ordering::Relaxed);
    Some((val & (1 << bit)) != 0)
}

fn atomic_btc(arr: &[AtomicU8], idx: usize) -> Option<bool> {
    let el = arr.get(idx / 8)?;
    let bit = (idx % 8) as u8;

    let val = el.fetch_and(!(1 << bit), Ordering::Relaxed);
    Some((val & (1 << bit)) != 0)
}

fn div_ceil(num: usize, dem: usize) -> usize {
    (num + dem) / dem
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlabError {
    /// The base address of the heap is not aligned properly.
    BadBaseAlignment,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlabAllocError {
    /// The heap was observed to be exhausted at the time of the allocation.
    HeapExhausted,
}

unsafe impl<T> Send for SlabAllocator<T> {}
unsafe impl<T> Sync for SlabAllocator<T> {}

/// A basic fixed-size slab allocator covering a range of memory.
#[derive(Debug)]
pub struct SlabAllocator<T: Sized> {
    _data: PhantomData<T>,
    /// The total number of elements in this slab allocator.
    num_elems: usize,
    /// The bitmap slice.
    bitmap: &'static mut [AtomicU8],
    /// The data slice.
    data: *mut [UnsafeCell<MaybeUninit<T>>],
}
                                 
impl<T: Sized> SlabAllocator<T> {
    /// Create a new instance of this allocator.
    /// If the input parameters are invalid, this will return a [SlabError].
    pub fn new(mem: *mut MaybeUninit<u8>, size: usize) -> Result<Self, SlabError> {
        // Ensure the size is aligned to the size of the contained objects.
        if size & (core::mem::size_of::<T>() - 1) != 0 {
            Err(SlabError::BadBaseAlignment)?;
        }

        let element_size = core::cmp::max(core::mem::size_of::<T>(), core::mem::align_of::<T>());

        // Calculate the size of the data segment, subtracting out the ideal
        // bitmap size.
        let data_size = size - div_ceil(size / element_size, 8);

        // Partition off the data first.
        // FIXME: Does this ensure the alignment of elements?
        let data = core::ptr::slice_from_raw_parts_mut(
             mem as *mut UnsafeCell<MaybeUninit<T>>,
            data_size / element_size, // N.B: `MaybeUninit<T>` is the same size/alignment as T.
        );

        // Calculate the actual number of elements that can be stored in the data segment.
        let num_elems = data_size / element_size;
        let bitmap_size = div_ceil(num_elems, 8);

        // Slice off the bitmap, taking care to initialize it.
        let bitmap = {
            let bitmap = unsafe {
                core::slice::from_raw_parts_mut(
                    mem.add(size - bitmap_size) as *mut MaybeUninit<AtomicU8>,
                    bitmap_size,
                )
            };

            for b in bitmap.iter_mut() {
                b.write(AtomicU8::new(0));
            }
           

            unsafe { MaybeUninit::slice_assume_init_mut(bitmap) }
        };

        Ok(Self {
            _data: PhantomData,

            num_elems,
            bitmap,
           data,
        })
    }

    fn allocate_slot(&self) -> Option<usize> {
        for i in 0..self.num_elems {
            if let Some(false) = atomic_bts(self.bitmap, i) {
                return Some(i);
            }
        }

        None
    }

    fn deallocate_slot(&self, slot: usize) {
        atomic_btc(self.bitmap, slot);
    }

    pub fn allocate_raw(&self) -> Result<&mut UnsafeCell<MaybeUninit<T>>, SlabAllocError> {
         // Allocate a slot, or return an error if the heap is exhausted
    let slot = self.allocate_slot().ok_or(SlabAllocError::HeapExhausted)?;

    // Get a reference to the data array
    let data = unsafe { &*self.data };

    // Get a mutable pointer to the element at the allocated slot
    let el_ptr = &data[slot] as *const _ as *mut UnsafeCell<MaybeUninit<T>>;

    Ok(unsafe {  el_ptr.as_mut().unwrap() })
    }

   pub unsafe fn deallocate_raw(&self, elm: *mut T) {
        
        let slot = elm.offset_from(self.data as *const T);
        if slot < 0 || (slot as usize) >= self.num_elems {
            panic!("Deallocate given an element that does not belong to us!");
        }

        self.deallocate_slot(slot as usize);
    }

    pub fn allocate<'a>(&'a self, item: T) -> Result<SlabAlloc<'a, T>, SlabAllocError> {
        let e = unsafe {
            let e = self.allocate_raw()?;
            e.get_mut().as_mut_ptr().write(item);
            e.get_mut().assume_init_mut()
        };

        Ok(SlabAlloc(self, unsafe { NonNull::new_unchecked(e) }))
    }
}

/// A container structure for an allocation made in a [SlabAllocator].
/// This is similar to a [core::alloc::Box].
#[derive(Debug)]
pub struct SlabAlloc<'a, T>(&'a SlabAllocator<T>, NonNull<T>);

impl<T> Drop for SlabAlloc<'_, T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.1.as_ptr());
            self.0.deallocate_raw(self.1.as_ptr())
        };
    }
}

impl<T> Deref for SlabAlloc<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.1.as_ref() }
    }
}

impl<T> DerefMut for SlabAlloc<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.1.as_mut() }
    }
}