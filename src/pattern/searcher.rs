use super::Pattern;

/// Trait implemented for types that can do pattern search.
pub trait PatternSearcher {
    /// Pattern search output.
    type Output;
    /// Iterator over all occurences.
    type Iter: Iterator<Item = Self::Output>;

    /// Finds an iterator over all occurences of the pattern.
    fn find_all_patterns(&self, pat: Pattern) -> crate::Result<Self::Iter>;

    /// Returns first occurence of the pattern if present.
    fn find_first_pattern(&self, pat: Pattern) -> crate::Result<Option<Self::Output>> {
        Ok(self.find_all_patterns(pat)?.next())
    }
}
