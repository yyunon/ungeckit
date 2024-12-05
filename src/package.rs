use std::io;
use std::fs::File;
use tokio::process::Command;
use log::info;

pub struct PackageManager {
    path_to_executable: Option<String>,
}

impl PackageManager {
    pub fn new() -> Self{
        let mut path = None;
        
        Self{
            path_to_executable: path, 
        }
    }
}
