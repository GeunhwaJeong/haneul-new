// Test that #[deny(lint(public_entry))] upgrades the haneul lint warning to an error.
module a::m {
    #[deny(lint(public_entry))]
    public entry fun foo() {}
}
