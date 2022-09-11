# How hard is it to find a function signature clash on Ethereum?

This is a simple weekend project meant as a fun way for me to learn Rust, so the code will inevitably be ugly and inefficient to some degree as I'm still learning the language.

## Roadmap

1. Lazy-scrape the [4Bytes](https://www.4byte.directory/) database for all known function signatures, and save them locally in a json file. (Done)
2. Load the full results into a hash map. (Done)
3. Tokenize function names and separate into words using regexes and heuristics. (Done)
4. Use Markov chains to generate new function names based on the existing ones.
5. Check if the generated function name clashes with a known function signature.
6. Repeat steps 4 & 5 billions of times and collect results

## Some context

### What are function selectors?

When writing programs that target the EVM, such as all Solidity smart-contracts, we declare functions using the `function` keyword. The function name is followed by a list of parameters, and a return type. For example:

```solidity
function foo(uint256 a, uint256 b) public returns (uint256) {
    return a + b;
}
```

This of course is only the human-readable version, that needs to be compiled into the EVM bytecode. To perform this translation, the compiler chooses a 4-bytes identifier for the function, called "selector". This is obtained by taking the first 4 bytes of the Keccak-256 hash of the function signature, which is the function name and the types of the parameters.
For instance, the function `foo` above has the signature `foo(uint256,uint256)`.

```solidity
bytes32 hash = keccak256(abi.encodePacked("foo(uint256,uint256)"));
```

`hash = 0x04bc52f87805d1b821cdd5f2eb95b2de798c17c056327397770e63e32b29a3ae`. The first 4 bytes of this hash are `0x04bc52f8`, which is our function selector.

Since the function selector is only 4 bytes, it is very possible to brute-force a collision. In fact, the selector space is 4x8 = 32 bits, which is around 4.3 billion possibilities.

### Why is this interesting?

Solidity smart contracts can have upgradability mechanisms, such as [OpenZeppelin's Proxy pattern](https://docs.openzeppelin.com/upgrades-plugins/1.x/proxies). These design patterns often rely on the fact that the function selector is unique, and that it's the only thing that is used to identify a function. This means that if a function selector is reused, the contract will call the wrong function, and potentially cause a security vulnerability.

In fact, the intentional reuse of function selectors is a known attack vector, best described in this [blog post](https://forum.openzeppelin.com/t/beware-of-the-proxy-learn-how-to-exploit-function-clashing/1070) by OpenZeppelin, called "Evil Proxy". It has been used in real-world hacks. If you want to read more about this kind of attacks, I recommend reading the [Upgadability of Smart Contracts](https://arxiv.org/pdf/2206.00716.pdf) paper.
