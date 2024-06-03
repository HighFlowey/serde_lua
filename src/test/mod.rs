use std::collections::HashMap;

use serde::Deserialize;

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

#[test]
fn deserialize() {
    #[allow(dead_code)]
    #[derive(Debug, serde_derive::Deserialize, PartialEq)]
    struct Vector2 {
        x: f64,
        y: f64,
    }

    let json = LuaPestPair::from_str("return { x = 50, y = 100 }")
        .unwrap()
        .into_serde_json()
        .unwrap();

    let lhs = Vector2 { x: 50.0, y: 100.0 };
    let rhs = Vector2::deserialize(json).unwrap();

    assert_eq!(lhs, rhs);
}

#[test]
fn fail_case1() {
    let lhs = LuaValue::Int(3000);
    let rhs = LuaPestPair::from_str("return 1000")
        .unwrap()
        .into_serde_lua();

    assert!(lhs != rhs);
}

#[test]
fn fail_case2() {
    let mut hashmap = HashMap::new();
    hashmap.insert(LuaValue::String("ident"), LuaValue::Nil);

    let lhs = LuaValue::Dict(hashmap);
    let rhs = LuaPestPair::from_str("return { ident = \"cool\" }")
        .unwrap()
        .into_serde_lua();

    assert!(lhs != rhs);
}
