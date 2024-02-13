#include <security/pam_appl.h>
#include <security/pam_misc.h>
#include <stdio.h>

const struct pam_conv conv = {
	misc_conv,
	NULL
};

// We'll do the entire authentication proccess in pam.rs for now
// This c file will be removed once pam_start() is written for pam-rs
int main(int argc, char *argv[]) {
	pam_handle_t* pamh = NULL;
	int retval;
	const char* user = "";

	retval = pam_start("siwe-auth", user, &conv, &pamh);

	retval = pam_authenticate(pamh, 0);

	if (retval == PAM_SUCCESS) {
		printf("PAM_SUCCESS\n");
	} else {
		printf("PAM_AUTH_ERR\n");
	}

	// close PAM (end session)
	if (pam_end(pamh, retval) != PAM_SUCCESS) {
		pamh = NULL;
		printf("check_user: failed to release authenticator\n");
		exit(1);
	}

	return retval == PAM_SUCCESS ? 0 : 1;
}
