use std::{
    fs,
    io::{self, stdout, Write},
};

struct Data<'a> {
    filename: String,
    location_target: String,
    byte_data: &'a Vec<u8>,
}

impl<'a> Data<'a> {
    fn copy_data(&self) {
        let copy_dir = format!("{}/{}", &self.location_target, &self.filename);
        fs::write(copy_dir, &self.byte_data).expect("Couldn't write data");
        println!("{} copied to {}", &self.filename, &self.location_target);
    }

    fn move_data(&self) {
        let move_dir = format!("{}/{}", &self.location_target, &self.filename);
        match fs::write(move_dir, &self.byte_data) {
            Ok(_) => {
                println!(
                    "{} has been moved to {}",
                    &self.filename, &self.location_target
                );

                match fs::remove_file(&self.filename) {
                    Ok(_) => {
                        println!("File {} removed from the original director", &self.filename);
                    }
                    Err(de) => {
                        println!("Error for delete/remove file {}: {}", &self.filename, de);
                    }
                }
            }
            Err(e) => {
                println!("Error for writing data: {}", e);
            }
        }
    }

    fn delete_data(&self) {
        match fs::remove_file(&self.filename) {
            Ok(_) => {
                println!("{} has been deleted", &self.filename);
            }
            Err(e) => {
                println!("Error for delete/remove file {}: {}", &self.filename, e);
            }
        }
    }
}

fn read_file_as_byte(filename: &String) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
    let mut _file = fs::read(filename)?;
    Ok(_file)
}

/// Will ask for command like copy, move, delete
#[allow(dead_code)]
fn show_file_command() -> String {
    print!("Command: ");
    stdout().flush().unwrap();
    let mut command = String::new();

    match io::stdin().read_line(&mut command) {
        Ok(_) => {
            let command = command.trim().to_string();
            return command;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    "".to_string()
}

fn main() {
    let filename = String::from("Cargo.toml");
    let byte_data = read_file_as_byte(&filename).unwrap();
    let data = Data {
        filename,
        location_target: "testing_area/".to_string(),
        byte_data: &byte_data,
    };

    data.copy_data();

    let data2 = Data {
        filename: "testing_area/Cargo.toml".to_string(),
        location_target: ".".to_string(),
        byte_data: &byte_data,
    };

    data2.move_data();

    data2.delete_data();
}
