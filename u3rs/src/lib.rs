use std::fmt;
use std::slice;

extern crate u3_alloc;

extern {
    //static U3_OS_LoomBase: *const u32;
}

// XXX: From portable.h
const U3_OS_LOOM_BASE: *const u32 = 0x36000000 as *const u32;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct LoomNoun(u32);

/*
impl fmt::Debug for LoomNoun {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.0 & (1 << 31) == 0 {
            write!(fmt, "Direct atom {}", self.0)
        } else {
            let addr_mask = !(1<<31) | !(1<<30); // Turn off two highest bits.
            let addr = unsafe { U3_OS_LoomBase.offset((self.0 & addr_mask) as isize / 4) };
            if self.0 & (1 << 30) != 0 {
                // Indirect atom
                let mug = unsafe { *addr };
                let len = unsafe { *addr.offset(1) };
                write!(fmt, "Indirect atom {} len {}", mug, len)
            } else {
                // Cell
                unimplemented!();
            }
        }
    }
}
*/

const NOUN_ADDR_MASK: u32 = !(1<<31) | !(1<<30);
impl LoomNoun {
    fn loom_addr(self) -> *const u32 {
        unsafe { U3_OS_LOOM_BASE.offset((self.0 & NOUN_ADDR_MASK) as isize / 4) }
    }

    pub fn is_direct(self) -> bool {
        self.0 & (1 << 31) == 0
    }

    pub fn is_atom(self) -> bool {
        self.is_direct() || self.0 >> 30 == 2
    }

    pub fn as_atom(self) -> Option<Atom> {
        if self.is_direct() {
            Some(Atom::Direct(self.0))
        } else if self.is_atom() {
            unsafe {
                let len = *self.loom_addr().offset(1) as usize;
                let data = self.loom_addr().offset(2);
                Some(Atom::Indirect(slice::from_raw_parts(data as *const u8, len)))
            }
        } else {
            None
        }
    }

    pub fn as_cell(self) -> Option<Cell> {
        if !self.is_atom() {
            unsafe {
                Some(Cell {
                    mug: *self.loom_addr(),
                    hed: LoomNoun(self.loom_addr().offset(1) as u32),
                    tel: LoomNoun(self.loom_addr().offset(2) as u32),
                })
            }
        } else {
            None
        }
    }
}

pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

// XXX: This does not match the data layout in loom, Rust code will generate new data when
// instantiating these.
pub enum Atom {
    Direct(u32),
    Indirect(&'static [u8]),
}

#[repr(C)]
pub struct Cell {
    pub mug: u32,
    pub hed: LoomNoun,
    pub tel: LoomNoun,
}


// TODO: FFI type conventions story.
type c_u3_atom = u32;
type c_u3_noun = u32;

#[no_mangle]
pub extern fn u3qa_add(a: LoomNoun, b: LoomNoun) -> LoomNoun {
    if let (Some(a), Some(b)) = (a.as_atom(), b.as_atom()) {
        use Atom::*;
        match (a, b) {
            (Direct(a), Direct(b)) => {
                let sum = a + b;
                // TODO: Handle overflow.
                assert!(sum & (1<<31) == 0);
                return LoomNoun(sum);
            }
            _ => {
                unimplemented!();
            }
        }
    } else {
        // TODO: Figure out u3 error handling conventions
        panic!("Trying to add a cell");
    }
}
