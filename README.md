# vehicle-rent

Develop a program using Anchor with features

- Creating vehicle NFTs with add_car instruction (ADMIN ONLY)

- Register as a user account that has a score (A wallet has one user account that corresponds to its publicKey)

- Deposit and withdraw functionalities to the user account registered and sending dollars as amounts that utilizes the Pyth oracle to get the SOL/USD price.

- Rent a car by providing the rent time in days, and the amount to pay that will be transferred to the rent account. 

- Admin can close the end account after its rent time passed and receive the lamports in the rent account after the close of the account.

Devnet https://explorer.solana.com/address/BGQFcsJJopkMEXHP3hLcad4dpKRbPUvh5utMA4932HxF?cluster=devnet

## Getting Started

### Prerequisites

- [Node v18.18.0 or higher](https://nodejs.org/en/download/package-manager)
- [Rust v1.77.2 or higher](https://www.rust-lang.org/learn/get-started)
- [Anchor CLI 0.30.1 or higher](https://www.anchor-lang.com/docs/installation)
- [Solana CLI 1.18.9 or higher](https://solana.com/docs/intro/installation)

### Dependencies

[dependencies]<br>


### Anchor.toml
```toml
[toolchain]
anchor_version = "0.30.1"
```
### Installation

#### Clone the repo

```shell
git clone https://github.com/iricehasan/solana-vehicle-rental.git
cd solana-vehicle-rental
```

#### Install Dependencies

```shell
yarn install
```


## Apps

### Anchor

This is a Solana program written in Rust using the Anchor framework.

#### Commands

You can use any normal anchor commands. Make sure you have the Anchor CLI 0.30.1 and Solana CLI 1.18.8 or higher.

#### Sync the program id:

Running this command will create a new keypair in the `anchor/target/deploy` directory and save the address to the Anchor config file and update the `declare_id!` macro in the `./src/lib.rs` file of the program.

You will manually need to update the constant in `anchor/lib/vehicle-rental.ts` to match the new program id.

```shell
anchor keys sync
```

#### Build the program:

```shell
anchor build
```

#### Deploy the program:

```shell
anchor deploy
```

#### Run the tests

To run all the tests

```shell
anchor run test
```

Also, you can run the tests one instruction

```shell
anchor run initialize
```

```shell
anchor run register-user
```

```shell
anchor run deposit
```

```shell
anchor run add-car
```

```shell
anchor run rent-car
```

```shell
anchor run end-rent
```

```shell
anchor run withdraw
```

#### Deploy to Devnet

```shell
anchor deploy --provider.cluster devnet
```
