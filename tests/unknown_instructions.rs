use lcf::raw::lmu::event::{EventChunk, instruction::Instruction, page::EventPageChunk};

#[test]
#[ignore] // takes 3 minutes to run on 2kki (on a hard drive)
fn find_unknown_instructions() {
    // rust-analyzer gave up on this one
    let results = std::fs::read_dir("tests/data/games/")
        .unwrap()
        .filter_map(|dir| dir.map(|dir| dir.path()).ok())
        .filter(|game| game.join("RPG_RT.ldb").exists())
        .filter_map(|game| std::fs::read_dir(&game).ok())
        .flat_map(|entries| {
            entries
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .map(|name| name.ends_with(".lmu"))
                        .unwrap_or_default()
                })
                .map(|file| file.path())
                .filter_map(|path| std::fs::read(&path).ok().zip(Some(std::rc::Rc::new(path))))
                .filter_map(|(bytes, map)| {
                    let mut buf = std::io::Cursor::new(bytes);
                    lcf::raw::lmu::RawLcfMapUnit::read(&mut buf)
                        .unwrap()
                        .0
                        .inner_vec
                        .into_iter()
                        .find_map(|chunk| match chunk.data {
                            lcf::raw::lmu::LcfMapUnitChunk::Events { chunks } => {
                                Some((map.clone(), chunks))
                            }
                            _ => None,
                        })
                })
                .flat_map(|(map, chunks)| {
                    chunks
                        .into_iter()
                        .filter_map(|(event, chunks)| {
                            chunks
                                .inner_vec
                                .into_iter()
                                .find_map(|chunk| match chunk.data {
                                    EventChunk::Pages { chunks } => Some((event.0, chunks)),
                                    _ => None,
                                })
                        })
                        .flat_map(|(event, pages)| {
                            pages
                                .into_iter()
                                .filter_map(|(page, chunks)| {
                                    chunks.inner_vec.into_iter().find_map(|chunk| {
                                        match chunk.data {
                                            EventPageChunk::Commands(commands) => {
                                                Some((page.0, commands))
                                            }
                                            _ => None,
                                        }
                                    })
                                })
                                .flat_map(|(page, commands)| {
                                    commands
                                        .0
                                        .into_iter()
                                        .enumerate()
                                        .map(|(line, command)| {
                                            (map.clone(), event, page, line, command)
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
        })
        .filter(|(_, _, _, _, command)| matches!(command.instruction, Instruction::Unknown { .. }))
        .collect::<Vec<_>>();

    if !results.is_empty() {
        for (map, event, page, line, command) in results {
            println!(
                "Unrecognized instruction in {}, E{event:04}, Page {page}, Line {line}: {:?}",
                map.file_name().unwrap().to_string_lossy(),
                command.instruction
            );
        }
        panic!("Found unknown instructions");
    }
}
