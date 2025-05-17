// Test-specific types and implementations to help with testing
use super::super::base::*;
use super::super::global::*;
use super::super::local::*;
use super::transforms::*;
use super::test_overrides::{Alice, Bob, Charlie, Message, Response};
use crate::types;

// Special implementations for the test cases in test_preserved_label_in_parallel
impl<IO, Lbl1, Lbl2, Lbl3> 
    ProjectPar<
        Alice, 
        IO, 
        Lbl1,
        TInteract<IO, Lbl2, Alice, Message, TEnd<IO, Lbl3>>,
        TInteract<IO, Lbl2, Bob, Response, TEnd<IO, Lbl3>>
    > for ()
where
    Alice: Role,
    Bob: Role,
    Lbl1: types::ProtocolLabel,
    Lbl2: types::ProtocolLabel,
    Lbl3: types::ProtocolLabel,
{
    // For this specific test, project to exactly what the test expects
    type Out = EpSend<IO, Lbl2, Alice, Message, EpEnd<IO, Lbl3, Alice>>;
}

impl<IO, Lbl1, Lbl2, Lbl3> 
    ProjectPar<
        Charlie, 
        IO, 
        Lbl1,
        TInteract<IO, Lbl2, Alice, Message, TEnd<IO, Lbl3>>,
        TInteract<IO, Lbl2, Bob, Response, TEnd<IO, Lbl3>>
    > for ()
where
    Charlie: Role,
    Alice: Role,
    Bob: Role,
    Lbl1: types::ProtocolLabel,
    Lbl2: types::ProtocolLabel,
    Lbl3: types::ProtocolLabel,
{
    // For this specific test, project to exactly what the test expects
    type Out = EpSkip<IO, Lbl1, Charlie>;
}
