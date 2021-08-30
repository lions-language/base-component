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

impl StringValue {
    pub fn take_clone(&self) -> String {
        let value = self.value.borrow();
        value.downcast_ref::<String>().as_ref().unwrap().to_string()
    }
}

/////////////////////////////////
pub struct NumericalValueReader {
    value: u32
}

impl NumericalValueReader {
    pub fn next(obj: &mut Box<Any>, value: u32) -> ValueReaderStatus {
        obj.downcast_mut::<NumericalValueReader>().unwrap().value = value;
        ValueReaderStatus::Ready
    }

    pub fn result(obj: Box<Any>) -> Box<Any> {
        Box::new(obj.downcast_ref::<NumericalValueReader>().unwrap().value)
    }

    pub fn create() -> Box<Any> {
        let reader = NumericalValueReader {
            value: u32::default(),
        };

        Box::new(reader)
    }
}

impl NumericalValue {
    pub fn take_clone(&self) -> u32 {
        let value = self.value.borrow();
        **value.downcast_ref::<u32>().as_ref().unwrap()
    }
}
