# How hard is it to find a function signature clash on Ethereum?

This is a simple weekend project meant as a fun way for me to learn Rust, so the code will inevitably be ugly and inefficient to some degree as I'm still learning the language.

## Roadmap

1. Lazy-scrape the [4Bytes](https://www.4byte.directory/) database for all known function signatures, and save them locally in a json file. (Done)
2. Load the full results into a hash map. (Done)
3. Tokenize function names and separate into words using regexes and heuristics. (Done)
4. Use Markov chains to generate new function names based on the existing ones. (Doing)
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

### What is a Markov Chain? Why am I using it?

A Markov chain is a mathematical model that describes a sequence of possible events in which the probability of each event depends only on the state attained in the previous event. In other words, it's a model that describes the probability of a sequence of events, where the probability of each event depends only on the previous event.

In this case, I'm using a Markov chain to generate new function names based on the existing ones. To generate a new function name, I look at all n-grams (that is, all sequences of n characters) in the existing corpus of function names, and figure out what is the most likely n-gram to follow each one. Let's take this example:

`transferWETHFromMultisig`

The n-grams of order 3 in this function name are:

`tra`, `ran`, `ans`, `nse`, `ser`, `erW`, `rWE`, `WET`, `ETH`, `THF`, `HFr`, `Fro`, `rom`, `omM`, `mMu`, `Mul`, `ult`, `lts`, `tsi`, `sig`.

Then I can easily build a map of the n-grams occurring most frequently after each one.

This means that if I start with the n-gram `tra`, the most likely n-gram to follow it is `nsf`. If I start with `ans`, the most likely n-gram to follow it is `fer`. And so on.

I can use this information to generate new function names, by starting with a random n-gram in the corpus and picking the following n-grams based on the rules explained above.

To be more specific, I don't only look at the most likely n-gram to follow each one, but rather I am using a hashmap describing the probabilities for each following n-gram to occur. It looks something like this:

`tra` -> `nsf` (0.5), `smi` (0.3), `ppe` (0.2)

so if I start with `tra`, there is a 50% chance the following n-gram is `nsf`, but there is also a 30% chance that it will be `smi` and a 20% chance that it will be `ppe`.
