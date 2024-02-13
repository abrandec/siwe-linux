pam_dir := "crates/pam"
pam_so_dir := "/lib/x86_64-linux-gnu/security"

# Debugging PAM
install-pam:
	cp {{pam_dir}}/conf/siwe-auth /etc/pam.d/
	cp .env.example .env
	cargo build --manifest-path {{pam_dir}}/Cargo.toml
	find /lib -type d -name "*-linux-gnu"
	mv target/debug/libpam_siwe.so {{pam_so_dir}}/pam_siwe.so

test-pam:
	g++ -o {{pam_dir}}/test_pam {{pam_dir}}/test.c -lpam -lpam_misc
	./{{pam_dir}}/test_pam

# Install all dependencies
install:
	echo "This will build and install the PAM module"