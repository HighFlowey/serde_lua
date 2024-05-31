use std::collections::HashMap;

use crate::{
    value::{IntoSerdeLua, LuaValue},
    LuaPestPair,
};

#[test]
fn return_nil() {
    let lhs = LuaValue::Nil;
    let rhs = LuaPestPair::from_str("return nil")
        .unwrap()
        .into_serde_lua();

    assert_eq!(lhs, rhs);
}

#[test]
fn return_int() {
    let lhs = LuaValue::Int(16);
    let rhs = LuaPestPair::from_str("return 16").unwrap().into_serde_lua();

    assert_eq!(lhs, rhs);
}

#[test]
fn return_negative_int() {
    let lhs = LuaValue::Int(-16);
    let rhs = LuaPestPair::from_str("return -16")
        .unwrap()
        .into_serde_lua();

    assert_eq!(lhs, rhs);
}

#[test]
fn return_float() {
    let lhs = LuaValue::Float(16.2536);
    let rhs = LuaPestPair::from_str("return 16.2536")
        .unwrap()
        .into_serde_lua();

    assert_eq!(lhs, rhs);
}

#[test]
fn return_negative_float() {
    let lhs = LuaValue::Float(-16.2536);
    let rhs = LuaPestPair::from_str("return -16.2536")
        .unwrap()
        .into_serde_lua();

    assert_eq!(lhs, rhs);
}

#[test]
fn return_array() {
    let lhs = LuaValue::Array(vec![LuaValue::Int(1), LuaValue::Int(6), LuaValue::Int(10)]);
    let rhs = LuaPestPair::from_str("return { 1, 6, 10 }")
        .unwrap()
        .into_serde_lua();

    assert_eq!(lhs, rhs);
}

#[test]
fn return_dictionary_in_array() {
    let mut hashmap = HashMap::new();
    hashmap.insert(LuaValue::String("a"), LuaValue::Int(1));

    let lhs = LuaValue::Array(vec![LuaValue::Dict(hashmap)]);
    let rhs = LuaPestPair::from_str("return { { a = 1 } }")
        .unwrap()
        .into_serde_lua();

    assert_eq!(lhs, rhs);
}

#[test]
fn return_dictionary() {
    let mut hashmap = HashMap::new();
    hashmap.insert(LuaValue::String("cool"), LuaValue::Nil);
    hashmap.insert(LuaValue::Nil, LuaValue::Int(5000));

    let lhs = LuaValue::Dict(hashmap);
    let rhs = LuaPestPair::from_str("return { cool = nil, [nil] = 5000 }")
        .unwrap()
        .into_serde_lua();

    assert_eq!(lhs, rhs);
}
