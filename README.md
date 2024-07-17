![Banner](https://github.com/user-attachments/assets/3ffad32b-ad7e-45c5-aa88-1ad9c68a3d08)

<p align="center">
  <img src="https://img.shields.io/github/languages/top/codeesura/taiko-spammer "Language"" alt=" Language" />
  <img src="https://img.shields.io/github/stars/codeesura/taiko-spammer "Stars"" alt=" Stars" />
  <img src="https://img.shields.io/github/issues-pr/codeesura/taiko-spammer "Pull Requests"" alt=" Pull Requests" />
  <img src="https://img.shields.io/github/issues/codeesura/taiko-spammer "Issues"" alt=" Issues" />
  <img src="https://img.shields.io/github/contributors/codeesura/taiko-spammer "Contributors"" alt=" Contributors" />
</p>

## Table of Contents

- [Stack](#stack)
- [Project Summary](#project-summary)
- [Setting Up](#setting-up)
- [Run Locally](#run-locally)
- [FAQ](#faq)
- [License](#license)

## Stack

- [reqwest](https://crates.io/crates/reqwest): For making HTTP requests, such as data fetching.
- [tokio](https://crates.io/crates/tokio): Used for running asynchronous tasks and managing future execution.
- [eyre](https://crates.io/crates/eyre): A user-friendly error handling library for Rust.
- [serde](https://crates.io/crates/serde): A powerful serialization and deserialization library for Rust.
- [serde_json](https://crates.io/crates/serde%5Fjson): A JSON serialization format for serde library.
- [dotenv](https://crates.io/crates/dotenv): Handles environment variables using .env files.
- [futures](https://crates.io/crates/futures): A library for working with asynchronous functions and tasks.
- [alloy](https://crates.io/crates/alloy): A Rust framework for building web applications with Tide.

## Project Summary

- [src](src/): Primary source code directory
- [src/transactions](src/transactions/): Transaction-related functionalities
- [src/utils](src/utils/): Utility functions and modules
- [Cargo.toml](Cargo.toml): Project configuration and dependencies
- [tests](tests/): Directory for integration and unit tests
- [benches](benches/): Directory for performance benchmarks
- [examples](examples/): Example projects and use-cases
- [docs](docs/): Documentation and project overview

## Setting Up

Insert your environment variables.

## Run Locally

1. Clone taiko-spammer repository:  
```bash  
git clone https://github.com/codeesura/taiko-spammer  
```
2. Install the dependencies with one of the package managers listed below:  
```bash  
cargo build  
```
3. Start the development mode:  
```bash  
cargo run  
```

## Contributors

[![Contributors](https://contrib.rocks/image?repo=codeesura/taiko-spammer)](https://github.com/codeesura/taiko-spammer/graphs/contributors)

## FAQ

#### 1.What is this project about?

A bot that performs WETH deposit and withdrawal operations on the Taiko network, capable of batching multiple transactions for efficiency.

#### 2.How can I contribute to this project?

Yes, we welcome contributions! Please refer to our [Contribution Guidelines](CONTRIBUTING.md) for more information on how to contribute.


## License

This project is licensed under the **MIT License** - see the [MIT License](https://github.com/codeesura/taiko-spammer/blob/main/LICENSE) file for details.
