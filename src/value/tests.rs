#![allow(unused_imports)]
use crate::{Array, Data, Number, Object, XffValue};

#[test]
fn general() {
    let string_val = XffValue::from("hello mom!");
    let num_val = XffValue::from(42.69);
    let array_val = XffValue::from(vec![XffValue::from("hi mom!"), XffValue::from(42.69)]);
    let object_val = XffValue::from(Object::from(vec![
        ("keyA".to_string(), XffValue::from("hi mom!")),
        ("keyB".to_string(), XffValue::from(42.69)),
    ]));
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
    assert_eq!(
        array.unwrap(),
        Array::from(vec![XffValue::from("hi mom!"), XffValue::from(42.69)])
    );
    assert_eq!(
        object.unwrap(),
        Object::from(vec![
            ("keyA".to_string(), XffValue::from("hi mom!")),
            ("keyB".to_string(), XffValue::from(42.69))
        ])
    );
    assert_eq!(data.unwrap(), Data::from(vec![1, 2, 3]));
    assert_eq!(boolean.unwrap(), true);
    assert_eq!(null, None);
}

#[test]
fn test_v3_variants() {
    use crate::value::table::Table;
    use crate::value::uuid::Uuid;

    let table_val = XffValue::Table(Table::with_columns(vec!["col1".to_string()]));
    assert!(table_val.is_table());
    assert!(table_val.into_table().is_some());

    let uuid_val = XffValue::Uuid(Uuid::new([0; 16]));
    assert!(uuid_val.is_uuid());
    assert!(uuid_val.into_uuid().is_some());

    let ord_obj_val = XffValue::OrderedObject(vec![("key".to_string(), XffValue::from(1))]);
    assert!(ord_obj_val.is_ordered_object());
    assert!(ord_obj_val.into_ordered_object().is_some());

    let nan_val = XffValue::NaN;
    assert!(nan_val.is_nan());
    assert_eq!(format!("{}", nan_val), "NaN");

    let inf_val = XffValue::Infinity;
    assert!(inf_val.is_infinity());
    assert_eq!(format!("{}", inf_val), "Infinity");

    let ninf_val = XffValue::NegInfinity;
    assert!(ninf_val.is_neg_infinity());
    assert_eq!(format!("{}", ninf_val), "-Infinity");

    let dt_val = XffValue::DateTime(123456789);
    assert_eq!(format!("{}", dt_val), "DT(123456789)");

    // Test Table::get_row
    let mut table = Table::with_columns(vec!["name".to_string(), "age".to_string()]);
    table
        .add_row(vec![XffValue::from("Alice"), XffValue::from(30)])
        .unwrap();
    let table_val = XffValue::Table(table);

    let row_obj = table_val.get_row(0).unwrap();
    assert!(row_obj.is_ordered_object());

    let ordered_data = row_obj.into_ordered_object().unwrap();
    assert_eq!(ordered_data[0].0, "name");
    assert_eq!(ordered_data[0].1, XffValue::from("Alice"));
    assert_eq!(ordered_data[1].0, "age");
    assert_eq!(ordered_data[1].1, XffValue::from(30));
}

#[test]
fn into() {
    let val = XffValue::from(42.69);
    assert_eq!(val.into_data(), None);
    assert_eq!(val.into_string(), Some("42.69".to_string()));
}

#[test]
fn time_interop() {
    use std::time::{Duration, UNIX_EPOCH};

    // DateTime
    let dt_ms = 1647081600000; // 2022-03-12 10:40:00 UTC
    let dt_val = XffValue::new_datetime(dt_ms);
    assert!(dt_val.is_datetime());
    assert_eq!(dt_val.into_datetime(), Some(dt_ms));
    assert_eq!(dt_val.into_unix_timestamp(), Some(dt_ms as f64 / 1000.0));

    let dt_from_sec = XffValue::from_unix_timestamp(dt_ms as f64 / 1000.0);
    assert_eq!(dt_from_sec.into_datetime(), Some(dt_ms));

    // Duration
    let dur_ms = 5000;
    let dur_val = XffValue::new_duration(dur_ms);
    assert!(dur_val.is_duration());
    assert_eq!(dur_val.into_duration(), Some(dur_ms));
    assert_eq!(dur_val.into_duration_seconds(), Some(5.0));
    assert_eq!(
        dur_val.into_std_duration(),
        Some(Duration::from_millis(dur_ms))
    );

    let dur_from_sec = XffValue::from_duration_seconds(5.0);
    assert_eq!(dur_from_sec.into_duration(), Some(dur_ms));

    // From implementations
    let std_dur = Duration::from_secs(10);
    let dur_val_from_std = XffValue::from(std_dur);
    assert_eq!(dur_val_from_std.into_duration(), Some(10000));

    let std_st = UNIX_EPOCH + Duration::from_secs(dt_ms / 1000);
    let dt_val_from_st = XffValue::from(std_st);
    assert_eq!(dt_val_from_st.into_datetime(), Some(dt_ms));
}

#[test]
fn mutation() {
    let mut array_val = XffValue::from(vec![1, 2, 3]);
    if let Some(array) = array_val.as_array_mut() {
        if let Some(val) = array.get_mut(1) {
            *val = XffValue::from(42);
        }
    }
    assert_eq!(
        array_val,
        XffValue::from(vec![XffValue::from(1), XffValue::from(42), XffValue::from(3)])
    );

    let mut obj_val = XffValue::from(Object::from(vec![("key".to_string(), XffValue::from(1))]));
    if let Some(obj) = obj_val.as_object_mut() {
        if let Some(val) = obj.get_mut("key") {
            *val = XffValue::from("changed");
        }
    }
    let obj = obj_val.into_object().unwrap();
    assert_eq!(obj.get("key"), Some(&XffValue::from("changed")));
}
