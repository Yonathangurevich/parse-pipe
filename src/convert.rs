use std::{fs::File, io::Read};

pub fn turn_txt_to_csv(txt_path: &str, delimiter: &str) -> std::io::Result<()> {

    let mut input = File::open(txt_path)?;
    let output_file_name_str = txt_path.split(".").collect::<Vec<&str>>()[0];
    let output_file_name = format!("{}.csv", output_file_name_str);

    let mut wtr = csv::Writer::from_path(output_file_name)?;

    let mut content = String::new();
    input.read_to_string(&mut content)?;

    for line in content.lines().collect::<Vec<&str>>() {
        let line_vec = line.split(delimiter).collect::<Vec<&str>>();

        wtr.write_record(line_vec)?;
    }

    Ok(())
}
