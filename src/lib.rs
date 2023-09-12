pub mod test;

pub mod lzw {
    use std::collections::HashMap;

    pub fn compress(data: &[u8]) -> Vec<u32> {
        // Build the initial dictionary with single-byte sequences.
        let mut dictionary: HashMap<Vec<u8>, u32> =
            (0u32..=255).map(|i| (vec![i as u8], i)).collect();

        let mut current_sequence = Vec::new();
        let mut compressed = Vec::new();

        for &byte in data {
            // Try to extend the current sequence.
            current_sequence.push(byte);

            // Check if the extended sequence is in the dictionary.
            if let Some(&_code) = dictionary.get(&current_sequence) {
                // The sequence is in the dictionary, continue to extend it.
                continue;
            }

            // The sequence is not in the dictionary, so add it.
            // Write the code for the current sequence to the output.
            if let Some(&code) = dictionary.get(&current_sequence[..current_sequence.len() - 1]) {
                compressed.push(code);
            } else {
                panic!("Invalid dictionary state.");
            }

            // Add the new sequence to the dictionary.
            dictionary.insert(current_sequence.clone(), dictionary.len() as u32);

            // Reset the current sequence to the last byte.
            current_sequence.clear();
            current_sequence.push(byte);
        }

        // Write the code for the last sequence to the output.
        if let Some(&code) = dictionary.get(&current_sequence) {
            compressed.push(code);
        } else {
            panic!("Invalid dictionary state.");
        }

        compressed
    }

    pub fn decompress(mut data: &[u32]) -> Vec<u8> {
        // Build the initial dictionary with single-byte sequences.
        let mut dictionary: HashMap<u32, Vec<u8>> =
            (0u32..=255).map(|i| (i, vec![i as u8])).collect();

        let mut result = Vec::new();

        let mut current_code = data[0];
        let mut entry = dictionary[&current_code].clone();
        result.extend_from_slice(&entry);

        for &code in &data[1..] {
            let mut new_entry = dictionary.get(&code).cloned();

            if new_entry.is_none() {
                if code == dictionary.len() as u32 {
                    new_entry = Some(entry.clone());
                    new_entry.as_mut().unwrap().push(entry[0]);
                } else {
                    panic!("Invalid data!");
                }
            }

            entry.extend_from_slice(new_entry.as_ref().unwrap());

            dictionary.insert(dictionary.len() as u32, entry.clone());
            result.extend_from_slice(new_entry.as_ref().unwrap());

            current_code = code;
        }

        result
    }
}
