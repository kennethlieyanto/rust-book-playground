struct User {
    active: bool,
    username: &str,
    emaiL: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        emaiL: "someone@example.com",
        sign_in_count: 1,
    };
}
