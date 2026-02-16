use std::io::Cursor;

use lcf::{ConvertExt, lmt::LcfMapTree, lmu::LcfMapUnit};
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

#[test]
#[ignore]
fn find_unreadable_maps() {
    let games = std::fs::read_dir("tests/data/games/")
        .unwrap()
        .filter_map(|dir| dir.map(|dir| dir.path()).ok())
        .collect::<Vec<_>>();

    let lmts = games
        .into_iter()
        .filter_map(|game| {
            let path = game.join("RPG_RT.lmt");
            let bytes = std::fs::read(&path).ok();
            Some(game).zip(bytes.and_then(|bytes| LcfMapTree::read(&mut Cursor::new(bytes)).ok()))
        })
        .collect::<Vec<_>>();

    let results = lmts
        .into_par_iter()
        .flat_map(|(game, lmt)| {
            lmt.maps
                .into_par_iter()
                .filter_map(move |map| {
                    let path = game.join(format!("Map{:>04}.lmu", map.0));
                    let bytes = std::fs::read(&path).ok();
                    Some(path).zip(bytes)
                })
                .map(|(path, bytes)| (path, LcfMapUnit::read(&mut Cursor::new(bytes))))
                .filter(|(_, map)| map.is_err())
        })
        .collect::<Vec<_>>();

    if !results.is_empty() {
        for (path, err) in results {
            println!("{path:?}: {err:?}");
        }
        panic!("Found unknown instructions");
    }
}
