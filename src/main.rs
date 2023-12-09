#![allow(dead_code)]


use std::{fs, env};
use std::io::ErrorKind;


enum DataType {
    Title([u8; 30]),
    Artist([u8; 30]),
    Album([u8; 30]),
    Year([u8; 4]),
    Genre(u8),
    Comment([u8; 28]),
}


// Main function returns a Result<(), ()> to allow clean shutdown
fn main() -> Result<(), ()> {
    let args: Vec<_> = env::args().collect();
    
    // Minimum 4 args: artest <filename> <operation> <data>
    if args.len() == 4 {
        let filename: &str = &args[1];
        let file = fs::read_to_string(filename);
        match file {
            Ok(f) => {
                let file = f.as_bytes();
                let mut v1data: &[u8] = &file[file.len() - 128..];
                
                // Determine the requested operation
                let op = &args[2][..];
                let value = &args[3];
                match op {
                    "ti" => {
                        modify_data(DataType::Title(fix_length(value.as_bytes())), v1data);
                    }
                    "ar" => {
                        
                    }
                    "al" => {
                        
                    }
                    "ye" => {
                        
                    }
                    "co" => {
                        
                    }
                    _ => {
                        println!("Fatal Error: Unknown operation {}. Use -h for help.", op);
                        return Ok(());
                    }
                }
                
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        println!("Fatal error: Could not find file {}.", filename);
                        return Ok(());
                    }
                    ErrorKind::PermissionDenied => {
                        println!("Fatal error: Permission denied to read file {}.", filename);
                        return Ok(());
                    }
                    _ => {
                        println!("Error reading file: {}", e);
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    } else {
        println!("Fatal error: Not enough arguments supplied for full operation. Run with -h for help.");
        Ok(())
    }
    
}

fn modify_data(data_type: DataType, data: &[u8]) {
    
}

fn fix_length(data: &[u8]) -> [u8; 30] {
    
    
    [0; 30]
}


