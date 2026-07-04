use std::fs;
use clap::Parser;
use crate::model::Extensions;
use crate::model::Extensions::NotExists;

// the pp would be the way of bring it in, the next -- would be for 'to what' you want the file to be, and the next is the file name \ file-path so you wont have to stand in the ".";
// pp --csv --file-name.txt
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    to_file: String,
    file_name: String,
    delimiter: String
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {

    let e = is_file_exists(&args.file_name)?;

    if !e || !is_extension_we_work(&args.to_file) {
        return Err("Not working with with this type".into());
    }

    let before_extension = get_files_extension(&args.file_name); // the existing file on ".";

    let after_extension = Extensions::from(args.to_file.as_str()); // the file we want to turn the existing to.

    before_extension.parse_to(after_extension, &args.file_name, &args.delimiter)?;


    Ok(())
}

fn is_extension_we_work(to_file: &str) -> bool {
    if let NotExists = Extensions::from(to_file) {
        return false;
    }
    true
}

fn get_files_extension(file_name: &str) -> Extensions {
    let file_name_spilt = file_name.split(".").collect::<Vec<&str>>();
    Extensions::from(file_name_spilt[1])
}

fn is_file_exists(file_name: &str) -> std::io::Result<bool> {

    for enteries in fs::read_dir(".")? {
        let entry = enteries?;
        let founded_file = entry.file_name().to_string_lossy().to_string();
        if file_name == founded_file {
            return Ok(true);
        }
    }

    Ok(false)
}
