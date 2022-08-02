// use bitvec::{order::Lsb0, vec::BitVec};
use serde::{Deserialize, Serialize};

use ic_cdk::api::stable::{stable64_read, stable64_write};
use std::mem::size_of;

// Utility functions needed for objects to avoid code duplication, probably could have been used as a trait but why stray away from data orientation?

//  use super::_read as _read ;
//  use super::_write as _write ;

#[inline(always)]
pub fn block_group_size(blk_size: u64) -> u64 {}

#[inline(always)]
pub fn inode_table_size(blk_size: u64) -> u64 {}
pub fn inode_size() {}
#[inline(always)]
pub fn data_table_size(blk_size: u64) -> u64 {
    blk_size * blk_size * 8
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SuperBlock {
    pub offset: u64,
    pub sb_size: u32,
    pub dangle: bool,

    pub bitMap_size: u32,
    pub inode_size: u32,
    pub block_size: u32,

    pub block_count: u32,
    pub inode_count: u32,

    pub bitmap: u64,
    pub inode_table: u64,
    pub root_inode: u64,

    pub max_blocks: u32,
}

impl SuperBlock {
    pub fn new(fs_start: u64, block_size: u8, max_blocks: u64, dangled: bool) -> Self {
        let mut sb = Self {
            offset: fs_start,
            sb_size: size_of::<SuperBlock>(),
            dangle: dangled,

            inode_size: size_of::<Inode>(), //
            bitMap_size: size_of::<BitMap>(),
            block_size: block_size,
            block_count: 0,
            inode_count: 0,

            bitmap: fs_start + size_of::<SuperBlock>(),
            inode_table: fs_start + size_of::<SuperBlock>() + size_of::<BitMap> > (),
            root_inode: fs_start
                + size_of::<SuperBlock>()
                + size_of::<BitMap>()
                + size_of::<InodeTable>(),

            max_blocks: max_blocks,
        };
    }
    #[inline(always)]
    pub fn write(self, fs_start: u64) {
        stable64_write(fs_start, bincode::serialize::<SuperBlock>(&self));
    }
    #[inline(always)]
    pub fn read(self, offset: u64) {
        let buf: &mut [u8];
        stable64_read(offset, buf);
        let sb = bincode::deserialize::<SuperBlock>(buf);
        return sb;
    }

    pub fn set_dangled(self) {
        self.dangle = true;
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BitMap {
    // pub data_bitmap: [(bool, u8); n], //
    pub offset: u64,
    pub block_start: u64,
    pub free_blocks: u16,
    pub free_inodes: u16,
    pub free_extents: u16,

    pub allocated_blocks: u64,

    pub inode_bitmap: [bool; super::fileSys::MAX_BLOCKS / 32], // Four times smaller than blockCount . I think this is probalby pretty good. I don't expect it to get bigger so this is probably fine.
    pub extent_bitmap: [(bool, Extent); super::fileSys::MAX_BLOCKS / 8],
}

impl BitMap {
    pub fn new(blockNo: u32, offset: u32, block_start: u64) -> Self {
        let bm = Self {
            // Initialized with None This is the default state , memory has not yet been grown,
            // or the bitmap has not yet been initialized.
            offset: offset,
            block_start: block_start,
            free_blocks: 0,
            free_inodes: 0,
            free_extents: 0,

            allocated_blocks: 0,

            inode_bitmap: [None; 1..super::fileSys::MAX_SIZE],
            extent_bitmap: [(None, (block_start, super::fileSys::MAX_BLOCKS));
                1..super::fileSys::MAX_SIZE],
        };

        return bm;
    }

    fn remainingBlocks(&self) {
        return super::fileSys::MAX_BLOCKS - self.allocated_blocks;
    }
    fn find_extent(&self, extent: Extent) -> u64 {
        for i in self.extent_bitmap {
            if i.1 == extent {
                return i.0;
            }
        }
        return -1; // Return -1 if not found
    }

    fn write(&self, offset: u64) {
        stable64_write(offset, bincode::serialize(self));
    }
    pub fn read(offset: u64) -> Self {
        let bytes: &[u8; size_of::<BitMap>()];
        stable64_read(offset, bytes);
        return bincode::deserialize(bytes);
    }
    fn add_extent(&mut self, extent: Extent) {
        self.extent_bitmap.push((true, extent));
    }
    fn free_extent(&mut self, loc: u8) {
        self.extent_bitmap[loc] = (false, self.extent_bitmap[loc].1);
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InodeTable {
    pub offset: u64,
    pub inodes: [Inode; super::fileSys::MAX_BLOCKS / 32],
}

pub type Extent = (u64, u8); //

impl InodeTable {
    pub fn new(offset: u64) -> Self {
        Self {
            offset: offset,
            inodes: [None; super::fileSys::MAX_BLOCKS / 32],
        }
    }

    pub fn write(&self, offset: u64) {
        stable64_write(offset, bincode::serialize(self));
    }
    pub fn read(offset: u64) {
        let bytes: &[u8; size_of::<InodeTable>()];
        stable64_read(offset, bytes);
        return bincode::deserialize(bytes);
    }
    pub fn add_extent_to_inode(&mut self, inode: u32, extent: Extent) {
        self.inodes[inode].extents.push(extent);
    }
    pub fn add_inode(&mut self, inode: Inode) {
        self.inodes.push(inode);
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Inode {
    // Size 2608 bits, 326 bytes
    pub offset: u64,
    pub parent: u64,     //  root points to itself
    pub name: [u8; 100], // The name of the node
    pub dir: bool,       // True if directory, false if file, None if Indirect
    pub dangle: bool, // True if inode is dangling, false if it is not dangled, None if it is present
    pub block_count: u32,

    pub size: u64,                  // Actual size  ( not block size or size on "disk")
    pub extents: [Extent; 12], // Location, size in blocks ,  a location may only be a multiple of the block count + baseOffset
    pub indirect_inodes: [u64; 12], // Location of indirect inodes
}
// Explanation of dangling
// Canister n:
// /dir |-file1 (fully stored, all extents are internal)
//      |-file1 ( dangling there are some extents which are external to the canister) -> Dangling dir stores extents
//Canister n+1
// /dir |-file1 (dangled) stores rest of extents in this canister this makes all parents dangling. eases retrieval should be the last file in alphabetical order. Meaning order
// A dangling Inode will only be created when all blocks have been fully exhausted, (meaning allocated)
// If all blocks are indeed exhausted then the writer stream will be passed to the next canister and the inode will be marked as dangling.
// The recorded size will be the actual size but the block_count will be only the number of blocks allocated to inode in this canister.
// There will be an indirect Inode entry signifying the dangled inode in the canister with the location of the dangled inode ( the address will simply be overflowing . It will point )
impl Inode {
    pub fn new(
        offset: u64,
        dir: bool,
        parent: u32,
        name: [u8; 100],
        dangle: bool,
        block_count: u32,
        size: u64,
        extents: [Extent; 12],
        indirect_inodes: [u64; 12],
    ) -> Self {
        Self {
            offset: offset,
            dir: dir,
            parent: parent,
            name: name,
            dangle: dangle,
            block_count: block_count,
            size: size,
            extents: extents,
            indirect_inodes: indirect_inodes,
        }
    }
    fn check_dir(&self) -> bool {
        return self.dir;
    }
    fn check_file(&self) -> bool {
        return !self.dir;
    }
    fn check_indirect(&self) -> bool {
        if self.dir == None {
            return true;
        } else {
            return false;
        }
    }
    fn get_all_dir(&self) -> [u64; 12] {}
    fn get_all_file_indirects(&self) {
        self.check_file();

        let vector = Vec::new();
        for i in self.indirect_inodes {}
    }
    fn get_dir_children(self) {
        for i in self.indirect_inodes {}
    }
    pub fn read(offset: u64) {
        let bytes: [u8; size_of::<Inode>()];
        stable64_read(offset, &bytes);
        return bincode::deserialize::<Inode>(&bytes);
    }
    pub fn write(&self, offset: u64) {
        stable64_write(offset, bincode::serialize(self));
    }
}
