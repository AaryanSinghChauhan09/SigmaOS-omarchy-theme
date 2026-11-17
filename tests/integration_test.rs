// Integration test for SigmaOS-omarchy-theme
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use omarchy_theme as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
