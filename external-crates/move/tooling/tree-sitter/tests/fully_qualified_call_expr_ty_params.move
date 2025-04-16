module a::b;

fun f() {
    let x = haneul::dynamic_field::borrow<vector<u8>, u64>(&parent, b"");
    let x = ::haneul::dynamic_field::borrow<vector<u8>, u64>(&parent, b"");
}
