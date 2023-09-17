pub fn concat_to_string(slice: &[u8]) -> String {
    slice.iter().map(|&x| x.to_string()).collect()
}
