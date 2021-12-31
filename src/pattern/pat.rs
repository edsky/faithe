use super::ByteMatch;
use std::num::ParseIntError;

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
    /// ```
    /// # use radon::pattern::Pattern;
    /// let ida_pat = Pattern::from_ida_style("48 89 85 F0 00 00 00 4C 8B ? ? ? ? ? 48 8D").unwrap();
    /// ```
    pub fn from_ida_style(pat: impl AsRef<str>) -> Result<Self, ParseIntError> {
        pat.as_ref()
            .split_ascii_whitespace()
            .map(|s| {
                if s == "?" {
                    Ok(ByteMatch::Any)
                } else {
                    match u8::from_str_radix(s, 16) {
                        Ok(b) => Ok(ByteMatch::Exact(b)),
                        Err(e) => Err(e),
                    }
                }
            })
            .collect::<Result<Vec<ByteMatch>, ParseIntError>>()
            .map(|m| Self(m))
    }

    /// Parses PEiD style pattern.
    /// ```
    /// # use radon::pattern::Pattern;
    /// let peid_pat = Pattern::from_peid_style("48 89 85 F0 00 00 00 4C 8B ?? ?? ?? ?? ?? 48 8D").unwrap();
    /// ```
    pub fn from_peid_style(pat: impl AsRef<str>) -> Result<Self, ParseIntError> {
        pat.as_ref()
            .split_ascii_whitespace()
            .map(|s| {
                assert_eq!(s.len(), 2);
                if s == "??" {
                    Ok(ByteMatch::Any)
                } else {
                    match u8::from_str_radix(s, 16) {
                        Ok(b) => Ok(ByteMatch::Exact(b)),
                        Err(e) => Err(e),
                    }
                }
            })
            .collect::<Result<Vec<ByteMatch>, ParseIntError>>()
            .map(|m| Self(m))
    }

    /// Parses code style pattern.
    /// ```
    /// # use radon::pattern::Pattern;
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
}
