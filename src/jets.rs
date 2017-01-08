use u3_types::*;
use Atom;

#[no_mangle]
pub extern fn u3qa_add(a: u3_atom, b: u3_atom) -> u3_noun {
    let (a, b) = (Atom(a), Atom(b));
    (a + b).0
}
