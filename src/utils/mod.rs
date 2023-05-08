/// convert a string to a [u8; 32] array
/// NOTE : fill the array with zeroes if the string is shorter than 32 characters
/// NOTE : truncate the string if it is longer than 32 characters
pub fn str_to_u8_array(input: &str) -> [u8; 32] {
    // Get the UTF-8 encoded bytes of the input string
    let bytes = input.as_bytes();

    // Create a mutable [u8; 32] array initialized with zeroes
    let mut result: [u8; 32] = [0u8; 32];

    // Calculate the length of the input string bytes and the result array
    let len = bytes.len().min(result.len());

    // Copy the appropriate portion of the input string bytes into the result array
    result[..len].copy_from_slice(&bytes[..len]);

    // Return the result array
    result
}