use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax::ext::quote::rt::Span;

pub fn expand_mote(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    if args.len() != 1 {
        cx.span_err(sp,
                    &format!("argument should be a single string, but got {} arguments",
                             args.len()));
        return DummyResult::any(sp);
    }

    let text = match args[0] {
        TokenTree::Token(_, token::Literal(token::Str_(n), _)) => n.as_str(),
        _ => {
            cx.span_err(sp, "argument should be a single string literal");
            return DummyResult::any(sp);
        }
    };

    if text.len() > 4 {
        cx.span_err(sp, "motes can be at most 4 characters");
        return DummyResult::any(sp);
    }

    let mut ret = 0u32;

    for c in text.chars().rev() {
        let c = c as u32;
        if c >= 128 {
            cx.span_err(sp, "motes can only have 7-bit ASCII characters");
            return DummyResult::any(sp);
        }

        ret <<= 8;
        ret |= c;
    }

    MacEager::expr(cx.expr_u32(sp, ret))
}
