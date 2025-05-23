use simple_config::Config;
use simple_config::ConfigType;
use std::fs;

#[derive(Config)]
struct TestConfig {
    pub float_value: f64,
    pub int_value: i32,
    pub string_value: String,
    pub custom_enum: TestConfigEnum,
    pub custom_struct: TestCustomConfigStruct
}

impl TestConfig {
    pub fn new() -> Self {
        TestConfig {
            float_value: 42.0,
            int_value: 69,
            string_value: "420".into(),
            custom_enum: TestConfigEnum::Value,
            custom_struct: TestCustomConfigStruct { one_value: 0, other_value: 0.0 }
        }
    }
}

#[derive(ConfigType, Debug, PartialEq)]
enum TestConfigEnum {
    Value,
    OtherValue,
    ThirdValue,
}

struct TestCustomConfigStruct {
    pub one_value: i32,
    pub other_value: f32,
}

impl ConfigType for TestCustomConfigStruct {
    fn parse_config<'a>(mut values: impl Iterator<Item = &'a str>) -> Result<Self, String> where Self: Sized {
        let one_value = values.next().unwrap().parse().unwrap();
        let other_value = one_value as f32 * 3.0;
        Ok(Self {
            one_value,
            other_value
        })
    }
    fn get_params() -> String {
        "one value or another value".into()
    }
}

#[test]
fn test_parse() {
    let mut config = TestConfig::new();
    fs::write("test.conf", "
        custom_struct = 420\n \
        string_value = 69\n \
        int_value = 420\n \
        custom_enum = ThirdValue\n \
        custom_struct  = 42\n \
    ").expect("could not write file");
    let result = config.parse_file("test.conf").expect("could not parse");
    assert!(result);
    assert_eq!(config.int_value, 420);
    assert_eq!(config.float_value, 42.0);
    assert_eq!(config.string_value, "69");
    assert_eq!(config.custom_enum, TestConfigEnum::ThirdValue);
    assert_eq!(config.custom_struct.one_value, 42);
    assert_eq!(config.custom_struct.other_value, 42.0 as f32 * 3.0);
}


#[test]
fn test_parse_file() {
    let mut config = TestConfig::new();
    let source = vec![
        "custom_struct", "420",
        "string_value", "69",
        "int_value", "420",
        "custom_enum", "ThirdValue",
        "custom_struct", "42",
    ].into_iter();
    if let Err(_) = config.add_source(source) {
        panic!("could not parse");
    }
    assert_eq!(config.int_value, 420);
    assert_eq!(config.float_value, 42.0);
    assert_eq!(config.string_value, "69");
    assert_eq!(config.custom_enum, TestConfigEnum::ThirdValue);
    assert_eq!(config.custom_struct.one_value, 42);
    assert_eq!(config.custom_struct.other_value, 42.0 as f32 * 3.0);
}


#[test]
fn test_help() {
    assert_eq!(TestConfig::get_help(), [ 
        "List of arguments with their respective parameters:",
        "float_value <f64>",
        "int_value <i32>",
        "string_value <String>",
        "custom_enum <Value|OtherValue|ThirdValue>",
        "custom_struct <one value or another value>"
    ].join("\n"));
}
