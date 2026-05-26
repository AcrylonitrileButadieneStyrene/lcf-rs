use std::{io::Seek as _, path::PathBuf};

#[test]
fn raw_database_round_trip() {
    get_games().for_each(|game| {
        raw_round_trip::<lcf::raw::ldb::RawLcfDataBase>(&game.join("RPG_RT.ldb"));
    });
}

#[test]
fn raw_map_tree_round_trip() {
    get_games().for_each(|game| {
        raw_round_trip::<lcf::raw::lmt::RawLcfMapTree>(&game.join("RPG_RT.lmt"));
    });
}

#[test]
fn raw_map_unit_round_trip() {
    get_games()
        .filter_map(|game| find_one(&game, "lmu"))
        .map(|file| raw_round_trip::<lcf::raw::lmu::RawLcfMapUnit>(&file))
        .next()
        .expect("found no data");
}

#[test]
fn raw_save_data_round_trip() {
    get_games()
        .filter_map(|game| find_one(&game, "lsd"))
        .map(|file| raw_round_trip::<lcf::raw::lsd::RawLcfSaveData>(&file))
        .next()
        .expect("found no data");
}

#[test]
fn database_recode_round_trip() {
    get_games().for_each(|game| {
        recode_round_trip::<lcf::ldb::LcfDataBase>(&game.join("RPG_RT.ldb"));
    });
}

#[test]
fn map_tree_recode_round_trip() {
    get_games()
        .for_each(|game| recode_round_trip::<lcf::lmt::LcfMapTree>(&game.join("RPG_RT.lmt")));
}

#[test]
fn map_unit_recode_round_trip() {
    get_games()
        .filter_map(|game| find_one(&game, "lmu"))
        .map(|file| recode_round_trip::<lcf::lmu::LcfMapUnit>(&file))
        .next()
        .expect("found no data");
}

#[test]
fn save_data_recode_round_trip() {
    get_games()
        .filter_map(|game| find_one(&game, "lsd"))
        .map(|file| recode_round_trip::<lcf::lsd::LcfSaveData>(&file))
        .next()
        .expect("found no data");
}

fn get_games() -> impl Iterator<Item = std::path::PathBuf> {
    std::fs::read_dir("tests/data/games/")
        .unwrap()
        .filter_map(|dir| dir.map(|dir| dir.path()).ok())
        .filter(|dir| dir.join("RPG_RT.ldb").exists())
}

fn find_one(path: &std::path::Path, ext: &str) -> Option<PathBuf> {
    std::fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .find(|entry| entry.ends_with(ext))
        .map(|file| path.join(file))
}

fn raw_round_trip<T>(path: &std::path::Path)
where
    for<'a> T: binrw::BinRead<Args<'a>: Default> + binrw::BinWrite<Args<'a>: Default>,
    T: binrw::meta::ReadEndian + binrw::meta::WriteEndian,
{
    let bytes = std::fs::read(path).unwrap();
    let mut cursor = std::io::Cursor::new(bytes);
    let data = T::read(&mut cursor).unwrap();
    let mut buffer = std::io::Cursor::new(Vec::new());
    data.write(&mut buffer).unwrap();
    assert_eq!(cursor.into_inner(), buffer.into_inner());
}

fn recode_round_trip<T: lcf::ConvertExt + PartialEq + std::fmt::Debug>(path: &std::path::Path)
where
    <T as TryFrom<<T as lcf::ConvertExt>::Raw>>::Error: From<binrw::Error> + std::fmt::Debug,
{
    println!("current: {path:?}");
    let bytes = std::fs::read(path).unwrap();
    let mut cursor = std::io::Cursor::new(bytes);
    let before = T::read(&mut cursor).unwrap();
    let mut buffer = std::io::Cursor::new(Vec::new());
    before.write(&mut buffer).unwrap();
    buffer.rewind().unwrap();
    let after = T::read(&mut buffer).unwrap();
    assert_eq!(before, after);
}
