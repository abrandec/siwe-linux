use pam;

#[test]
fn test_pam() {

    let res = pam::start("test", Some("test"), "test");
}