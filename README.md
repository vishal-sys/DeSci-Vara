# DeSci Funding Vara

DeSci Funding Vara is a blockchain-based funding platform for scientific research, leveraging the Vara Network and Sails framework. This project enables transparent, community-driven funding for high-risk, groundbreaking scientific experiments.

## Features

- **Decentralized Funding:** Direct investment in scientific projects through smart contracts.
- **Transparent Contributions:** View funding progress and contributor details for each project.
- **Tokenized Rewards:** Distribute rewards to contributors when funding goals are met.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version)
- Vara Network environment
- [Sails Framework](https://vara.network/sails)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/desci-funding-vara.git
   cd desci-funding-vara
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests to ensure everything is working:
   ```bash
   cargo test
   ```

## Usage

1. Deploy the smart contract:
   ```bash
   sails deploy
   ```

2. Interact with the contract using the Vara client:
   ```bash
   sails call <function_name> <parameters>
   ```

3. View logs and activity:
   ```bash
   sails logs
   ```

## Project Structure

- `src/`
  - Contains the main Rust code for the smart contract.
- `Cargo.toml`
  - Lists dependencies and configuration for the project.
- `client/`
  - Contains the client-side scripts to interact with the smart contract.
- `app/`
  - Contains the app configuration and deployment scripts.

## Contribution

Contributions are welcome! Please fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any inquiries, please contact [Vishal](mailto:your.vishalsaroj6363@gmail.com).
