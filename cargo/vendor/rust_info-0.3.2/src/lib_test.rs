use super::*;
use doc_comment as _;

#[test]
fn get_test() {
    let rust_info = get();

    assert!(rust_info.version.is_some());
    assert!(rust_info.channel.is_some());
}
