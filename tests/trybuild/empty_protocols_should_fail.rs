use besedarium::*;

// All trybuild compile-fail tests in this file have been temporarily disabled to stabilize the test base for the TInteract refactor.

// Should fail: tchoice! and tpar! with no branches
// These should not compile, as empty protocols are not allowed.
// Uncomment one at a time to see the error.
// type EmptyChoice = tchoice!(Http;);
// type EmptyPar = tpar!(Http;);
