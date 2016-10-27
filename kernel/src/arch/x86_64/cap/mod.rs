mod paging;

pub use self::paging::{PageHalf, PageFull, PML4Half, PML4Full,
                       PDPTHalf, PDPTFull, PDHalf, PDFull,
                       PTHalf, PTFull};

use cap::{MDB, CPoolHalf, Cap};

#[derive(Debug)]
pub enum ArchCap {
    PDPT(PDPTFull),
    PD(PDFull),
    PT(PTFull),
}

impl ArchCap {
    pub unsafe fn set_mdb(&mut self, cpool: CPoolHalf, cpool_index: usize) {
        match self {
            &mut ArchCap::PDPT(ref mut full) => full.set_mdb(cpool, cpool_index),
            &mut ArchCap::PD(ref mut full) => full.set_mdb(cpool, cpool_index),
            &mut ArchCap::PT(ref mut full) => full.set_mdb(cpool, cpool_index),
        }
    }

    pub fn mdb(&self, index: usize) -> &MDB {
        match self {
            &ArchCap::PDPT(ref full) => full.mdb(index),
            &ArchCap::PD(ref full) => full.mdb(index),
            &ArchCap::PT(ref full) => full.mdb(index),
        }
    }

    pub fn mdb_mut(&mut self, index: usize) -> &mut MDB {
        match self {
            &mut ArchCap::PDPT(ref mut full) => full.mdb_mut(index),
            &mut ArchCap::PD(ref mut full) => full.mdb_mut(index),
            &mut ArchCap::PT(ref mut full) => full.mdb_mut(index),
        }
    }
}

impl From<PDPTFull> for Cap { fn from(full: PDPTFull) -> Cap { Cap::Arch(ArchCap::PDPT(full)) } }
impl From<PDFull> for Cap { fn from(full: PDFull) -> Cap { Cap::Arch(ArchCap::PD(full)) } }
impl From<PTFull> for Cap { fn from(full: PTFull) -> Cap { Cap::Arch(ArchCap::PT(full)) } }
