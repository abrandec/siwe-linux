#include <security/pam_appl.h>
#include <security/pam_misc.h>
#include <stdio.h>

const struct pam_conv conv = {
	misc_conv,
	NULL
};

int main(int argc, char *argv[]) {
	pam_handle_t* pamh = NULL;
	int retval;
	const char* user = "nobody";

	if(argc != 2) {
		printf("Usage: app [username]\n");
		exit(1);
	}

	user = argv[1];

    // Test with correct password
    printf("Test with correct password\n");

	retval = pam_start("siwe-auth", user, &conv, &pamh);

	// Are the credentials correct?
	if (retval == PAM_SUCCESS) {
		printf("Credentials accepted.\n");
		retval = pam_authenticate(pamh, 0);
	}
    
	// Did everything work?
	if (retval == PAM_SUCCESS) {
		printf("Authentication Success!\n");
	} else {
		printf("Authenticated failed!\n");
	}

	// close PAM (end session)
	if (pam_end(pamh, retval) != PAM_SUCCESS) {
		pamh = NULL;
		printf("check_user: failed to release authenticator\n");
		exit(1);
	}

	return retval == PAM_SUCCESS ? 0 : 1;
}
