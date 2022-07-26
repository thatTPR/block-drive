#[cfg(test)]
pub mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn super_block() {
        let sb = super::SuperBlock::new(0, 1024, 100, false);
        assert_eq!(sb, sb.deserialize(sb.serialize()))
    }
    #[wasm_bindgen_test]
    fn bit_map() {
        let bm = super::BitMap::new(0, 1024, 100);
        assert_eq!(bm, bincode::deserialize(bincode::serialize(&bm)))
        //TODO
    }
    #[wasm_bindgen_test]
    fn inode() {
        let Ino = super::Inode::new(0, false, 0, [0; 100], false, 0, 0, [(0, 0); 12], [0; 12]);
        assert_eq!(Ino, bincode::deserialize(bincode::serialize(&Ino)));
    }
    #[wasm_bindgen_test]
    fn inode_table() {
        let it = super::InodeTable::new(0);
        assert_eq!(it, bincode::deserialize(bincode::serialize(&it)))
    }
}
