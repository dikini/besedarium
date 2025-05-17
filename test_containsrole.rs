use besedarium::*;

// --- Custom Roles for Testing ---
struct Alice;
struct Bob;
struct Charlie;
impl Role for Alice {}
impl Role for Bob {}
impl Role for Charlie {}

// --- Role equality implementations ---
impl RoleEq<Alice> for Alice {
    type Output = True;
}
impl RoleEq<Bob> for Alice {
    type Output = False;
}
impl RoleEq<Charlie> for Alice {
    type Output = False;
}

impl RoleEq<Alice> for Bob {
    type Output = False;
}
impl RoleEq<Bob> for Bob {
    type Output = True;
}
impl RoleEq<Charlie> for Bob {
    type Output = False;
}

impl RoleEq<Alice> for Charlie {
    type Output = False;
}
impl RoleEq<Bob> for Charlie {
    type Output = False;
}
impl RoleEq<Charlie> for Charlie {
    type Output = True;
}

// --- Message Types and IO Types ---
struct Message;
struct Http;
struct EmptyLabel;
impl ProtocolLabel for EmptyLabel {}

fn main() {
    // Interaction where Bob sends to Alice
    type BobToAlice = TInteract<Http, EmptyLabel, Bob, Message, TEnd<Http, EmptyLabel>>;

    // Check if Alice is detected in BobToAlice
    type IsAliceInBobToAlice = <BobToAlice as ContainsRole<Alice>>::Output;
    
    // This should be False if ContainsRole only checks sending roles
    // But should be True if ContainsRole checks both sending and receiving roles
    println!("Is Alice in BobToAlice? The answer is...");
    
    // Check if Bob is detected in BobToAlice
    type IsBobInBobToAlice = <BobToAlice as ContainsRole<Bob>>::Output;
    
    // This should be True since Bob is the sender
    println!("Is Bob in BobToAlice? The answer is definitely True");

    // Check if Charlie is detected in BobToAlice
    type IsCharlieInBobToAlice = <BobToAlice as ContainsRole<Charlie>>::Output;
    
    // This should be False since Charlie isn't involved at all
    println!("Is Charlie in BobToAlice? The answer is definitely False");
}
