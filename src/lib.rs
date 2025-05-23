pub use config_macro::Config;
pub use config_macro::ConfigType;
pub use std::fs;

#[derive(Debug)]
pub struct ParseErr;

pub trait Config {
    fn add_source<'a>(&mut self, values: impl Iterator<Item = &'a str>) -> Result<(), String>;
    fn get_help() -> String;
    fn parse_file(&mut self, path: &str) -> Result<bool, ParseErr> {
        if let Ok(content) = fs::read_to_string(path) {
            self.add_source(
                content
                    .lines()
                    .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
                    .flat_map(|line| line.split('=').map(|part| part.trim())),
            ).map_err(|_| ParseErr)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn parse_cli(&mut self) -> Result<bool, ParseErr> {
        let args: Vec<String> = std::env::args()
            .skip(1)
            .map(|string| string.replace("--", ""))
            .collect();
        if !args.is_empty() && args[0] == "help" {
            println!("help:");
            println!("{}", Self::get_help());
            return Ok(true);
        }
        self.add_source(args.iter().map(|string| string.as_str()))
            .map_err(|_| ParseErr)?;
        Ok(false)
    }
}

pub trait ConfigType {
    fn parse_config<'a>(values: impl Iterator<Item = &'a str>) -> Result<Self, String> where Self: Sized;
    fn get_params() -> String;
}
