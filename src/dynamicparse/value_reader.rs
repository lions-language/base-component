use quote::quote;

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

macro_rules! numerical_value_reader {
    ($t:ty) => {{
        quote! {
            let struct_name = format!("{}ValueReader", stringify!($t));
            pub struct #struct_name {
                value: $t
            }
        }
    }}
}

numerical_value_reader!{u32}

/////////////////////////////////
pub struct U32ValueReader {
    value: u32
}

impl U32ValueReader {
    pub fn next(obj: &mut Box<Any>, value: u32) -> ValueReaderStatus {
        obj.downcast_mut::<U32ValueReader>().unwrap().value = value;
        ValueReaderStatus::Ready
    }

    pub fn result(obj: Box<Any>) -> Box<Any> {
        Box::new(obj.downcast_ref::<U32ValueReader>().unwrap().value)
    }
}

impl U32ValueReader {
    pub fn create() -> Box<Any> {
        let reader = U32ValueReader {
            value: 0 as u32
        };

        Box::new(reader)
    }
}

impl U32Value {
    pub fn take_clone(&self) -> u32 {
        let value = self.value.borrow();
        **value.downcast_ref().as_ref().unwrap()
    }
}
