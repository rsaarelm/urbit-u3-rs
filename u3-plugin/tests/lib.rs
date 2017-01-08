#![feature(plugin)]
#![plugin(u3_plugin)]

#[test]
fn test_mote() {
    assert_eq!(0, mote!(""));
    assert_eq!(7303014, mote!("foo"));
}
