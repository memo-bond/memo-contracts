## Creating a new repo from template

Assuming you have a recent version of rust and cargo (v1.51.0+) installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Require install [Terrain](https://docs.terra.money/docs/develop/dapp/quick-start/README.html#) cli

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
Unless you did that before, run this line now:

```sh
cargo install cargo-generate cargo-run-script --features vendored-openssl 
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:


**Latest: 0.16**
create new contract with name `demo`
```sh
make new contract-name=demo
````

## Gitpod integration

[Gitpod](https://www.gitpod.io/) container-based development platform will be enabled on your project by default.

Workspace contains:
 - **rust**: for builds
 - [wasmd](https://github.com/CosmWasm/wasmd): for local node setup and client
 - **jq**: shell JSON manipulation tool

Follow [Gitpod Getting Started](https://www.gitpod.io/docs/getting-started) and launch your workspace.

# memo-contracts
