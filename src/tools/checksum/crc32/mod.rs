
pub fn crc32(data: &Vec<u8>) -> u32 {
    let crc32_table = generate_crc32_lookuptable();

    !data.iter().fold(!0, |acc, octet| {
        (acc >> 8) ^ crc32_table[((acc & 0xff) ^ *octet as u32) as usize]
    })
}

fn generate_crc32_lookuptable() -> [u32; 256] {
    let mut crc32_table = [0u32; 256];

    (0..256).for_each(|n| {
        crc32_table[n as usize] = (0..8).fold(n as u32, |acc, _| {
            match acc & 1 == 1 {
                true => 0xedb88320 ^ (acc >> 1),
                false => acc >> 1,
            }
        });
    });

    crc32_table
}

#[test]
fn crc32_quicktest() {
    let string = "The quick brown fox jumps over the lazy dog";
    let crc32 = crc32(&string.as_bytes().to_vec());
    assert_eq!(crc32, 0x414fa339);
}
