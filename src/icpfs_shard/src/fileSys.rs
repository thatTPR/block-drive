use ic_cdk::storage ;
use ic_cdk::api::stable ;
use ic_cdk::api ;

use std::{collections::BTreeMap, path::Path};
use wasm_tracing_allocator::WasmTracingAllocator ;
use crate::{types::{BitMap, SuperBlock}, path};

// use wasm_bindgen::prelude::wasm_bindgen
use super::types ;
use std::collections::HashMap;
use std::mem::size_of ;
use std::option ;

 use ic_cdk::api::stable::{stable64_read as stable64_read  , stable64_write as stable64_write};
use serde::{Deserialize, Serialize};

pub static MAX_SIZE :u32 = 1024* 1024 * 1024 * 7.9 ;  // I left some pretty hefty slack space out of paranoia Theoretically we have 100 mb of extra stable space . Who knows it might come in handy
pub static MAX_HEAP: u32 = 1024 * 1024 * 1024 * 3.5; // There is no fucking way I am using more than that . 
pub static MAX_BLOCKS: u32 = MAX_SIZE % 1024 ; // This is the maximum number of blocks allowed in the filesystem

pub static BLOCK_FLUSH: u32 = 2048* 1024 ; // With a block size of 1kby that gives 2gb max allocated blocks, Everything else does not go over a few mb so no need to worry about that

struct bitmap { // Mirror of types::BitMap
     pub offset: bool,

    pub free_blocks: bool,
    pub free_inodes: bool,
    pub free_extents: bool,

    pub allocated_blocks: bool,

    pub inode_bitmap: [bool; super::fileSys::MAX_BLOCKS / 32], // Four times smaller than blockCount . I think this is probalby pretty good. I don't expect it to get bigger so this is probably fine.
    pub extent_bitmap: [(bool, types::Extent); super::fileSys::MAX_BLOCKS / 8],

}
impl bitmap {
    fn new() {
        Self {
            offset: false,
            free_blocks: false,
            free_inodes: false,
            free_extents: false,
            allocated_blocks: false,
            inode_bitmap: [false; super::fileSys::MAX_BLOCKS / 32], // Four times smaller than blockCount . I think this is probalby pretty good. I don't expect it to get bigger so this is probably fine.
            extent_bitmap: [(false, (1..1024)); super::fileSys::MAX_BLOCKS / 8],
        }
    }
}
struct inodeTable{
    pub offset: bool,
    pub inodes: [(bool, Inode); super::fileSys::MAX_BLOCKS / 32],
}
impl inodeTable {
    fn new() {
        Self {
            offset: false,
            inodes: [(false, Inode::new()); super::fileSys::MAX_BLOCKS / 32],
        }
    }
}

pub struct Inode {
    pub offset: bool,
    pub parent: bool,     //  root points to itself
    pub name: bool, // The name of the node
    pub dir: bool,       // True if directory, false if file, None if Indirect
    pub dangle: bool, // True if inode is dangling, false if it is not dangled, None if it is present
    pub block_count: bool,

    pub size: bool,                  // Actual size  ( not block size or size on "disk")
    pub extents: [bool; 12], 
    pub indirect_inodes: [bool; 12],
}
impl  Inode {
    fn new() {
        Self {
            offset: false,
            parent: false,
            name: false,
            dir: false,
            dangle: false,
            block_count: false,
            size: false,
            extents: [false; 12],
            indirect_inodes: [false; 12],
        }
    }
    fn walk(self){
   
    }
    fn get_dir_children(self){
        for i in self.indirect_inodes {
            
        }
    }
    fn get_file_children(self){

    }
    fn get_all(self){
        let t = Vec::new();
        for i in self {
            t.push(i);
        }
        return t 
    }
}


type changed = bool ;
pub struct dataMap{ // Non stable writes to stable,  Everything allocated read or written is done through this. Pseudo allocate on flush. Everything is allocated when a trigger command is made.
    sb: types::SuperBlock,
    bitMap: types::BitMap,
    bitMapMap: bitmap ,
    InodeTable: types::InodeTable,
    InodeTableMap: inodeTable,

