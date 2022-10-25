use std::{collections::HashMap, env, sync::Mutex};

pub struct Translator<'a> {
    en_us_resource: Option<HashMap<&'a str, &'a str>>,
    zh_cn_resource: Option<HashMap<&'a str, &'a str>>,
}

impl Translator<'_> {
    pub fn new() -> Self {
        Translator {
            en_us_resource: None,
            zh_cn_resource: None,
        }
    }
}

lazy_static::lazy_static! {
  static ref RESOURCE: Mutex<Translator<'static>> = Mutex::new(Translator::new());
}

pub fn load_resources() {
    load_en_us_resource();
    load_zh_cn_resource();
}

pub fn translate(key: &str) -> String {
    let language = env::var("LANGUAGE").unwrap();
    let translator = RESOURCE.lock().unwrap();
    let mut value = translator.en_us_resource.as_ref().unwrap();
    if language == "en_US" {
        value = translator.en_us_resource.as_ref().unwrap();
    } else if language == "zh_CN" {
        value = translator.zh_cn_resource.as_ref().unwrap();
    }

    let result = match value.get(key) {
        None => key,
        Some(text) => text,
    };

    String::from(result)
}

fn load_en_us_resource() {
    let en_us_resource_map: HashMap<&str, &str> = [
        "menu_title",
        "menu_body_analysis",
        "menu_language_setting",
        "daily_energy_expenditure_result_text",
        "daily_energy_expenditure_unit",
        "suggested_carbon_cycle_energy_intake_result_text",
        "body_metabolism_height_question",
        "body_metabolism_height_question_error_message",
        "body_metabolism_weight_question",
        "body_metabolism_weight_question_error_message",
        "body_metabolism_age_question",
        "body_metabolism_age_question_error_message",
        "body_metabolism_sport_strength_question",
        "body_metabolism_sport_strength_1",
        "body_metabolism_sport_strength_2",
        "body_metabolism_sport_strength_3",
        "body_metabolism_sport_strength_4",
        "body_metabolism_sport_strength_5",
        "body_metabolism_sport_strength_6",
        "energy_intake_body_fat_percentage_select_question",
        "energy_intake_body_fat_percentage_1",
        "energy_intake_body_fat_percentage_2",
        "energy_intake_body_fat_percentage_3",
        "energy_intake_body_fat_percentage_4",
        "energy_intake_body_fat_percentage_5",
        "energy_intake_body_fat_percentage_6",
        "energy_intake_body_fat_percentage_7",
        "energy_intake_body_fat_percentage_customize_question",
        "energy_intake_body_fat_percentage_customize_question_error_message",
        "carbohydrate",
        "protein",
        "fat",
        "low_carbon_day",
        "mid_carbon_day",
        "high_carbon_day",
        "energy_intake_unit",
    ]
    .into_iter()
    .zip(
        [
            "Choose what your want",
            "Body Analysis",
            "Language setting",
            "Your daily energy expenditure is roughly: ",
            "kcal",
            "Suggested carbon cycle energy intake: ",
            "Input your height(cm)",
            "Please enter a valid height",
            "Input your weight(kg)",
            "Please enter a valid weight",
            "Input your age(year)",
            "Please enter a valid age",
            "Select your daily sport strength",
            "Sedentary",
            "Very light",
            "Light",
            "Moderate",
            "High",
            "Extreme",
            "Select your current body fat percentage",
            "Clearly see your abdominals(10%)",
            "Roughly see your abdominals with waist fat(15%)",
            "Could not see your abdominals but could touch it(20%)",
            "Lots of waist fat(25%)",
            "Lots of chest fat(30%)",
            "Lots of body fat(35%)",
            "Customize",
            "Input your current body fat percentage(%)",
            "Please enter a valid body fat percentage",
            "carbohydrate",
            "protein",
            "fat",
            "low carbon day",
            "mid carbon day",
            "high carbon day",
            "g",
        ]
        .into_iter(),
    )
    .collect();
    RESOURCE.lock().unwrap().en_us_resource = Some(en_us_resource_map);
}

fn load_zh_cn_resource() {
    let zh_cn_resource_map: HashMap<&str, &str> = [
        "menu_title",
        "menu_body_analysis",
        "menu_language_setting",
        "daily_energy_expenditure_result_text",
        "daily_energy_expenditure_unit",
        "suggested_carbon_cycle_energy_intake_result_text",
        "body_metabolism_height_question",
        "body_metabolism_height_question_error_message",
        "body_metabolism_weight_question",
        "body_metabolism_weight_question_error_message",
        "body_metabolism_age_question",
        "body_metabolism_age_question_error_message",
        "body_metabolism_sport_strength_question",
        "body_metabolism_sport_strength_1",
        "body_metabolism_sport_strength_2",
        "body_metabolism_sport_strength_3",
        "body_metabolism_sport_strength_4",
        "body_metabolism_sport_strength_5",
        "body_metabolism_sport_strength_6",
        "energy_intake_body_fat_percentage_select_question",
        "energy_intake_body_fat_percentage_1",
        "energy_intake_body_fat_percentage_2",
        "energy_intake_body_fat_percentage_3",
        "energy_intake_body_fat_percentage_4",
        "energy_intake_body_fat_percentage_5",
        "energy_intake_body_fat_percentage_6",
        "energy_intake_body_fat_percentage_7",
        "energy_intake_body_fat_percentage_customize_question",
        "energy_intake_body_fat_percentage_customize_question_error_message",
        "carbohydrate",
        "protein",
        "fat",
        "low_carbon_day",
        "mid_carbon_day",
        "high_carbon_day",
        "energy_intake_unit",
    ]
    .into_iter()
    .zip(
        [
            "选择你需要的功能",
            "体测分析",
            "语言设置",
            "你的每日能量消耗大约是: ",
            "千卡",
            "推荐碳循环期间的热量摄入：",
            "输入你的身高(cm)",
            "请输入一个正确的身高",
            "输入你的体重(kg)",
            "请输入一个正确的体重",
            "输入你的年龄(year)",
            "请输入一个正确的年龄",
            "选择你每日的运动强度",
            "久坐不动",
            "几乎不动",
            "少量运动",
            "适量运动",
            "高强度运动",
            "超高强度运动",
            "选择你的体脂率",
            "腹肌清晰可见(10%)",
            "腹肌隐约可见，伴随一些腰部赘肉(15%)",
            "看不见腹肌，但是能摸到(20%)",
            "大量腰部赘肉(25%)",
            "大量胸部赘肉(30%)",
            "大量身体赘肉(35%)",
            "自定义输入",
            "输入你的体脂率(%)",
            "请输入一个正确的体脂率",
            "碳水化合物",
            "蛋白质",
            "脂肪",
            "低碳日",
            "中碳日",
            "高碳日",
            "克",
        ]
        .into_iter(),
    )
    .collect();
    RESOURCE.lock().unwrap().zh_cn_resource = Some(zh_cn_resource_map);
}
