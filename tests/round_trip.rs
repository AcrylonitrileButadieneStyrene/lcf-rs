#[test]
fn database_round_trip() {
    get_games().for_each(|game| {
        round_trip::<lcf::raw::ldb::RawLcfDataBase>(&game.join("RPG_RT.ldb"));
    });
}

#[test]
fn map_tree_round_trip() {
    get_games().for_each(|game| {
        round_trip::<lcf::raw::lmt::RawLcfMapTree>(&game.join("RPG_RT.lmt"));
    });
}

#[test]
fn map_unit_round_trip() {
    get_games().for_each(|game| {
        round_trip::<lcf::raw::lmu::RawLcfMapUnit>(&game.join(find_one(&game, "lmu")));
    });
}

#[test]
fn save_data_round_trip() {
    get_games().for_each(|game| {
        round_trip::<lcf::raw::lsd::RawLcfSaveData>(&game.join(find_one(&game, "lsd")));
    });
}

fn get_games() -> impl Iterator<Item = std::path::PathBuf> {
    std::fs::read_dir("tests/data/games/")
        .unwrap()
        .filter_map(|dir| dir.map(|dir| dir.path()).ok())
        .filter(|dir| dir.join("RPG_RT.ldb").exists())
}

fn find_one(path: &std::path::Path, ext: &str) -> String {
    std::fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .find(|entry| entry.ends_with(ext))
        .unwrap()
}

fn round_trip<T>(path: &std::path::Path)
where
    for<'a> T: binrw::BinRead<Args<'a>: Default> + binrw::BinWrite<Args<'a>: Default>,
    T: binrw::meta::ReadEndian + binrw::meta::WriteEndian,
{
    let bytes = std::fs::read(path).unwrap();
    let mut cursor = std::io::Cursor::new(bytes);
    let database = T::read(&mut cursor).unwrap();
    let mut buffer = std::io::Cursor::new(Vec::new());
    database.write(&mut buffer).unwrap();
    assert_eq!(cursor.into_inner(), buffer.into_inner());
}
