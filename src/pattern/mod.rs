extern crate alloc;
use alloc::vec::Vec;

mod searcher;
pub use searcher::*;

use crate::FaitheError;

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
    /// Panics if pattern of invalid style was supplied or failed to parse a byte.
    /// ```
    /// # use faithe::pattern::Pattern;
    /// let ida_pat = Pattern::from_ida_style("48 89 85 F0 00 00 00 4C 8B ? ? ? ? ? 48 8D");
    /// ```
    pub fn from_ida_style(pat: impl AsRef<str>) -> Self {
        assert!(pat.as_ref().is_ascii());

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

    /// Parses ida style pattern. Same as [`Self::from_ida_style`] but no panics.
    /// # Panics
    /// Panics if pattern of invalid style was supplied or failed to parse a byte.
    /// ```
    /// # use faithe::pattern::Pattern;
    /// let ida_pat = Pattern::from_ida_style("48 89 85 F0 00 00 00 4C 8B ? ? ? ? ? 48 8D");
    /// ```
    pub fn try_from_ida_style(pat: impl AsRef<str>) -> crate::Result<Self> {
        if pat.as_ref().is_ascii() {
            Err(FaitheError::NonAsciiPattern)
        } else {
            Ok(Self(
                pat.as_ref()
                    .split_ascii_whitespace()
                    .map(|s| {
                        if s == "?" {
                            Ok(ByteMatch::Any)
                        } else {
                            if let Ok(b) = u8::from_str_radix(s, 16) {
                                Ok(ByteMatch::Exact(b))
                            } else {
                                Err(FaitheError::InvalidPattern)
                            }
                        }
                    })
                    .collect::<crate::Result<Vec<ByteMatch>>>()?,
            ))
        }
    }

    /// Parses PEiD style pattern.
    /// # Panics
    /// Panics if pattern of invalid style was supplied or failed to parse a byte.
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

    /// Parses PEiD style pattern.
    /// # Panics
    /// Panics if pattern of invalid style was supplied or failed to parse a byte.
    /// ```
    /// # use faithe::pattern::Pattern;
    /// let peid_pat = Pattern::from_peid_style("48 89 85 F0 00 00 00 4C 8B ?? ?? ?? ?? ?? 48 8D");
    /// ```
    pub fn try_from_peid_style(pat: impl AsRef<str>) -> crate::Result<Self> {
        if pat.as_ref().is_ascii() {
            Err(FaitheError::NonAsciiPattern)
        } else {
            Ok(Self(
                pat.as_ref()
                    .split_ascii_whitespace()
                    .map(|s| {
                        if s == "??" {
                            Ok(ByteMatch::Any)
                        } else {
                            if let Ok(b) = u8::from_str_radix(s, 16) {
                                Ok(ByteMatch::Exact(b))
                            } else {
                                Err(FaitheError::InvalidPattern)
                            }
                        }
                    })
                    .collect::<crate::Result<Vec<ByteMatch>>>()?,
            ))
        }
    }

    /// Parses code style pattern.
    /// # Panics
    /// Panics if length os mask is not equal to the length of the pattern.
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

    /// Parses code style pattern.
    /// # Panics
    /// Panics if length os mask is not equal to the length of the pattern.
    /// ```
    /// # use faithe::pattern::Pattern;
    /// let code_pat = Pattern::from_code_style(
    ///     b"\x48\x89\x85\xF0\x00\x00\x00\x4C\x8B\x00\x00\x00\x00\x00\x48\x8D",
    ///     b"xxxxxxxxx?????xx"
    /// );
    /// ```
    pub fn try_from_code_style(pat: &[u8], mask: &[u8]) -> crate::Result<Self> {
        if pat.len() != mask.len() {
            Err(FaitheError::PatternMaskMismatch)
        } else {
            Ok(Self(
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
            ))
        }
    }
}

impl Pattern {
    /// Finds all pattern occurences in memory range
    /// Panics
    /// if `from` > `to`
    pub unsafe fn find_all(&self, from: *const u8, to: *const u8) -> impl Iterator<Item = *const u8> + '_ {
        assert!(to as usize >= from as usize);

        std::slice::from_raw_parts(from, to.offset_from(from) as usize)
            .windows(self.len())
            .enumerate()
            .filter(|(_, w)| self.matches(*w))
            .map(move |(i, _)| from.add(i))
    }
}