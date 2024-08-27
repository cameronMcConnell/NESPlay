use crate::nes::Nes;

#[test]
fn test_new_nes() {
    let nes = Nes::new();

    assert!(nes);
}