# Haneul Explorer

[Haneul Explorer](https://explorer.haneul.io/) is a network explorer for the Haneul network, similar in functionality to [Etherscan](https://etherscan.io/) or [Solana Explorer](https://explorer.solana.com/). Use Haneul Explorer to see the latest transactions and objects.

# Set Up

**Requirements**: Node 18.0.0 or later.

Dependencies are managed using [`pnpm`](https://pnpm.io/). You can start by installing dependencies in the root of the Haneul repository:

```
$ pnpm install
```

> All `pnpm` commands below are intended to be run in the root of the Haneul repo.

## Developing the Haneul Explorer

To start the explorer dev server, you can run the following command:

```
pnpm explorer dev
```

This will start the dev server on port 3000, which should be accessible on http://localhost:3000/

## To run end-to-end localnet test

Start validators locally:

```bash
cargo run --bin haneul-test-validator
```

In a a separate terminal, you can now run the end-to-end tests:

```bash
pnpm --filter haneul-explorer playwright test
```

# Other pnpm commands

### `pnpm explorer test`

This runs a series of end-to-end browser tests using the website as connected to the static JSON dataset. This command is run by the GitHub checks. The tests must pass before merging a branch into main.

### `pnpm explorer build`

Builds the app for production to the `build` folder.

It bundles React in production mode and optimizes the build for the best performance.

### `pnpm explorer lint`

Run linting check (prettier/eslint).
