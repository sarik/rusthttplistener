use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

#[derive(Debug)]
pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&Value<'a>> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(querystring: &'a str) -> Self {
        let mut data = HashMap::new();

        for substr in querystring.split("&") {
            let mut key = substr;
            let mut value = "";

            if let Some(index) = key.find("=") {
                key = &substr[..index];
                value = &substr[index + 1..];
            }
            data.entry(key)
                .and_modify(|val| match val {
                    Value::Single(prev_val) => {
                        *val = Value::Multiple(vec![prev_val, value]);
                    }
                    Value::Multiple(prev_val) => prev_val.push(value),
                })
                .or_insert(Value::Single(value));
        }

        Self { data }
    }
}
