use crate::{
    helpers::{Array, Chunk},
    ldb::LcfDataBaseReadError,
    raw::ldb::system::SystemChunk,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct System {}

impl System {
    pub fn with_chunks(
        mut self,
        chunks: Array<Chunk<SystemChunk>>,
    ) -> Result<Self, LcfDataBaseReadError> {
        for chunk in chunks.inner_vec {
            match chunk.data {
                SystemChunk::Unknown { id, bytes } => {
                    return Err(LcfDataBaseReadError::UnknownData(id, bytes));
                }
                _ => (),
            }
        }

        Ok(self)
    }

    pub fn to_chunks(&self) -> Array<Chunk<SystemChunk>> {
        let mut chunks = Vec::new();
        chunks.push(SystemChunk::ID(0u32.into()));

        Array {
            inner_vec: chunks.into_iter().map(Into::into).collect(),
            null_terminated: true,
        }
    }
}
