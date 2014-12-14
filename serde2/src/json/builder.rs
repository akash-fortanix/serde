// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::TreeMap;
use std::str::StrAllocating;

use ser::{mod, Serialize};
use json::value::{mod, Value};

pub struct ArrayBuilder {
    array: Vec<Value>,
}

impl ArrayBuilder {
    pub fn new() -> ArrayBuilder {
        ArrayBuilder { array: Vec::new() }
    }

    pub fn unwrap(self) -> Value {
        Value::Array(self.array)
    }

    pub fn push<T: ser::Serialize>(mut self, v: T) -> ArrayBuilder {
        self.array.push(value::to_value(&v));
        self
    }

    pub fn push_array(mut self, f: |ArrayBuilder| -> ArrayBuilder) -> ArrayBuilder {
        let builder = ArrayBuilder::new();
        self.array.push(f(builder).unwrap());
        self
    }

    pub fn push_object(mut self, f: |ObjectBuilder| -> ObjectBuilder) -> ArrayBuilder {
        let builder = ObjectBuilder::new();
        self.array.push(f(builder).unwrap());
        self
    }
}

pub struct ObjectBuilder {
    object: TreeMap<String, Value>,
}

impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder { object: TreeMap::new() }
    }

    pub fn unwrap(self) -> Value {
        Value::Object(self.object)
    }

    pub fn insert<K: StrAllocating, V: ser::Serialize>(mut self, k: K, v: V) -> ObjectBuilder {
        self.object.insert(k.into_string(), value::to_value(&v));
        self
    }

    pub fn insert_array<S: StrAllocating>(mut self, key: S, f: |ArrayBuilder| -> ArrayBuilder) -> ObjectBuilder {
        let builder = ArrayBuilder::new();
        self.object.insert(key.into_string(), f(builder).unwrap());
        self
    }

    pub fn insert_object<S: StrAllocating>(mut self, key: S, f: |ObjectBuilder| -> ObjectBuilder) -> ObjectBuilder {
        let builder = ObjectBuilder::new();
        self.object.insert(key.into_string(), f(builder).unwrap());
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::TreeMap;

    use json::value::Value;
    use super::{ArrayBuilder, ObjectBuilder};

    #[test]
    fn test_array_builder() {
        let value = ArrayBuilder::new().unwrap();
        assert_eq!(value, Value::Array(Vec::new()));

        let value = ArrayBuilder::new()
            .push(1i)
            .push(2i)
            .push(3i)
            .unwrap();
        assert_eq!(value, Value::Array(vec!(Value::I64(1), Value::I64(2), Value::I64(3))));

        let value = ArrayBuilder::new()
            .push_array(|bld| bld.push(1i).push(2i).push(3i))
            .unwrap();
        assert_eq!(value, Value::Array(vec!(Value::Array(vec!(Value::I64(1), Value::I64(2), Value::I64(3))))));

        let value = ArrayBuilder::new()
            .push_object(|bld|
                bld
                    .insert("a".to_string(), 1i)
                    .insert("b".to_string(), 2i))
            .unwrap();

        let mut map = TreeMap::new();
        map.insert("a".to_string(), Value::I64(1));
        map.insert("b".to_string(), Value::I64(2));
        assert_eq!(value, Value::Array(vec!(Value::Object(map))));
    }

    #[test]
    fn test_object_builder() {
        let value = ObjectBuilder::new().unwrap();
        assert_eq!(value, Value::Object(TreeMap::new()));

        let value = ObjectBuilder::new()
            .insert("a".to_string(), 1i)
            .insert("b".to_string(), 2i)
            .unwrap();

        let mut map = TreeMap::new();
        map.insert("a".to_string(), Value::I64(1));
        map.insert("b".to_string(), Value::I64(2));
        assert_eq!(value, Value::Object(map));
    }
}