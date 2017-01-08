use std::ops;
use std::slice;

extern crate libc;
extern crate gmp;
extern crate u3_alloc;

type c3_w = libc::uint32_t;
type u3_noun = c3_w;
type u3_atom = u3_noun;
type u3_cell = u3_noun;

extern {
    /// Copy count of words into a new atom.
    fn u3i_words(count: c3_w, data: *const c3_w) -> u3_noun;

    /// Copy atom into GMP value.
    fn u3r_mp(a_mp: *mut gmp::mpz_struct, b: u3_atom);

    /// Gain a reference count in normal space.
    fn u3a_gain(som: u3_noun) -> u3_noun;

    /// Lose a reference count
    fn u3a_lose(som: u3_noun);

    // TODO: This is the old C func as fallback while I'm not handling all the cases.
    // Remove it completely when I've got everything covered.
    fn u3qa_add_orig(a: u3_atom, b: u3_atom) -> Atom;
}


fn loom_addr(noun: u32) -> *const u32 {
    // TODO: This is from portable.h, with different defines for different platforms, the Rust-side
    // needs that too. Current one is Linux only.
    const U3_OS_LOOM_BASE: u32 = 0x36000000;
    const NOUN_ADDR_MASK: u32 = !(1<<31) | !(1<<30);

    (U3_OS_LOOM_BASE + (noun & NOUN_ADDR_MASK)) as *const u32
}

/// Rust wrapper for any noun value in the u3 loom.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Noun(u3_noun);

impl Noun {
    pub fn as_atom(&self) -> Option<Atom> {
        if self.0 >> 30 != 3 {
            Some(Atom(self.0))
        } else {
            None
        }
    }

    pub fn as_cell(&self) -> Option<Cell> {
        if self.0 >> 30 == 3 {
            Some(Cell(self.0))
        } else {
            None
        }
    }
}

/// Rust wrapper for value in the u3 loom that's known to be an atom.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Atom(u3_atom);

impl Atom {
    pub fn as_noun(&self) -> Noun { Noun(self.0) }

    pub fn as_direct(&self) -> Option<u32> {
        if self.0 >> 31 == 0 {
            Some(self.0)
        } else {
            None
        }
    }

    // XXX: Not sure what the lifetime story for this should be, exactly.
    pub fn to_slice<'a>(&'a self) -> Option<&'a [u8]> {
        if self.0 >> 30 == 2 {
            let addr = loom_addr(self.0);
            unsafe {
                let len = *addr.offset(1) as usize;
                let p = addr.offset(2) as *const u8;
                Some(slice::from_raw_parts(p, len))
            }
        } else {
            None
        }
    }
}

impl ops::Add for Atom {
    type Output = Atom;

    fn add(self, other: Atom) -> Atom {
        if let (Some(a), Some(b)) = (self.as_direct(), other.as_direct()) {
            let sum = a + b;
            if sum >> 31 != 0 {
                // TODO: Handle overflow.
                unsafe { u3qa_add_orig(self.0, other.0) }
            } else {
                Atom(sum)
            }
        } else {
            unsafe { u3qa_add_orig(self.0, other.0) }
        }
    }
}

/// Rust wrapper for value in the u3 loom that's known to be a cell.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Cell(u3_cell);

impl Cell {
    pub fn as_noun(&self) -> Noun { Noun(self.0) }

    pub fn hed(&self) -> Noun {
        let addr = loom_addr(self.0) as *const Noun;
        unsafe {
            *addr.offset(1)
        }
    }

    pub fn tel(&self) -> Noun {
        let addr = loom_addr(self.0) as *const Noun;
        unsafe {
            *addr.offset(2)
        }
    }
}


#[no_mangle]
pub extern fn u3qa_add(a: u3_atom, b: u3_atom) -> Noun {
    let (a, b) = (Atom(a), Atom(b));
    (a + b).as_noun()
}
