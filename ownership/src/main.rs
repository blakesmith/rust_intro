use std::io;
use std::io::Write;
use std::fs::{File, OpenOptions};

fn write_data(mut file: File) -> Result<usize, io::Error> {
    let buf = [0, 1, 2, 3, 4];
    file.write(&buf)
}

fn heap_memory() {
    let value = Box::new("Meep meep".to_string());
    println!("Roadrunner says: {}", value);
}

fn main() {
    let file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open("/tmp/my_file").expect("Error: Could not open file");
    let result = write_data(file);
    println!("Write result was: {:?}", result);
    heap_memory();
}
