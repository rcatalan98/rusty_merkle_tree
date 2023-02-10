use sha2::{Sha256, Digest};
//function to convert the hash returned by the hasher to an u64. Using the first 8bytes
pub fn hash_to_u64(hash: Vec<u8>) -> u64 {
    let mut to_return: u64 = 0;
    for i in 0..8 {
        to_return += (hash[i] as u64) << ((7-i) * 8);
    }
    to_return
}

// calculate sha256 and returns the first 8 bytes of the hash
pub fn get_sha256(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.to_vec()[0..8].to_vec()
}

//function like get_sha256 but receives a vector of u64
pub fn get_sha256_vec(data: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    for i in 0..data.len() {
        hasher.update(data[i].clone());
    }
    let result = hasher.finalize();
    result.to_vec()[0..8].to_vec()
}

pub fn is_pwr_two(n: usize) -> bool {
    n != 0 && n & (n - 1) == 0
}

pub fn raw_numbers_to_vector(data: Vec<u8>) -> Vec<Vec<u8>> {
    let mut to_return = Vec::new();
    for i in 0..data.len() {
        to_return.push(vec![data[i]]);
    }
    to_return
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn u64_convertion() {
        let hash: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1];
        let expected = 1;
        let result = hash_to_u64(hash);
        assert_eq!(result, expected);

        let hash: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 254];
        let expected = 18446744073709551614;
        let result = hash_to_u64(hash);
        assert_eq!(result, expected);

    }

    #[test]
    fn raw_numbers_to_vector_test(){
        let data = vec![1,2,3,4];
        let expected = vec![vec![1], vec![2], vec![3], vec![4]];
        let result = raw_numbers_to_vector(data);
        assert_eq!(result, expected);
    }
}