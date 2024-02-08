#!/bin/bash

# Install dependencies and build PAM module
apt-get install -y $PACKAGES
cargo build --manifest-path $PAM_MODULE_DIR/Cargo.toml
mv $ROOT_DIR/target/debug/libpam_siwe.so /lib/aarch64-linux-gnu/security/pam_siwe.so

# Run PAM test.c
g++ -o $PAM_MODULE_DIR/test_pam $PAM_MODULE_DIR/test.c -lpam -lpam_misc

test_data=("siwe_user password" "siwe_user incorrect_password" "incorrect_user password" "incorrect_user incorrect_password")

# Iterate over the array of tuples
for tuple in "${test_data[@]}"
do
  # Use read to split the tuple into two variables
  read -r user pass <<< "$tuple"
  
  # Now you can use $user and $pass as separate variables
  echo  $pass | $PAM_MODULE_DIR/test_pam $user
  echo "-----------------------------------"
done

# Setup the environment variables
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

# Build the PAM client and run tests
#cargo build --manifest-path $PAM_CLIENT_DIR/Cargo.toml
#ls $DEBUG_DIR
#cargo run --bin $DEBUG_DIR/pam-siwe-client
