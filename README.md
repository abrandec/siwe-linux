# Overview
## What is SIWE Linux?

## Repo structure
- `crates/siwe-pam`: Pluggable Authentication Module (PAM) that uses WalletConnect qr codes for authentication
- `crates/cli`: Client for configuring PAM accounts
- `crates/rollup`: Avail rollup for backing up/restoring authentication configurations

## Getting started
### Requirements
1. [docker](https://www.docker.com/)
2. [act](https://github.com/nektos/act)

### Testing
Running tests:
```
act -j build --container-architecture linux/amd64
```