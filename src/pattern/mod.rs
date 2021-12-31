mod pat;
pub use pat::*;

mod searcher;
pub use searcher::*;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ByteMatch {
    Exact(u8),
    Any,
}

impl ByteMatch {
    #[inline]
    pub fn matches(&self, b: u8) -> bool {
        match self {
            ByteMatch::Exact(e) => *e == b,
            ByteMatch::Any => true,
        }
    }
}