    blockNum: u64 , // Must not go over BLOCK_FLUSH
    

    extents: HashMap<u64, Vec<( (types::Extent , changed ), Vec<([u8; 1024] , changed )>  )>>, // K: Index of inode that points to this in InodeTable, // Root holds everything duh
   
    limit: u32

}

impl dataMap {
    fn new(sb: types::SuperBlock, bitMap: types::BitMap, inodes: types::InodeTable,extents:HashMap<u64, (Vec<(types::Extent, Vec<[u8; 1024]>)>)> )-> Self{
        
        Self {
            sb: sb,
            bitMap: bitMap,
            bitMapMap: bitmap::new(),
            InodeTable: inodes,
            InodeTableMap: inodeTable::new(),
            extents: extents,
            blockNum: extents,
            extents: {
                let mut map = HashMap::new();
                map.insert(0, Vec::new());  
            },
            limit: BLOCK_FLUSH ,
             
            }

        }
    fn check_flush(&self){ // Checks if the block num is over the limit (or the cycles index has been reached) and if so flushes the data to stable, deallocationg the blocks in the extent data structure
           if self.blockNum > self.limit {
               self.optimize();
               self.flush();
           }
            else {
            self.optimize()
           }
          self.check_cycles();
       }
    fn bitmap_find_extent(&self, extent : types::Extent){
        for i in self.bitMap.extent_bitmap {
            if extent == i {return &i;}
            else {
                return false ;
            }
        }
    }
    fn merge_extents(&self,mut inode: types::Inode,  mut extent: u8, mut extentto: u8 )
    {
        inode.extents[extent] = (inode.extents[extent], (inode.extents[extentto][0] - inode.extents[extent][0]) as u8 + inode.extents[extentto][1]);

        let ext = self.bitmap_find_extent(extent) ;
        if ext == 0  {
            todo!()
        }
        else {
            self.bitMap.extent_bitmap.add(ext);
        }
        
        self.bitMap.extent_bitmap;
    }
    fn optimize_extents(&self){ 
        for i in self.InodeTable.inodes {
            let k = i.extents[0];
            for j in i.extents{
                if k.0 + k.1 + 1 == j.0 {
                   self.merge_extents(i, k.0, k.1);
                } 
            }
        }
    } 
    
    fn optimize(&self){ // TODO 
        for i in self.InodeTable.inodes {
        
    }
    } 
    fn flush(&self){
        self.sb.writeStable(self.superBlock.offset, self.superBlock);
        self.bitMap.writeStable(self.sb.bitmap);
        self.InodeTable.writeStable(self.sb.inode_table);
        for i in self.InodeTable.inodes{
            
        }
    }
    fn get_extent_blocks(extent: types::Extent){ 
        return extent.1 ;
    }
    fn get_block_num(self)-> u64 { 
        let t = 0 ;
        for i in self.extents {
            let t = t + i.0 ;
    }}

    fn cache_inode_block_num(self, blockNum: u64){
        self.blockNum = blockNum ;
    }
    fn cache_extents_from_stable_inode(self, Inodeoffset: u64){
        let Inode : types::Inode = types::Inode::read(Inodeoffset);
        
    }   
    fn getInodeExtents(self, inode: u64) -> Vec<u64> {
        
        let mut extents = Vec::new();

        for i in inode {
            let mut extents = Vec::new();
            for j in i.extents {

            }
        }
    }

