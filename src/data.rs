pub fn get_common_words() -> Vec<&'static str> {
    vec![
        "add",
        "remove",
        "get",
        "set",
        "update",
        "create",
        "delete",
        "transfer",
        "mint",
        "burn",
        "approve",
        "allowance",
        "balance",
        "totalSupply",
        "owner",
        "name",
        "symbol",
        "decimals",
        "version",
        "pause",
        "unpause",
        "paused",
        "renounce",
    ]
}

pub fn get_common_argument_types() -> Vec<&'static str> {
    vec![
        "address", "uint256", "uint8", "uint16", "uint32", "uint64", "uint128", "uint256",
        "int256", "int8", "int16", "int32", "int64", "int128", "int256", "int512", "int1024",
        "bool", "string", "bytes", "bytes32",
    ]
}
