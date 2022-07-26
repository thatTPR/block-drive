# ICPFS File sytem, Multi canister architecture (controllable)
File Size limit Around 4tb i think

The implementation should only be created after all other stable varaibles have been made i don't think it breaks if you do things differently but better safe than sorry
The workflow involves building out your own interface for everything you plan to do with this storage, 
While it is perfectly possible to send messages through the canister list
The filesystem is Ext4-like though it is in no way directly inspired from it : It has some huge architectural differences out of neccesity and because of its pseudo-sharded nature and its dynamic nature 
 Limitations, right  now the filesystem does not support links. This would have been kind of a pain to implement right now. I'm not doing os specific stuff right now. I refuse. 



Structure 

SuperBlock
fs - {
    DataMap,
    
} 
BitMap
InodeTable
