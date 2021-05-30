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

        for sub_str in querystring.split("&") {
            let mut key = sub_str;
            let mut value = "";

            if let Some(ind) = sub_str.find("=") {
                key = &sub_str[..ind];
                value = &sub_str[ind + 1..];

                data.entry(key)
                    .and_modify(|existingValue| match existingValue {
                        Value::Single(prev_val) => {
                            *existingValue = Value::Multiple(vec![prev_val, value]);
                        }
                        Value::Multiple(prev) => {
                            prev.push(value);
                        }
                    })
                    .or_insert(Value::Single(value));
            }
        }

        Self { data }
    }
}
