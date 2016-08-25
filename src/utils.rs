pub fn align_up(paddr: PAddr, alignment: usize) {
    let raw = paddr.as_usize();
    let aligned = if raw % alignment == 0 {
        raw
    } else {
        raw + (alignment - (raw % alignment))
    };
    PAddr::from_usize(aligned)
}

pub fn block_count(length: usize, block_length: usize) {
    if length % block_length == 0 {
        length / block_length
    } else {
        length / block_length + 1
    }
}