    fn allocate_extent(self, mut Inode: types::Inode, mut extent: types::Extent){
        let mut extents = self.extents.get_mut(&Inode.inode_num).unwrap();
        extents.push((extent, 0));
        self.bitMap.allocate_extent(extent);
        self.InodeTable.inodes[Inode.inode_num] = Inode;
    } 
    fn deallocate_extent(self , mut Inode: types::Inode, mut extent: types::Extent){
        
    }
    fn removeInode(self, loc: u64){
        self.inodes.remove(&loc);
    }
    fn persist_inode(self, loc:u64){ 
        self.InodeTable.inodes[loc].write(self.InodeTable.inodes[loc], self.InodeTable.inodes.loc);
        self.persist_inode_data(loc);
        self.persist_inode_contents(loc);
    }
    fn persist_inode_data(self, loc:u64){
        let inode = self.InodeTable.inodes[loc];
    }
    fn persist_inode_contents(&self, loc:u64){
        for i in self.extents[&loc] {
            
            let cursor = i.0.0 ;
            let range = i.0.1 ;
            for j in i.1 {
                stable64_write(cursor, &j) ;
                cursor = cursor + 1024  ;
            }
            

        }     
    }
    fn persist_extent(self, extent: (types::Extent , Vec<[u8;1024]>)) {

    }
    fn free_Inode(self, loc: u64, Index: u32 ) {
       
    }
}

struct Handle{
    Inode: types::Inode,
    cursor: u64,
}
impl Handle {
    fn new(inode: u64,  cursor: u64) -> Self{
        Self {
            Inode: inode,
            cursor: cursor,
        }
    }
    fn setInode(self, Inode: u64)
    {
        self.Inode = Inode ;
    }
    fn getInode(self) {
        return self.Inode ;
    }
    fn setCursor(self, value: u64 ){
        self.cursor = value ;
    }
}

pub struct Fs {
    pub dataMap: dataMap ,
    pub handles: Vec<Handle>,
    pub offset: u64 , 
    pub cursor: u64 
}

impl Fs { 
    pub fn new(blocksize: u32, start: u32 ) 
    {
        let sb = types::SuperBlock::new(start, blocksize, None, MAX_BLOCKS) ;
        let bm  = types::BitMap::new(MAX_BLOCKS, (start + size_of<types::SuperBlock>), ( start +  size_of<types::BitMap>())) ; 

        let it  = types::InodeTable::new(start + size_of<types::SuperBlock> + size_of<types::BitMap>() ) ; 
        let ex = HashMap::new() ;
        let mut fs = Self {
            offset: start,
            dataMap: dataMap::new(sb , bm , it , ex),                     
            cursor: start,
            handles: Vec::new(),
            cursor: start, 
        };
        
        
        fs.create_root()?;
        
        Ok(fs)
    }
    pub fn init(blockSize: u32 ,start: u32){ 
        let mut fs = Self {
            offset: start,
            dataMap: dataMap::new(types::SuperBlock::read(start) ,
                                    types::BitMap::read(start + size_of<types::SuperBlock>()) , types::InodeTable::read(start + size_of<types::SuperBlock>() + size_of<types::BitMap>()), todo!() )  ,
            handles: Vec::new(),
            cursor: start,
                       
        };
    }
    fn create_file(&self){
        self.dataMap.create_file();
    }
    
    pub fn read_to_file(&self){

    }
    pub fn write_to_file(&self) { 

    }
    fn find_path(&self, path: path::Path){
        let tokens = path.iter() ; 
        for i in path.iter() {

        }
    }
    fn delete_items(&self, Items: Vec<path::Path>){

    }
    fn create_root(){
        self.reach_root();
    }
    pub fn create_root(&mut self) -> List<Test>{
        
    }
    pub fn read(){

    }
    fn find_children(&self, path: path::Path ) -> Result<Vec<Path> , bool> { // F
        
    }
    fn walkInode(&self, Inode: types::Inode){
        
    }
    fn walkDir(&self, handle: &mut Handle, path: path::Path){
       
    }
    pub fn ls(&self) -> Result<Vec<&str>, str>{
        if self.Inode.dir == true {
            
        }
        for i in self.dataMap.InodeTable.inodes {
            if i.dir == true {
                println!("{}", i.name);
            }
        }
            
        }
    }

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_fs() {
        let fs = super::Fs::new(1024, 0);
        assert_eq!(fs.dataMap.sb.blocksize, 1024);
        assert_eq!(fs.dataMap.sb.offset, 0);
    }
    
}


// Starting at the first provided offset all files and directories are stored in a list which