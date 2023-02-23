use std::collections::HashMap;


static mut MAP: Option<HashMap<Box<String>, Box<String>>> = None;

fn init() {
    unsafe{
        MAP = Some(HashMap::new());
    }
}

pub fn set(key: String, value: String) {
    unsafe {
        if MAP == None {
            init();
        }

        MAP.as_mut().unwrap().insert(
            Box::new(key),
            Box::new(value),
        );
    }
}

pub fn get(key: String) -> Option<String> {
    unsafe {
        if MAP == None {
            init();
        }
        let value = MAP.as_ref().unwrap().get(&key);
        if value.is_some() {
            Some(value.unwrap().to_string())
        }else {
            None
        }
    }
}