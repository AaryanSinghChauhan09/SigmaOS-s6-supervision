// Integration test for SigmaOS-s6-supervision
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use s6_supervision as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
