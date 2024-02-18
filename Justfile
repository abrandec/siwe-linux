#############
# Variables #
#############

# Crate directories. Set here in case we want to change them later
pam_dir := "crates/pam"
client_dir := "crates/client"
rollup_dir := "crates/rollup"
pam_tests_dir := "crates/pam-tests"

# This is the directory where the PAM module will be installed. Limited to Ubuntu x86_64-linux-gnu for now
pam_so_dir := "/lib/x86_64-linux-gnu/security"

#######################
# PAM module commands #
#######################

check-pam:
	cargo check --manifest-path {{pam_dir}}/Cargo.toml

build-pam:
	cargo build --manifest-path {{pam_dir}}/Cargo.toml

# Debugging PAM
install-pam:
	cp {{pam_dir}}/conf/siwe-auth /etc/pam.d/
	cp .env.example .env
	mv target/debug/libpam_siwe.so {{pam_so_dir}}/pam_siwe.so

test-pam:
	just build-pam
	just install-pam
	cargo build --manifest-path {{pam_tests_dir}}/Cargo.toml
	./target/debug/pam-tests

#######################
# All-in-one commands #
#######################

build-all:
	cargo build --release

# Install all dependencies
install-all:
	echo "This will build and install the PAM module, client, and rollup"