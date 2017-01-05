use std::slice;

extern crate u3_alloc;

extern {
    //static U3_OS_LoomBase: *const u32;
}


fn loom_addr(noun: u32) -> *const u32 {
    // TODO: This is from portable.h, with different defines for different platforms, the Rust-side
    // needs that too. Current one is Linux only.
    const U3_OS_LOOM_BASE: u32 = 0x36000000;
    const NOUN_ADDR_MASK: u32 = !(1<<31) | !(1<<30);

    (U3_OS_LOOM_BASE + (noun & NOUN_ADDR_MASK)) as *const u32
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Noun(u32);

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


#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Atom(u32);

impl Atom {
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

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Cell(u32);

impl Cell {
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


// TODO: FFI type conventions story.
type c_u3_atom = u32;
type c_u3_noun = u32;

#[no_mangle]
pub extern fn u3qa_add(a: Noun, b: Noun) -> Noun {
    if let (Some(a), Some(b)) = (a.as_atom(), b.as_atom()) {
        if let (Some(a), Some(b)) = (a.as_direct(), b.as_direct()) {
            let sum = a + b;
            // TODO: Handle overflow.
            assert!(sum & (1<<31) == 0);
            return Noun(sum);
        } else {
            unimplemented!();
        }
    } else {
        // TODO: Figure out u3 error handling conventions
        panic!("Trying to add a cell");
    }
}
