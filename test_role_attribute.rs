use besedarium_derive::role;

// Test the #[role] attribute macro without display_name
#[role]
struct BasicRole;

// Test the #[role] attribute macro with display_name
#[role(display_name = "Custom Display Name")]
struct CustomRole;

fn main() {
    let basic = BasicRole;
    let custom = CustomRole;
    
    println!("BasicRole: {}", basic);
    println!("CustomRole: {}", custom);
    
    // Test that they implement the Role trait
    fn assert_role<T: besedarium::protocol::foundation::Role>(_: T) {}
    assert_role(basic);
    assert_role(custom);
    
    println!("All tests passed!");
}
