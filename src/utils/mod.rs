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

/// apply the score on the scale (the score begin at 0, and every score_interval point, 
/// the scale increase by scale_interval)
pub fn get_scale_value(
    max : f64, 
    min : f64, 
    scale_interval : f64, 
    score : u64, 
    score_interval : f64,
    reverse : bool
) -> f64 {
    let nb_interval = ((score as f64)/ score_interval) as u64;
    let scale_interval = scale_interval * (nb_interval as f64);
    let scale: f64;
    if reverse {
        scale = max - scale_interval ;
        scale.max(min)
    } else {
        scale = scale_interval + min ;
        scale.min(max)
    }
    
}

/// check if 2 squares overlap
pub fn check_collision(
    x1: f64,
    y1: f64,
    width1: u16,
    height1: u16,
    x2: f64,
    y2: f64,
    width2: u16,
    height2: u16,
) -> bool {
    let right1 = x1 + width1 as f64;
    let bottom1 = y1 + height1 as f64;
    let right2 = x2 + width2 as f64;
    let bottom2 = y2 + height2 as f64;

    // Check for collision by comparing the boundaries
    !(right1 < x2 || x1 > right2 || bottom1 < y2 || y1 > bottom2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_value(){
        let max = 10.0;
        let min = 2.0;
        let scale_interval = 2.0;
        let score_interval = 10.0;
        assert_eq!(get_scale_value(max, min, scale_interval, 0, score_interval, false), min);
        assert_eq!(get_scale_value(max, min, scale_interval, 5, score_interval, false), min);
        assert_eq!(get_scale_value(max, min, scale_interval, 10, score_interval, false), min + scale_interval);
        assert_eq!(get_scale_value(max, min, scale_interval, 15, score_interval, false), min + scale_interval);
        assert_eq!(get_scale_value(max, min, scale_interval, 20, score_interval, false), min + scale_interval * 2.0);
        assert_eq!(get_scale_value(max, min, scale_interval, 150, score_interval, false), max);


        assert_eq!(get_scale_value(max, min, scale_interval, 0, score_interval, true), max);
        assert_eq!(get_scale_value(max, min, scale_interval, 5, score_interval, true), max);
        assert_eq!(get_scale_value(max, min, scale_interval, 10, score_interval, true), max - scale_interval);
        assert_eq!(get_scale_value(max, min, scale_interval, 15, score_interval, true), max - scale_interval);
        assert_eq!(get_scale_value(max, min, scale_interval, 20, score_interval, true), max - scale_interval * 2.0);
        assert_eq!(get_scale_value(max, min, scale_interval, 150, score_interval, true), min);


    }

    #[test]
    fn test_check_collision() {
        // Test case 1: Squares overlap
        let collision_1 = check_collision(0.0, 0.0, 5, 5, 3.0, 3.0, 5, 5);
        assert!(collision_1);

        // Test case 2: Squares do not overlap
        let collision_2 = check_collision(0.0, 0.0, 5, 5, 10.0, 10.0, 5, 5);
        assert!(!collision_2);

        // Test case 3: Squares partially overlap
        let collision_3 = check_collision(0.0, 0.0, 5, 5, 3.0, 3.0, 5, 10);
        assert!(collision_3);

        // Test case 4: Squares share an edge
        let collision_4 = check_collision(0.0, 0.0, 5, 5, 5.0, 5.0, 5, 5);
        assert!(collision_4);

        // negatif

        // Test case 1: Squares overlap
        let collision_5 = check_collision(-5.0, -5.0, 10, 10, -3.0, -3.0, 5, 5);
        assert!(collision_5);

        // Test case 2: Squares do not overlap
        let collision_6 = check_collision(-5.0, -5.0, 10, 10, 10.0, 10.0, 5, 5);
        assert!(!collision_6);

        // Test case 3: Squares partially overlap
        let collision_7 = check_collision(-5.0, -5.0, 10, 10, -3.0, -3.0, 5, 20);
        assert!(collision_7);

        // Test case 4: Squares share an edge (no overlap)
        let collision_8 = check_collision(-5.0, -5.0, 10, 10, 5.0, 5.0, 10, 10);
        assert!(collision_8);
    }

}