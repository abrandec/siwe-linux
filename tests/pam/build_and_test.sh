#!/bin/sh

# Install dependencies and build PAM module
apt-get install -y $PACKAGES
cargo build --manifest-path $PAM_MODULE_DIR/Cargo.toml
cp $ROOT_DIR/.env.example $ROOT_DIR/.env

update_env_vars() {
  local env_file="$1"
  shift # Skip the first argument to process the rest as var=value pairs

  for pair in "$@"; do
    local var_name="${pair%%=*}"
    local new_value="${pair#*=}"
  
    if grep -q "^${var_name}=" "$env_file"; then
      sed -i "s/^${var_name}=.*/${var_name}=${new_value}/" "$env_file"
    else
      echo "${var_name}=${new_value}" >> "$env_file"
    fi
  done
}

update_env_vars "$ROOT_DIR/.env" "WC_PROJECT_ID=69420"

# Install PAM module and configure PAM files
cp $PAM_MODULE_DIR/conf/siwe-auth /etc/pam.d/

# Build the PAM client and run tests
cargo build --manifest-path $PAM_CLIENT_DIR/Cargo.toml
ls $DEBUG_DIR
cargo run --bin $DEBUG_DIR/pam-siwe-client
