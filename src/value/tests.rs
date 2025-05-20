#![allow(unused_imports)]
use crate::{XffValue, Number, Array, Object, Data};

#[test]
fn general() {
    let string_val = XffValue::from("hello mom!");
    let num_val = XffValue::from(42.69);
    let array_val = XffValue::from(
        vec![
            XffValue::from("hi mom!"),
            XffValue::from(42.69)
        ]
    );
    let object_val = XffValue::from(
        Object::from(
           vec![
               ("keyA".to_string(), XffValue::from("hi mom!")),
               ("keyB".to_string(), XffValue::from(42.69))
           ]
        )
    );
    let data_val = XffValue::from(Data::from(vec![1, 2, 3]));
    let boolean_val = XffValue::from(true);
    let null_val = XffValue::Null;

    assert!(string_val.is_string());
    assert!(num_val.is_number());
    assert!(array_val.is_array());
    assert!(object_val.is_object());
    assert!(data_val.is_data());
    assert!(boolean_val.is_boolean());
    assert!(null_val.is_null());

    let string = string_val.into_string();
    let num = num_val.into_number();
    let array = array_val.into_array();
    let object = object_val.into_object();
    let data = data_val.into_data();
    let boolean = boolean_val.into_boolean();
    let null = null_val.into_null();

    assert_eq!(string.is_some(), true);
    assert_eq!(num.is_some(), true);
    assert_eq!(array.is_some(), true);
    assert_eq!(object.is_some(), true);
    assert_eq!(data.is_some(), true);
    assert_eq!(boolean.is_some(), true);
    assert_eq!(null.is_none(), true);

    assert_eq!(string.unwrap(), "hello mom!");
    assert_eq!(num.unwrap(), Number::from(42.69));
    assert_eq!(array.unwrap(), Array::from(vec![XffValue::from("hi mom!"), XffValue::from(42.69)]));
    assert_eq!(object.unwrap(), Object::from(vec![("keyA".to_string(), XffValue::from("hi mom!")), ("keyB".to_string(), XffValue::from(42.69))]));
    assert_eq!(data.unwrap(), Data::from(vec![1, 2, 3]));
    assert_eq!(boolean.unwrap(), true);
    assert_eq!(null, None);
}

#[test]
fn into() {
    let val = XffValue::from(42.69);
    assert_eq!(val.into_data(), None);
    assert_eq!(val.into_string(), Some("42.69".to_string()));
}
