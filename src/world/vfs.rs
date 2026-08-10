use std::collections::HashMap;
use typst::foundations::Bytes;

#[derive(Default)]
pub(crate) struct VirtualFs(HashMap<String, Bytes>);

impl VirtualFs {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, path: impl Into<String>, data: impl Into<Vec<u8>>) {
        self.0.insert(path.into(), Bytes::new(data.into()));
    }

    pub(crate) fn get(&self, path: &str) -> Option<&Bytes> {
        self.0.get(path)
    }
}
