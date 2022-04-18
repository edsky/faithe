use super::ByteMatch;
use alloc::vec::Vec;
extern crate alloc;

/// Memory pattern
#[derive(Debug, Clone)]
pub struct Pattern(pub(crate) Vec<ByteMatch>);

impl Pattern {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn matches(&self, data: &[u8]) -> bool {
        data.iter().zip(self.0.iter()).all(|(b, m)| m.matches(*b))
    }
}

impl Pattern {
    /// Parses ida style pattern.
    /// # Panics
    /// Panics if pattern of invalid style was supplied.
    /// ```
    /// # use faithe::pattern::Pattern;
    /// let ida_pat = Pattern::from_ida_style("48 89 85 F0 00 00 00 4C 8B ? ? ? ? ? 48 8D");
    /// ```
    pub fn from_ida_style(pat: impl AsRef<str>) -> Self {
        Self(
            pat.as_ref()
                .split_ascii_whitespace()
                .map(|s| {
                    if s == "?" {
                        ByteMatch::Any
                    } else {
                        ByteMatch::Exact(
                            u8::from_str_radix(s, 16).expect("Failed to parse the pattern."),
                        )
                    }
                })
                .collect::<Vec<ByteMatch>>(),
        )
    }

    /// Parses PEiD style pattern.
    /// # Panics
    /// Panics if pattern of invalid style was supplied.
    /// ```
    /// # use faithe::pattern::Pattern;
    /// let peid_pat = Pattern::from_peid_style("48 89 85 F0 00 00 00 4C 8B ?? ?? ?? ?? ?? 48 8D");
    /// ```
    pub fn from_peid_style(pat: impl AsRef<str>) -> Self {
        Self(
            pat.as_ref()
                .split_ascii_whitespace()
                .map(|s| {
                    assert_eq!(s.len(), 2);
                    if s == "??" {
                        ByteMatch::Any
                    } else {
                        ByteMatch::Exact(
                            u8::from_str_radix(s, 16).expect("Failed to parse the pattern."),
                        )
                    }
                })
                .collect::<Vec<ByteMatch>>(),
        )
    }

    /// Parses code style pattern.
    /// ```
    /// # use faithe::pattern::Pattern;
    /// let code_pat = Pattern::from_code_style(
    ///     b"\x48\x89\x85\xF0\x00\x00\x00\x4C\x8B\x00\x00\x00\x00\x00\x48\x8D",
    ///     b"xxxxxxxxx?????xx"
    /// );
    /// ```
    pub fn from_code_style(pat: &[u8], mask: &[u8]) -> Self {
        assert_eq!(pat.len(), mask.len());

        Self(
            pat.iter()
                .zip(mask.iter())
                .map(|(p, m)| {
                    if *m == b'?' {
                        ByteMatch::Any
                    } else {
                        ByteMatch::Exact(*p)
                    }
                })
                .collect(),
        )
    }

    /// Searches for all occurences in the specified memory region.
    pub unsafe fn find_all(&self, start_region: *const u8, region_size: usize) -> impl Iterator<Item = *const u8> + '_ {
        core::slice::from_raw_parts(
            start_region,
            region_size
        ).windows(self.len())
        .enumerate()
        .filter(|(_, s)| self.matches(s))
        .map(move |(i, _)| start_region.add(i))
    }

    /// Searches for the first occurence of pattern in the specified memory region.
    pub unsafe fn find_first(&self, start_region: *const u8, region_size: usize) -> Option<*const u8> {
        core::slice::from_raw_parts(
            start_region,
            region_size
        ).windows(self.len())
        .enumerate()
        .find(|(_, s)| self.matches(s))
        .map(|(i, _)| start_region.add(i))
    }
}
