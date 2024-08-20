use serde::ser::{SerializeMap, SerializeSeq};
use serde::{de, Deserialize, Serialize};
use serde_json::Number;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::convert::From;
use std::ops::{Deref, Index, IndexMut};

fn type_of<T>(_: &T) -> Option<&str> {
    Some(std::any::type_name::<T>())
}

type BoxedValue = Box<DictType>;
type BoxedVec = Vec<BoxedValue>;
type BoxedStringMap = HashMap<String, BoxedValue>;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum DictType {
    Number(i32),
    String(String),
    Bool(bool),
    Vec(BoxedVec),
    Map(BoxedStringMap),
}

impl DictType {
    /// A method to insert in vec in maps in the context recursively
    fn insert<T>(dict: &mut HashMap<String, BoxedValue>, key: &str, new_value: T)
    where
        T: Into<DictType>,
    {
        match dict.get_mut(key) {
            Some(val) => {
                // If the value exists, we recursively insert considering type
                match val.borrow_mut() {
                    DictType::Bool(_) | DictType::Number(_) | DictType::String(_) => {
                        *val = Box::new(new_value.into())
                    }
                    DictType::Vec(t) => t.push(Box::new(new_value.into())),
                    DictType::Map(t) => {
                        if let DictType::Map(newv) = new_value.into() {
                            for key in newv.keys() {
                                let inner_h = *(*(newv.get(key).unwrap())).clone(); // TODO: This doesn't sound like a very good idea
                                DictType::insert::<DictType>(t, &key, inner_h);
                            }
                        } else {
                            panic!("The value you are appending alreadys holds map type, you cannot append another type");
                        }
                    }
                }
            }
            // If the value, does not exist just insert into it.
            _ => {
                dict.insert(key.to_owned(), Box::new(new_value.into()));
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Map(v) => v.len(),
            Self::Vec(v) => v.len(),
            _ => panic!("We don't implement this on primitive types"),
        }
    }

    fn extend(&mut self, other: DictType) {
        match self {
            Self::Map(v) => {
                let unwrapped_other = other.take_map();
                for key in unwrapped_other.keys() {
                    let inner_h = *(*(unwrapped_other.get(key).unwrap())).clone(); // TODO: This doesn't sound like a very good idea
                    DictType::insert::<DictType>(v, &key, inner_h); // TODO Clone a good idea?
                }
            }
            Self::Vec(v) => v.extend(other.take_vec().into_iter()),
            _ => panic!("We don't implement this on primitive types"),
        }
    }

    fn take_vec(self) -> BoxedVec {
        match self {
            Self::Vec(v) => v,
            _ => panic!("We don't implement this on primitive types"),
        }
    }

    fn take_map(self) -> BoxedStringMap {
        match self {
            Self::Map(v) => v,
            _ => panic!("We don't implement this on primitive types"),
        }
    }

    fn take_vec_mut(&mut self) -> &mut BoxedVec {
        match self {
            Self::Vec(v) => v,
            _ => panic!("We don't implement this on primitive types"),
        }
    }

    fn take_map_mut(&mut self) -> &mut BoxedStringMap {
        match self {
            Self::Map(v) => v,
            _ => panic!("We don't implement this on primitive types"),
        }
    }
}

impl Index<&'_ str> for DictType {
    type Output = DictType;
    fn index(&self, s: &str) -> &DictType {
        match self {
            Self::Map(v) => v.get(s).unwrap(),
            _ => panic!("You cannot index this type with string"),
        }
    }
}

impl IndexMut<&'_ str> for DictType {
    fn index_mut(&mut self, s: &str) -> &mut Self::Output {
        match self {
            Self::Map(v) => v.get_mut(s).unwrap(),
            _ => panic!("You cannot index this type with string"),
        }
    }
}

impl Index<usize> for DictType {
    type Output = DictType;
    fn index(&self, s: usize) -> &DictType {
        match self {
            Self::Vec(v) => v.get(s).unwrap(),
            _ => panic!("You cannot index this type with number"),
        }
    }
}

impl IndexMut<usize> for DictType {
    fn index_mut(&mut self, s: usize) -> &mut Self::Output {
        match self {
            Self::Vec(v) => v.get_mut(s).unwrap(),
            _ => panic!("You cannot index this type with number"),
        }
    }
}

impl From<i32> for DictType {
    fn from(v: i32) -> DictType {
        DictType::Number(v)
    }
}

impl From<&str> for DictType {
    fn from(v: &str) -> DictType {
        DictType::String(v.to_owned())
    }
}

impl From<String> for DictType {
    fn from(v: String) -> DictType {
        DictType::String(v)
    }
}

impl From<bool> for DictType {
    fn from(v: bool) -> DictType {
        DictType::Bool(v)
    }
}

impl<T> From<Vec<T>> for DictType
where
    T: Into<DictType> + Clone,
{
    fn from(v: Vec<T>) -> DictType {
        DictType::Vec(v.iter().map(|v| Box::new((*v).clone().into())).collect())
    }
}

impl<T> From<HashMap<&str, T>> for DictType
where
    T: Copy + Into<DictType>,
{
    fn from(v: HashMap<&str, T>) -> DictType {
        DictType::Map(
            v.iter()
                .map(|(&k, &v)| (k.to_owned(), Box::new(v.into())))
                .collect(),
        )
    }
}

