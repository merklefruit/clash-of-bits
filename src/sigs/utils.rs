pub fn get_selector(fn_name: &str) -> String {
    let hash = ethers::core::utils::keccak256(fn_name.as_bytes());
    let selector = ethers::utils::hex::encode(hash[0..4].to_vec());
    selector
}
