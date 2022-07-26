mod fileSys;
mod types;
use std::mem::size_of ;

mod path; 


use ic_cdk::api::stable::*;
use ic_cdk_macros::*;
use ic_utils::canister::Canister;
use serde::{Serialize, Deserialize};

 
fn spawn(){

}



// any stable operation that is not done through the fs api in the same canister breaks it do not do it

#[derive(Serialize, Deserialize, Debug, Default)]
struct ICPFS<'a> { // In the wild you might want to deploy this within your own structs that serve http endpoints so that you can serve the canisters to an Authenticated actor
    dangledFrom: u32,
    danglingTo: u32,
    single: bool , // True for single canister, false for multiple canisters
    references: Vec<ic_utils::Canister<'a>>, // References of all canisters in the filesystem
    size: u32, // How many canisters make up the filesystem
    current: u32, // Current canister in use index
     
    wd: path::Path, // Non stable Used cause I wanted to have cwd operations and because I thought there could be worse things than possibly giving the user a terminal
    fs: fileSys::Fs // Stable but handled internally
    
}
impl ICPFS<'a> {
    // danglingFrom index to reference, single: true if filesystem runs in single canister mode,  false if multiple canisters can be created or already exist
    fn new(danglingFrom: u32, danglingTo:u32 ,single:bool, references:Vec<reference> , BlockSize: u32, prevNo: u8   ) { 
        
        
            let current = Self {
                dangledFrom: danglingFrom,
                danglingTo: danglingTo,
                
                single: mode ,
                size: references.len() as u32,

                current: references.len() , 
                references: references.push(ic_utils::canister::Canister<'a>::canister_id_(&self)),

                wd: path::Path::new("/"),
                fs: fileSys::Fs::new( 1024  , 0)               
            };
        
           
            
            return current ;

    }
   
    fn init() { // This used for initializes the canister from the filesystem
         
    }
    fn newStore() { 
        let service =  candid::types::reference::Service;
    }
    fn stable(self) { // Creates a new directory in the filesystem root /sys and creates a json file inside /sys/current.json // Storing inside it the members of the canister( encrypted )
        // Contains itself, fs, which contains dataMap, inodeMap, and so on all is loaded from the filessystem sorted in json form , hidden from the user , The name of the file is :
        
        let id = self.references[self.current];
        newFile("/sys"+ s +"/current.json", &self.fs);
        
    }

    fn sendCurrent(){

    }
    fn updateReferences(){
        for i in self.references {
            sendCurrent(i.canister_id_(), current.references[current.references.len()-1].canister_id_());
        }
    }
    fn passNext(n: T)
    {

    }
    fn cd(path: path::Path) {
        if path.is_absolute() {
            self.wd = path;
        } else {
            self.wd = self.wd.join(path);
        }
    }
    fn wd(self) -> path::path {return self.wd ;}
    fn openFile(file: path::path)  {

    }

    fn check_available(&self){// Checks if the filesys has enough space to store new contents // 100 kby is the limit
        self.fs.dataMap.
    }
    fn newFile(dir: fileSys::dir) {

    }
    
    
        
  
    fn mkdir(String: String)   { // TODO

        
    }
    fn writeFile(self, path: path::Path) {

 }
    fn findCurrent(name: String) {

    }
    fn write(){

    }
    fn pass_write(){

    }
    fn read(path: path::path ){
        let result: Result<T,E>;
        if result.contains_err("Dangled file") {
            pass_read(path::Path );
        }
    }
    fn pass_read(){ // Called when file is dangled points to oher shard
        
    }
    fn find(name: String){// Finds all files and directories that match the search sequence 
       
    }
    fn pass_find(name:String){

    }
    fn ls(dir: path::Path) { // can be set to none and returns the current directory
        if dir == None {
            
        }
        if dir.is_dir() {
            for entry in self.dir.. {
                println!("{:?}", entry);
            }
        }
        
        
    }
    fn tree(dir: path::Path) {
       
    }
}

use ic_cdk_macros::* ;

#[update( name = "clone")]
fn clone(){

}

#[update (name = "update_reference")]
fn update_reference(refs: Vec<ic_utils::Canister>, from : ic_utils::Canister ){ // Updates the references of all shards 

}
struct Driver {

    // TODO protocol handling, candid interfacing, with authentication,  allow various operations, upload, modify, create new, delete , read, view , all with permissions in mind, Files do not have individual permissions because
    // that would I think be redundant to some extent , Instead the filesystem itself has permissions associated. It works kinda like Unix Namespaces in that a namespace can only be accessed by authorized principals.
}
mod tests {
    
}
