#[cfg(test)]
mod tests {

    #[test]
    fn test_compress() {
        // Test data: a simple repetitive sequence
        let data = vec![1, 2, 3, 4, 1, 2, 3, 5];

        // Expected compressed result (codes)
        let expected_result = vec![1, 2, 3, 4, 256, 3, 5];

        // Compress the data
        let compressed = crate::lzw::compress(&data);

        // Check if the compressed result matches the expected result
        assert_eq!(compressed, expected_result);
    }

    #[test]
    fn test_decompress() {
        // Test data: a simple repetitive sequence
        let data = vec![1, 2, 3, 4, 256, 3, 5];

        // Expected decompressed result (codes)
        let expected_result = vec![1, 2, 3, 4, 1, 2, 3, 5];

        // Decompress the data
        let decompressed = crate::lzw::decompress(&data);

        // Check if the compressed result matches the expected result
        assert_eq!(decompressed, expected_result);
    }
}