impl<T> From<HashMap<String, T>> for DictType
where
    T: Copy + Into<DictType>,
{
    fn from(v: HashMap<String, T>) -> DictType {
        DictType::Map(
            v.iter()
                .map(|(k, &v)| (k.clone(), Box::new(v.into())))
                .collect(),
        )
    }
}

impl Serialize for DictType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        /// Serialize while copying
        match self {
            Self::Bool(val) => serializer.serialize_bool(*val),
            Self::Number(val) => serializer.serialize_i32(*val),
            Self::String(val) => serializer.serialize_str(&val),
            Self::Vec(val) => {
                let mut seq = serializer.serialize_seq(Some(val.len()))?;
                for el in val {
                    seq.serialize_element(el);
                }
                seq.end()
            }
            Self::Map(val) => {
                let mut ser = serializer.serialize_map(Some(val.len()))?;
                for (k, v) in val.iter() {
                    ser.serialize_entry::<String, DictType>(k, &v).unwrap();
                }
                ser.end()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dict {
    values: DictType, // A map type infact
    num_entries: usize,
}

impl Dict {
    /// A container logic to hold parameters to be determined on compile time. We pass
    /// string views owned by the main application and the idea is all these variables are known at
    /// compile time.
    /// The dict is not intended to be changed runtime
    pub fn new() -> Self {
        Self {
            values: DictType::Map(HashMap::new()),
            num_entries: 0,
        }
    }

    pub fn insert<V: Into<DictType>>(&mut self, k: &str, new_value: V) {
        DictType::insert(self.values.take_map_mut(), k, new_value.into());
        self.num_entries = self.values.len();
    }

    pub fn extend(&mut self, dict: Dict) {
        self.values.extend(dict.values);
        self.num_entries = self.values.len();
    }

    pub fn len(&self) -> Option<usize> {
        Some(self.num_entries)
    }
}

impl From<&HashMap<&str, &str>> for Dict {
    fn from(val: &HashMap<&str, &str>) -> Dict {
        let mut d = Dict::new();
        for (&k, &v) in val.iter() {
            d.insert(k, v)
        }
        d
    }
}

impl From<Dict> for DictType {
    fn from(v: Dict) -> DictType {
        v.values
    }
}

impl Serialize for Dict {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map_ser = serializer.serialize_map(self.len())?;
        for (k, v) in self.values.clone().take_map().iter() {
            map_ser.serialize_entry::<String, DictType>(k, v).unwrap();
        }
        map_ser.end()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use assert_json_diff::assert_json_include;
    use serde_json;
    use simplelog::*;
    use std::{thread, time};

    #[test]
    fn test_dict() {
        let mut dict1 = Dict::new();
        dict1.insert("option1", "123");
        let mut dict2 = Dict::new();
        dict2.insert("option2", "asdfk");

        let mut dict = Dict::new();

        dict.insert("a", dict1.values);
        dict.insert("a", dict2.values);

        let actual: serde_json::Value =
            serde_json::from_str(r#"{"a": {"option1": "123", "option2": "asdfk"}}"#).unwrap();
        let expected: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&dict).unwrap()).unwrap();

        assert!(actual == expected);
    }

    #[test]
    fn test_dict_int() {
        let mut dict1 = Dict::new();
        dict1.insert("option1", 123);
        dict1.insert("option2", true);
        let mut dict2 = Dict::new();
        dict2.insert("option2", "asdfk");

        let mut dict = Dict::new();

        dict.insert("a", dict1.values);
        dict.insert("a", dict2.values);

        let actual: serde_json::Value =
            serde_json::from_str(r#"{"a": {"option1": 123, "option2": "asdfk"}}"#).unwrap();
        let expected: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&dict).unwrap()).unwrap();

        assert!(actual == expected);

        let mut dict = Dict::new();
        dict1 = Dict::new();
        dict1.insert("option1", 123);
        dict1.insert("option2", true);
        dict2 = Dict::new();
        dict2.insert("option2", "asdfk");

        // Change order to test update as dict2 should override
        dict.insert("a", dict2.values);
        dict.insert("a", dict1.values);

        let actual: serde_json::Value =
            serde_json::from_str(r#"{"a": {"option1": 123, "option2": true}}"#).unwrap();
        let expected: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&dict).unwrap()).unwrap();

        assert!(actual == expected);
    }

    #[test]
    fn test_dict_extend() {
        let mut dict1 = Dict::new();
        dict1.insert("option1", 123);
        dict1.insert("option2", true);
        let mut dict2 = Dict::new();
        dict2.insert("option3", "asdfk");
        let mut dict3 = Dict::new();
        dict3.insert("a", dict1);
        let mut dict4 = Dict::new();
        dict4.insert("a", dict2);

        dict3.extend(dict4);

        let actual: serde_json::Value =
            serde_json::from_str(r#"{"a": {"option1": 123, "option2": true, "option3":"asdfk"}}"#)
                .unwrap();
        let expected: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&dict3).unwrap()).unwrap();

        assert!(actual == expected);
    }
}
