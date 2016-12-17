extern crate u3_alloc;

/// Type of tagged noun in u3.
pub enum NounTag {
    // 31-bit direct atom.
    Cat,
    // Indirect atom.
    Pug,
    // Indirect cell.
    Pom,
}

pub enum Atom {
    Direct(u32),
    Indirect(&'static [u8]),
}

pub fn as_atom(ptr: *const u32) -> Option<Atom> {
    unsafe {
        if *ptr & (1 << 31) == 0 {
            // is_cat, direct atom.
            Some(Atom::Direct(*ptr))
        } else if *ptr * (1 << 30) != 0 {
            // is_pug, indirect atom.
            // TODO: Loom ptr construction
            // - Snip off two highest bits.
            // - Add the remain to C global u3_Loom.
            // - Get the length and build the slice
            unimplemented!();
        } else {
            // is_pom, indirect cell (not an atom)
            None
        }
    }
}

impl NounTag {
    pub fn new(word: u32) -> NounTag {
        if word & (1<<31) == 0 {
            NounTag::Cat
        } else if word & (1 << 30) != 0 {
            NounTag::Pom
        } else {
            NounTag::Pug
        }
    }
}


#[no_mangle]
pub extern fn hello(x: usize) {
    println!("Hello, {}", x);
}
