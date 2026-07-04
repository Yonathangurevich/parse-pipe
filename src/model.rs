use crate::convert::turn_txt_to_csv;

#[derive(Debug)]
pub enum Extensions {
    Txt,
    Csv,
    NotExists
}

impl From<&str> for Extensions {
    fn from(value: &str) -> Self {

        match value {
            "txt" => Extensions::Txt,
            "csv" => Extensions::Csv,
            _ => Extensions::NotExists
        }

    }
}

impl Extensions {
    pub fn parse_to(&self, to: Self, file_path: &str, delimiter: &str) -> std::io::Result<()> {
        match self {
            Self::Txt => {
                match to {
                    Self::Csv => {
                        turn_txt_to_csv(file_path, delimiter)?;
                    },
                    Self::Txt => {
                        println!("cant turn txt to txt");
                    },
                    Self::NotExists => {}
                }
            },
            Self::Csv => {},
            Self::NotExists => {},
        }
        Ok(())
    }
}
