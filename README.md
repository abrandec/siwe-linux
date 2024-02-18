# Overview
## What is SIWE Linux?

## Repo structure
- `crates/siwe-pam`: Pluggable Authentication Module (PAM) that uses WalletConnect qr codes for authentication
- `crates/cli`: Client for configuring PAM accounts
- `crates/rollup`: Avail rollup for backing up/restoring authentication configurations

## Getting started
### 1. Requirements
1. [docker](https://www.docker.com/)
2. [act](https://github.com/nektos/act)

### 2. Configure S3 Bucket for caching dependencies

### Testing
Check the Justfile for available commands
```
./act.sh {Justfile command}

example:
./act.sh check-pam
```