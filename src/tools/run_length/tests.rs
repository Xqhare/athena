#![allow(unused_imports)]
use super::*;

#[test]
fn serde_simple() {
    let data_no_double = "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    let data_some_double = "lorem ipsummm doloor siiittt ameeet consecteetttur adipiscinggg ellittt seddd do eiusmmod temppor incididddduunt ut labbore eet dolllllore maggna aliquaa.";
    let compressed_no_double = run_length_encoder(&data_no_double.as_bytes().to_vec());
    let compressed_some_double = run_length_encoder(&data_some_double.as_bytes().to_vec());
    let decompressed_no_double = run_length_decoder(&compressed_no_double);
    let decompressed_some_double = run_length_decoder(&compressed_some_double);
    assert_eq!(String::from_utf8(decompressed_no_double).unwrap(), data_no_double);
    assert_eq!(String::from_utf8(decompressed_some_double).unwrap(), data_some_double);
}

#[test]
fn serde_advanced() {
    let data0 = {
        let abc = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        let mut s = String::new();
        for i in 0..1000 {
            for _ in 0..3 {
                s.push(abc[i % abc.len()]);
            }
        }
        s.as_bytes().to_vec()
    };
    assert_eq!(data0.len(), 3000);
    let compressed0 = run_length_encoder(&data0);
    let decompressed0 = run_length_decoder(&compressed0);
    assert_eq!(compressed0.len(), 1000);
    assert_eq!(decompressed0, data0);
    assert!(compressed0.len() < data0.len());
}

#[test]
fn serde_real_data() {
    let data0 = std::fs::read_to_string("README.md").unwrap();
    let data1 = std::fs::read_to_string("LICENSE").unwrap();
    let data2 = std::fs::read_to_string("src/utils/compression/run_length/mod.rs").unwrap();
    let compressed0 = run_length_encoder(&data0.as_bytes().to_vec());
    let compressed1 = run_length_encoder(&data1.as_bytes().to_vec());
    let compressed2 = run_length_encoder(&data2.as_bytes().to_vec());
    let decompressed0 = run_length_decoder(&compressed0);
    let decompressed1 = run_length_decoder(&compressed1);
    let decompressed2 = run_length_decoder(&compressed2);
    assert_eq!(String::from_utf8(decompressed0).unwrap(), data0);
    assert_eq!(String::from_utf8(decompressed1).unwrap(), data1);
    assert_eq!(String::from_utf8(decompressed2).unwrap(), data2);

    // Not guaranteed to be true, but highly likely
    assert!(compressed0.len() < data0.len());
    assert!(compressed1.len() < data1.len());
    assert!(compressed2.len() < data2.len());
}
