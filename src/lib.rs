pub use config_macro::Config;

pub trait Config {
    fn add_source<'a>(&mut self, values: impl Iterator<Item = &'a str>) -> Result<(), String>;
}

pub trait ConfigEnum {
    fn parse_config<'a>(values: impl Iterator<Item = &'a str>) -> Result<Self, String> where Self: Sized;
}
