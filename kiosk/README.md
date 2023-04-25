# Kiosk Ecosystem

Includes collection of transfer policies, kiosk extensions and libraries to work with all of them. It is meant to act as a Kiosk Haneul Move monorepo with a set release cycle and a very welcoming setting for external contributions.

> Currently, `published-at` field contains testnet addresses.

## Published Envs

Currently Haneul Testnet is the only supported environment, to access it, import the package directly in your Move.toml:

```toml
[dependencies]
Kiosk = { git = "https://github.com/GeunhwaJeong/haneul.git", subdir = "kiosk", rev = "main" }
```
