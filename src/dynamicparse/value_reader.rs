use super::*;

pub struct StringValueReader {
    value: String
}

impl StringValueReader {
    pub fn next(obj: &mut Box<Any>, value: &str) -> ValueReaderStatus {
        obj.downcast_mut::<StringValueReader>().unwrap().value = value.to_string();
        ValueReaderStatus::Ready
    }

    pub fn result(obj: Box<Any>) -> Box<Any> {
        Box::new(obj.downcast_ref::<StringValueReader>().unwrap().value.to_string())
    }

    pub fn create() -> Box<Any> {
        let reader = StringValueReader {
            value: String::new()
        };

        Box::new(reader)
    }
}

