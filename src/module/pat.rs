use super::ModuleEntry;
use crate::{
    pattern::{Pattern, PatternSearcher},
    process::Process,
};
use windows::Win32::System::Threading::PROCESS_VM_READ;

/// Iterator over module pattern occurences.
pub struct ModulePatIter {
    proc: Process,
    pat: Pattern,
    from: usize,
    to: usize,
    buf: Box<[u8]>,
}

impl ModulePatIter {
    pub(crate) fn new(pid: u32, from: usize, to: usize, pat: Pattern) -> crate::Result<Self> {
        let proc = Process::open_by_id(pid, false, PROCESS_VM_READ)?;

        Ok(Self {
            proc,
            from,
            to,
            buf: vec![0; pat.len()].into_boxed_slice(),
            pat,
        })
    }
}

impl Iterator for ModulePatIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.from > self.to - self.pat.len() {
            None
        } else {
            loop {
                if let Err(_) = self
                    .proc
                    .read_process_memory_buf(self.from, &mut self.buf[..])
                {
                    return None;
                }

                if self.pat.matches(&self.buf) {
                    break Some(self.from);
                }
                self.from += 1;
            }
        }
    }
}

impl PatternSearcher for ModuleEntry {
    type Output = usize;
    type Iter = ModulePatIter;

    fn find_all_patterns(&self, pat: Pattern) -> crate::Result<Self::Iter> {
        Self::Iter::new(
            self.process_id,
            self.mod_base_addr,
            self.mod_base_addr + self.mod_base_size,
            pat,
        )
    }
}
