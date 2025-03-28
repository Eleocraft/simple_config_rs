pub use config_macro::Config;
pub use config_macro::ConfigType;

pub trait Config {
    fn add_source<'a>(&mut self, values: impl Iterator<Item = &'a str>) -> Result<(), String>;
    fn get_help() -> String;
}

pub trait ConfigType {
    fn parse_config<'a>(values: impl Iterator<Item = &'a str>) -> Result<Self, String> where Self: Sized;
    fn get_params() -> String;
}
