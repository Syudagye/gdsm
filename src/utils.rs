pub fn string_xor_cipher(str: &str, key: u8) -> String
{
    let p: Vec<u8> = String::from(str).into_bytes().into_iter()
        .map(|c| c ^ key).collect();
    String::from_utf8(p).unwrap()
}