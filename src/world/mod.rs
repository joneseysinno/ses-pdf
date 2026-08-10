mod fonts;
mod vfs;

pub(crate) use vfs::VirtualFs;

use std::path::PathBuf;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

pub(crate) struct SesWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main_id: FileId,
    main_source: Source,
    vfs: VirtualFs,
}

impl SesWorld {
    pub(crate) fn new(source: String, vfs: VirtualFs) -> Result<Self, crate::PdfError> {
        let loaded = fonts::load_fonts()?;

        let vpath = VirtualPath::new("/main.typ")
            .map_err(|_| crate::PdfError::Builder("invalid main.typ path"))?;
        let main_id = RootedPath::new(VirtualRoot::Project, vpath).intern();
        let main_source = Source::new(main_id, source);

        Ok(Self {
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(loaded.book),
            fonts: loaded.fonts,
            main_id,
            main_source,
            vfs,
        })
    }
}

impl World for SesWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            Ok(self.main_source.clone())
        } else {
            Err(FileError::NotFound(PathBuf::from(
                id.vpath().get_without_slash(),
            )))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let key = id.vpath().get_without_slash();
        self.vfs
            .get(key)
            .cloned()
            .ok_or_else(|| FileError::NotFound(PathBuf::from(key)))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        None
    }
}
