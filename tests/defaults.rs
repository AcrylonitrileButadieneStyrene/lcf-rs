#[test]
fn default_map() {
    let bytes = std::fs::read("tests/data/empty.lmu").unwrap();
    let empty = lcf::lmu::LcfMapUnit::read(&mut std::io::Cursor::new(bytes)).unwrap();
    let default = lcf::lmu::LcfMapUnit::default();
    assert_eq!(empty, default);
}
