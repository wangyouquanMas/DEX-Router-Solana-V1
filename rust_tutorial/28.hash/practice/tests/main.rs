use crate::hash::hash;

pub mod hash {
    pub fn hash(input: &[u8]) -> [u8; 32] {
        let mut result = [0u8; 32];
        for (i, &byte) in input.iter().enumerate() {
            result[i % 32] = byte;  // Simple mock hash logic
        }
        result
    }
}

pub fn sighash(namespace: &str, name: &str) -> [u8;8]{
    let preimage = format!("{namespace}:{name}");

    let mut sighash = [0u8;8];
    sighash.copy_from_slice(&crate::hash::hash(preimage.as_bytes())[..8]);
    sighash
}



#[test]
fn test_sighash(){
    let namespace = "namespace";
    let name = "name";

    let result = sighash(namespace,name);

    println!("{:?}",result);
}