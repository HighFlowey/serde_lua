use crate::{LuaPestPair, Rule};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

pub trait IntoSerdeLua<'a> {
    fn into_serde_lua(self) -> LuaValue<'a>;
}

impl<'config> IntoSerdeLua<'config> for LuaPestPair<'config> {
    fn into_serde_lua(self) -> LuaValue<'config> {
        let pair = self.0;

        match pair.as_rule() {
            Rule::array => {
                let inner = pair.into_inner();

                let mut vec: Vec<LuaValue> = Vec::new();

                for field in inner {
                    let mut field_inner = field.into_inner();
                    let value = field_inner.next().unwrap();

                    vec.push(LuaPestPair::from_pair(value).into_serde_lua());
                }

                LuaValue::Array(vec)
            }
            Rule::dict => {
                let inner = pair.into_inner();

                let mut map: HashMap<LuaValue, LuaValue> = HashMap::new();

                for field in inner {
                    let mut field_inner = field.into_inner();

                    let key = field_inner.next().unwrap();
                    let value = field_inner.next().unwrap();

                    map.insert(
                        LuaPestPair::from_pair(key).into_serde_lua(),
                        LuaPestPair::from_pair(value).into_serde_lua(),
                    );
                }

                LuaValue::Dict(map)
            }
            Rule::int => match pair.as_str().parse::<i32>() {
                Ok(value) => LuaValue::Int(value),
                Err(err) => panic!("{err}"),
            },
            Rule::float => match pair.as_str().parse::<f64>() {
                Ok(value) => LuaValue::Float(value),
                Err(err) => panic!("{err}"),
            },
            Rule::string => {
                let _raw = pair.as_str();
                let value = if _raw.starts_with('"') {
                    _raw.trim_matches('"')
                } else {
                    _raw.trim_matches('\'')
                };
                
                LuaValue::String(value)
            }
            Rule::boolean => LuaValue::Boolean(pair.as_str() == "true"),
            Rule::nil => LuaValue::Nil,
            Rule::ident => LuaValue::String(pair.as_str().trim()),
            _ => unimplemented!("{pair}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LuaValue<'a> {
    Nil,
    Int(i32),
    Float(f64),
    Boolean(bool),
    String(&'a str),
    Array(Vec<LuaValue<'a>>),
    Dict(HashMap<LuaValue<'a>, LuaValue<'a>>),
}

impl<'a> Eq for LuaValue<'a> {}

impl<'a> PartialEq for LuaValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Compare basic variants directly
            (LuaValue::Nil, LuaValue::Nil) => true,
            (LuaValue::Int(i), LuaValue::Int(j)) => i == j,
            (LuaValue::Float(f), LuaValue::Float(g)) => f == g,
            (LuaValue::Boolean(b), LuaValue::Boolean(c)) => b == c,
            (LuaValue::String(s), LuaValue::String(t)) => s == t,
            // For complex variants, compare their contents
            (LuaValue::Array(a), LuaValue::Array(b)) => a == b,
            (LuaValue::Dict(d), LuaValue::Dict(e)) => d == e,
            _ => false,
        }
    }
}

impl<'a> Hash for LuaValue<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.clone() {
            LuaValue::Nil => state.write_u8(0),
            LuaValue::Int(i) => i.hash(state),
            LuaValue::Float(f) => f.to_bits().hash(state),
            LuaValue::Boolean(b) => b.hash(state),
            LuaValue::String(s) => s.hash(state),
            LuaValue::Array(a) => {
                for v in a {
                    v.hash(state)
                }
            }
            LuaValue::Dict(d) => {
                let iter = d.into_iter();
                for (k, v) in iter {
                    k.hash(state);
                    v.hash(state);
                }
            }
        }
    }
}
