#![allow(non_camel_case_types)]
#![feature(plugin)]
#![plugin(u3_plugin)]

use std::ops;
use std::slice;
use std::mem;
use gmp::mpz::{Mpz, mpz_struct};
use u3_types::*;

extern crate libc;
extern crate gmp;
extern crate u3_alloc;

pub mod jets;

pub mod u3_types {
    use libc;

    pub type c3_w = libc::uint32_t;
    pub type c3_i = libc::c_int;
    pub type u3_noun = c3_w;
    pub type u3_atom = u3_noun;
    pub type u3_cell = u3_noun;
}

pub mod u3_consts {
    // Conventional axes for gate call.
    pub const u3x_pay: u32 = 3; // payload
    pub const u3x_sam: u32 = 6; // sample
    pub const u3x_sam_1: u32 = 6;
    pub const u3x_sam_2: u32 = 12;
    pub const u3x_sam_3: u32 = 13;
    pub const u3x_sam_4: u32 = 24;
    pub const u3x_sam_5: u32 = 25;
    pub const u3x_sam_6: u32 = 26;
    pub const u3x_sam_12: u32 = 52;
    pub const u3x_sam_13: u32 = 53;
    pub const u3x_sam_7: u32 = 27;
    pub const u3x_sam_14: u32 = 54;
    pub const u3x_sam_15: u32 = 55;
    pub const u3x_con: u32 = 7; // context
    pub const u3x_con_2: u32 = 14; // context
    pub const u3x_con_3: u32 = 15; // context
    pub const u3x_con_sam: u32 = 30; // sample in gate context
    pub const u3x_bat: u32 = 2; // battery
}

extern "C" {
    /// Copy count of words into a new atom.
    fn u3i_words(count: c3_w, data: *const c3_w) -> u3_noun;

    /// Copy atom into GMP value.
    fn u3r_mp(a_mp: *mut mpz_struct, b: u3_atom);

    /// Copy GMP integer into an atom and clear it.
    fn u3i_mp(a_mp: *const mpz_struct) -> u3_noun;

    /// Gain a reference count in normal space.
    fn u3a_gain(som: u3_noun) -> u3_noun;

    /// Lose a reference count
    pub fn u3a_lose(som: u3_noun);

    fn u3m_bail(how: u3_noun) -> c3_i;
}

fn loom_addr(noun: u32) -> *const u32 {
    // TODO: This is from portable.h, with different defines for different platforms, the Rust-side
    // needs that too. Current one is Linux only.
    const U3_OS_LOOM_BASE: u32 = 0x36000000;
    const NOUN_ADDR_MASK: u32 = !(1 << 31 | 1 << 30);

    (U3_OS_LOOM_BASE + (noun & NOUN_ADDR_MASK) * mem::size_of::<u32>() as u32) as *const u32
}

pub fn bail(how: u32) {
    unsafe {
        u3m_bail(how);
    }
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

    pub fn as_cell(&self) -> Option<(Noun, Noun)> {
        if self.0 >> 30 == 3 {
            let addr = loom_addr(self.0) as *const Noun;
            unsafe { Some((*addr.offset(1), *addr.offset(2))) }
        } else {
            None
        }
    }

    /// Return a child of the noun if it's a suitably shaped cell tree.
    ///
    /// The `axis` value must be greater or equal to 1. The axis is the standard Urbit tree
    /// notation, the numbers correspond to left-to-right level order traversal assuming a complete
    /// binary tree and starting with 1 for the root of the tree.
    pub fn axis(&self, axis: u32) -> Option<Noun> {
        // XXX: Do we need panic-handling story for jet code, Rust panics (eg. from a failed
        // assert) propagating past the FFI boundary is unspecified behavior.

        // TODO: Will probably need to support bignum axis parameters at some point.

        // XXX: There's probably a faster implementation than the recursive one.
        assert!(axis > 0);

        if axis == 1 {
            return Some(*self);
        }

        let mut ret = *self;

        // Iterate bits from below most significant bit (msb) in order of decreasing significance,
        // these correspond to a path through the axis tree.
        let msb = mem::size_of::<u32>() as u32 * 8 - axis.leading_zeros();
        for i in (0..(msb - 1)).rev() {
            if let Some((hed, tel)) = ret.as_cell() {
                if axis & (1 << i) == 0 {
                    ret = hed;
                } else {
                    ret = tel;
                }
            } else {
                return None;
            }
        }

        Some(ret)
    }
}

/// Rust wrapper for value in the u3 loom that's known to be an atom.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Atom(u3_atom);

impl Atom {
    pub fn as_noun(&self) -> Noun {
        Noun(self.0)
    }

    pub fn to_mpz(&self) -> Mpz {
        unsafe {
            let mut mp: mpz_struct = mem::uninitialized();
            u3r_mp(&mut mp, self.0);
            mem::transmute(mp)
        }
    }

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

impl From<Mpz> for Atom {
    fn from(mp: Mpz) -> Self {
        unsafe {
            let mp: mpz_struct = mem::transmute(mp);
            Atom(u3i_mp(&mp))
        }
    }
}

impl ops::Add for Atom {
    type Output = Atom;

    fn add(self, other: Atom) -> Atom {
        if let (Some(a), Some(b)) = (self.as_direct(), other.as_direct()) {
            let sum = (a + b) as c3_w;
            unsafe { Atom(u3i_words(1, &sum)) }
        } else if self == Atom(0) {
            // Adding zero to indirect atom, just return the other atom with its refcount
            // incremented.
            unsafe {
                u3a_gain(other.0);
            }
            other
        } else {
            let mut mp = self.to_mpz();
            mp += other.to_mpz();
            mp.into()
        }
    }
}
