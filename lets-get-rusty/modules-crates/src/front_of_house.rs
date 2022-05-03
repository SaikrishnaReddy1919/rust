pub mod hosting; // loads hosting.rs file from folder with the same name as current file name(2..check hosting.rs for 3)

pub fn check() {}

//------Modules Summary-----
//Rust lets you split a package into multiple crates and a crate into modules so you can 
//refer to items defined in one module from another module. 
//You can do this by specifying absolute or relative paths. 
//These paths can be brought into scope with a use statement so you can use a shorter path 
//for multiple uses of the item in that scope. Module code is private by default, 
//but you can make definitions public by adding the pub keyword.