use requestty::{Answer, Result, ListItem};

pub fn transform_answer_to_float(answer: Result<Answer>) -> f32 {
    answer.unwrap().try_into_float().unwrap() as f32
}

pub fn transform_answer_to_int(answer: Result<Answer>) -> u8 {
    answer.unwrap().try_into_int().unwrap() as u8
}

pub fn transform_answer_to_list_item(answer: Result<Answer>) -> ListItem {
    answer.unwrap().try_into_list_item().unwrap()
}
