use candid::*;
use ic_cdk_macros::update;
use std::{collections::BTreeMap, path};
struct FileHandle {
    canisters: Vec<shardRef>,
    path: path::Path,
    cursor: u64, // Current position in the file
}
impl FileHandle {
    fn new(path: path::Path) {}
    fn open(path: path::Path) {}
    fn readLine() {}
    fn close() {}
    fn write(path: path::Path) {}
    fn close() {}
}
struct dirHandle {
    canisters: Vec<shardRef>,
    path: path::Path,
    children: Vec<path::Path>,
}
impl dirHandle {
    fn new(path: path::Path, mut sysRef: &fs) { // Creates it if it does not already exist
    }
}

type shardRef<'a> = ic_utils::canister::Canister<'a>;

struct fs<'a> {
    file_handles: Vec<FileHandle>,
    dirHandles: Vec<dirHandle>,
    canisters: Vec<shardRef<'a>>,
    dirTree: BTreeMap,
}
impl fs<'_> {
    fn new() -> Self {
        Self {
            file_handles: Vec::new(),
            dirHandles: Vec::new(),
            canisters: Vec::new(),
            dirTree: BTreeMap::new(),
        }
    }
    fn init() -> Self {
        Self {
            file_handles: (),
            dirHandles: (),
            canisters: (),
            dirTree: (),
        }
    }
    fn new_shard(self) {
        self.canisters.push(self.cloneShard())
    }
}

#[update(name = "cloneShard")]
fn cloneShard(canister: ic_utils::canister::Canister) -> shardRef<'static> {
    use ic_cdk::*;
    ic_utils::canister::Canister::clone_with_()
}

#[update(name = "getFolders")]
fn getFolders(path: String) -> Vec<T> { // Gets folders for the current path
}
#[update(name = "getFIles")]
fn getFiles(path: String) -> Vec<T> { // Gets files for the current path
}
#[update(name = "getAll")]
fn getAll(path: String) -> Vec<T> {}
#[update(name = "getContents")]
fn getFileContents(path: String, offset: u128, numberOfBytes: u128) -> Vec<T> {}
#[update(name = "getFileShards")]
fn getFileShards(path: String) -> Vec<T> {}
