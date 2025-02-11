use std::fs::{File, read_dir};
use std::io::{self, Read, Write};
use std::path::Path;
use serde::Serialize; 
use serde::de::DeserializeOwned; 
use bincode::serialize;

/*
    This is functions for handling files
*/

pub fn check_file(file_path: &str) -> bool {
    /*
    Checks weather there is a file at the path given 
    */
    match Path::new(file_path).exists() {
        true => true,
        false => false,
    }
}

pub fn write_binary_file<T: Serialize>(file_path: &str, data: &T) -> bool {
    // Serialize the Player struct into a binary format
    match serialize(data) {
        Ok(data) => {
            // Write the serialized data to the binary file
            match File::create(file_path) {
                Ok(mut file) => match file.write_all(&data) {  // Pass the byte slice correctly
                    Ok(_) => true, // Successfully wrote to file
                    Err(e) => {
                        eprintln!("Failed to write to file: {}", e);
                        false
                    }, // Failed to write to file but did open
                },
                Err(e) => {
                    eprintln!("Failed to open file: {}", e);
                    false
                }, // Failed to open file
            }
        }
        Err(_) => false, // Serialization failed
    }
}

pub fn read_binary_file<T: DeserializeOwned>(file_path: &str) -> Result<T, io::Error> {
    /*
    This function reads a binary file and deserializes its content into a struct of type T.
    If an error occurs during reading or deserialization, an appropriate io::Error is returned.
    */

    let mut file = File::open(file_path)?; // This has a standard error return in case of failure

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    match bincode::deserialize(&data) {
        Ok(obj) => Ok(obj), // Successfully deserialized
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to deserialize data")),
    }
}

pub fn read_json_file<T: DeserializeOwned>(file_path: &str) -> io::Result<Vec<T>> {
    // Use the `?` operator to unwrap the `File` inside the `Result`
    let mut file = File::open(file_path)?;  // This extracts the `File` from the Result
    let mut contents = String::new();
    
    // Now you can safely call `read_to_string` on `file`, which is a `File` type
    file.read_to_string(&mut contents)?;

    // Deserialize the contents of the file to get the consumable list
    let consumable_list: Result<Vec<T>, serde_json::Error> = serde_json::from_str(&contents);
    
    match consumable_list {
        Ok(list) => Ok(list),
        Err(e) => {
            // Log or handle the error here instead of panicking
            eprintln!("Error deserializing consumable list: {}", e);
            Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse consumable list"))
        }
    }
}