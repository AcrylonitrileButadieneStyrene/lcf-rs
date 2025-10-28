use crate::helpers::{Array, Chunk, Number, chunk::ChunkTraitBounds};

#[binrw::binrw]
#[derive(Clone, Debug)]
#[brw(little)]
pub struct Array2D<T: ChunkTraitBounds + 'static> {
    #[bw(calc = Number(inner_vec.len() as u32))]
    count: Number,
    #[br(count = count.0)]
    pub inner_vec: Vec<(Number, Array<Chunk<T>>)>,
}

impl<T: ChunkTraitBounds> Array2D<T> {
    #[must_use]
    pub fn to_vec(self) -> Option<Vec<Array<Chunk<T>>>> {
        self.inner_vec
            .into_iter()
            .enumerate()
            .map(|(index, (id, item))| {
                if id.0 as usize == index + 1 {
                    Ok(item)
                } else {
                    Err(())
                }
            })
            .try_collect()
            .ok()
    }
}

impl<T: ChunkTraitBounds> FromIterator<Array<Chunk<T>>> for Array2D<T> {
    fn from_iter<U: IntoIterator<Item = Array<Chunk<T>>>>(iter: U) -> Self {
        Self {
            inner_vec: iter
                .into_iter()
                .enumerate()
                .map(|(index, item)| (Number(index as u32 + 1), item))
                .collect(),
        }
    }
}

impl<T: ChunkTraitBounds> FromIterator<(Number, Array<Chunk<T>>)> for Array2D<T> {
    fn from_iter<U: IntoIterator<Item = (Number, Array<Chunk<T>>)>>(iter: U) -> Self {
        Self {
            inner_vec: iter.into_iter().collect(),
        }
    }
}
