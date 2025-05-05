
#[test]
fn simple_delta_encoding() {
    let data = vec![1, 2, 3, 4, 5];
    let compressed = super::delta_encoder(&data);
    assert_eq!(compressed, vec![1, -1, -1, -1, -1]);

    let data2 = vec![5, 4, 3, 2, 1];
    let compressed2 = super::delta_encoder(&data2);
    assert_eq!(compressed2, vec![5, 1, 1, 1, 1]);
}

#[test]
fn advanced_delta_encoding() {
    let data = vec![1, 10, 2, 30, 3, 55, 4, 80, 5, 110];
    let compressed = super::delta_encoder(&data);
    assert_eq!(compressed, vec![1, -9, 8, -28, 27, -52, 51, -76, 75, -105]);
}

#[test]
fn simple_delta_decoding() {
    let compressed = vec![1, -1, -1, -1, -1];
    let decompressed = super::delta_decoder(&compressed);
    assert_eq!(decompressed, vec![1, 2, 3, 4, 5]);

    let compressed2 = vec![5, 1, 1, 1, 1];
    let decompressed2 = super::delta_decoder(&compressed2);
    assert_eq!(decompressed2, vec![5, 4, 3, 2, 1]);
}

#[test]
fn advanced_delta_decoding() {
    let compressed = vec![1, -9, 8, -28, 27, -52, 51, -76, 75, -105];
    let decompressed = super::delta_decoder(&compressed);
    assert_eq!(decompressed, vec![1, 10, 2, 30, 3, 55, 4, 80, 5, 110]);
}

#[test]
fn simple_encoding_and_decoding() {
    let data = "The quick brown fox jumps over the lazy dog";
    let compressed = super::delta_encoder(&data.as_bytes().to_vec());
    let decompressed = super::delta_decoder(&compressed);
    assert_eq!(String::from_utf8(decompressed).unwrap(), data);

    let mut small_vec = Vec::with_capacity(10);
    for i in 100..110 {
        small_vec.push(i);
    }
    let compressed = super::delta_encoder(&small_vec);
    let decompressed = super::delta_decoder(&compressed);
    assert_eq!(decompressed, small_vec);
}

#[test]
fn advanced_encoding_and_decoding() {
    let data = "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    let compressed = super::delta_encoder(&data.as_bytes().to_vec());
    let decompressed = super::delta_decoder(&compressed);
    assert_eq!(String::from_utf8(decompressed).unwrap(), data);

    let large_num = u64::MAX;
    let compressed = super::delta_encoder(&large_num.to_le_bytes().to_vec());
    let decompressed = super::delta_decoder(&compressed);
    assert_eq!(decompressed.len(), 8);
    assert_eq!(u64::from_le_bytes(decompressed.try_into().unwrap()), large_num);

    let file = std::fs::read_to_string("README.md").unwrap();
    let compressed = super::delta_encoder(&file.as_bytes().to_vec());
    let decompressed = super::delta_decoder(&compressed);
    assert_eq!(String::from_utf8(decompressed).unwrap(), file);
}

