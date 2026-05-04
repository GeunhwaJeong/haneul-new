// Test that #[expect(lint(public_entry))] suppresses the haneul lint and the
// expectation is fulfilled.
module a::m {
    #[expect(lint(public_entry))]
    public entry fun foo() {}
}
