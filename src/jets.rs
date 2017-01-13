use u3_types::*;
use u3_consts::*;
use {Noun, Atom, bail};
use u3a_lose;

#[no_mangle]
pub extern "C" fn u3qa_add(a: Atom, b: Atom) -> Noun {
    (a + b).as_noun()
}

#[no_mangle]
pub extern "C" fn u3wa_add(cor: Noun) -> Noun {
    if let (Some(a), Some(b)) = (cor.axis(u3x_sam_2).and_then(|x| x.as_atom()),
                                 cor.axis(u3x_sam_3).and_then(|x| x.as_atom())) {
        u3qa_add(a, b)
    } else {
        bail(mote!("exit"));
        unreachable!();
    }
}

#[no_mangle]
pub extern "C" fn u3ka_add(a: Atom, b: Atom) -> Noun {
    let ret = (a + b).as_noun();
    // TODO: Rust-side implementation.
    unsafe {
        u3a_lose(a.0);
        u3a_lose(b.0);
    }

    ret
}
