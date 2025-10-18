extern crate arrayref;

use arrayref::array_ref;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_ref_usage() {
        // Let's create a sample slice or array.
        let data: [i32; 5] = [10, 20, 30, 40, 50];

        // Generate a reference to a subset of the array using `array_ref`.
        let subset: &[i32; 2] = array_ref!(&data, 1, 2);  // Start from index 1 and take 2 elements.

        println!("subset: {:?}", subset);
        // Assert that the subset reference is correct.
        assert_eq!(subset, &[20, 30]);
    }
}
