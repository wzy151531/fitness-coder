use std::{env, fs::File, sync::Mutex};

pub struct Translator {
    en_us_resource: Option<serde_json::Value>,
    zh_cn_resource: Option<serde_json::Value>,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            en_us_resource: None,
            zh_cn_resource: None,
        }
    }
}

lazy_static::lazy_static! {
  static ref RESOURCE: Mutex<Translator> = Mutex::new(Translator::new());
}

pub fn load_resources() {
    load_en_us_resource();
    load_zh_cn_resource();
}

pub fn translate(key: &str) -> String {
    let language = env::var("LANGUAGE").unwrap();
    let translator = RESOURCE.lock().unwrap();
    let mut value: &serde_json::Value = translator.en_us_resource.as_ref().unwrap();
    if language == "en_US" {
        value = translator.en_us_resource.as_ref().unwrap();
    } else if language == "zh_CN" {
        value = translator.zh_cn_resource.as_ref().unwrap();
    }

    String::from(value[key].as_str().unwrap())
}

fn load_en_us_resource() {
    RESOURCE.lock().unwrap().en_us_resource =
        Some(serde_json::from_reader(File::open("./src/i18n/en_US.json").unwrap()).unwrap());
}

fn load_zh_cn_resource() {
    RESOURCE.lock().unwrap().zh_cn_resource =
        Some(serde_json::from_reader(File::open("./src/i18n/zh_CN.json").unwrap()).unwrap());
}
