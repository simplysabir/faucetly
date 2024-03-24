
# Faucetly

## Overview

Faucetly is a command-line interface (CLI) tool built in Rust, designed to automate the process of receiving daily Solana airdrops into a specified wallet. This tool simplifies the process of scheduling and executing airdrops, ensuring your Solana wallet is consistently funded.

## Features

- **Automated Airdrops**: Schedule Solana airdrops to occur daily without manual intervention.
- **Simple Configuration**: Save your Solana wallet address once and let the tool handle the rest.
- **Ease of Use**: A user-friendly command-line interface that makes Solana transactions effortless.

## Installation

To use the Faucetly, ensure Rust is installed on your system. You can set up the tool with the following steps:

```
git clone https://github.com/simplysabir/faucetly.git
cd faucetly
cargo build --release
```

or simply install it from here [crates]("https://crates.io/crates/faucetly")

The compiled binary will be located in the `target/release/` directory. Alternatively, watch for its release on [crates.io](https://crates.io/faucetly) for an easier installation process.

## Usage

First, save your wallet address:

```
faucetly save YOUR_WALLET_ADDRESS
```

Then, start the scheduler to run airdrops daily:

```
faucetly
```

**Note:** The tool must be running for the scheduled airdrops to occur. Consider using your operating system's task scheduler for automatic starts.

## Error Handling

Faucetly provides clear error messages to assist with troubleshooting issues related to file access, network errors, or command execution.

## Contributions

Your contributions are welcome! If you'd like to improve the Faucetly, please fork the repository and submit a pull request with your changes. For significant modifications, open an issue first to discuss what you'd like to alter.

Explore more projects and contributions on [GitHub](https://github.com/simplysabir) or visit my [Portfolio](https://simplysabir.live/).

## License

This project is licensed under the MIT License - see the LICENSE file for details.
