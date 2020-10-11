# honeycomb-rs

A solver(-ish) for New York Times's [Spelling Bee](https://www.nytimes.com/puzzles/spelling-bee). Presents all of the
words located in the system's `words` file that:

* Are 4 or more characters long

* Contain the "prime" character (i.e. the one character that all words must contain)

* Contain only the available characters, which are the prime character and any other charcters provided on the CLI

This was going to be more interesting (something about a trie, probably), but then I realized that I'd rather see if I
can write Python extensions in Rust.

## License

MIT Open Source License. See `LICENSE` file for more info.
