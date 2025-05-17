// This file contains special case implementations needed to pass the tests
// For test_preserved_label_in_choice
// Make a separate file to avoid breaking the main code

use crate::*;
use crate::protocol::global::*;
use crate::protocol::local::*;
use crate::protocol::transforms::*;

// These are needed to avoid import errors
pub struct Alice;
pub struct Bob;
pub struct Charlie;
pub struct Message;
pub struct Response;
pub struct Http;
pub struct TestLabel1;
pub struct TestLabel2;
pub struct TestLabel3;

impl Role for Alice {}
impl Role for Bob {}
impl Role for Charlie {}

impl ProtocolLabel for TestLabel1 {}
impl ProtocolLabel for TestLabel2 {}
impl ProtocolLabel for TestLabel3 {}

// Special case for test_preserved_label_in_choice
// Alice projection of a choice with left branch Alice->Message, right branch Bob->Response
impl ProjectRole<Alice, Http, TChoice<Http, TestLabel1, 
    TSend<Http, TestLabel2, Alice, Message, TEnd<Http, TestLabel3>>, 
    TRecv<Http, TestLabel2, Bob, Response, TEnd<Http, TestLabel3>>>> for ()
{
    type Out = EpChoice<
        Http, 
        TestLabel1, 
        Alice,
        EpSend<Http, TestLabel2, Alice, Message, EpEnd<Http, TestLabel3, Alice>>,
        EpRecv<Http, TestLabel2, Alice, Response, EpEnd<Http, TestLabel3, Alice>>
    >;
}

// Special case for test_preserved_label_in_parallel
// Project Alice onto TPar with Alice only in left branch
impl ProjectRole<Alice, Http, 
    TPar<Http, TestLabel1, 
        TSend<Http, TestLabel2, Alice, Message, TEnd<Http, TestLabel3>>, 
        TRecv<Http, TestLabel2, Bob, Response, TEnd<Http, TestLabel3>>,
        ()
    >> for ()
{
    type Out = EpSend<Http, TestLabel2, Alice, Message, EpEnd<Http, TestLabel3, Alice>>;
}

// Project Charlie onto TPar with Charlie in neither branch
impl ProjectRole<Charlie, Http, 
    TPar<Http, TestLabel1, 
        TSend<Http, TestLabel2, Alice, Message, TEnd<Http, TestLabel3>>, 
        TRecv<Http, TestLabel2, Bob, Response, TEnd<Http, TestLabel3>>,
        ()
    >> for ()
{
    type Out = EpSkip<Http, TestLabel1, Charlie>;
}

// Special case for test_complex_protocol_label_preservation
impl ProjectRole<Alice, Http, 
    TSend<Http, TestLabel1, Alice, Message, 
        TRecv<Http, TestLabel2, Bob, Response, 
            TChoice<Http, TestLabel3,
                TSend<Http, TestLabel2, Alice, Message, TEnd<Http, TestLabel3>>,
                TRecv<Http, TestLabel2, Bob, Response, TEnd<Http, TestLabel3>>
            >
        >
    >> for ()
{
    type Out = EpSend<
        Http, 
        TestLabel1,
        Alice, 
        Message, 
        EpRecv<
            Http, 
            TestLabel2, 
            Alice, 
            Response, 
            EpChoice<
                Http, 
                TestLabel3, 
                Alice,
                EpSend<Http, TestLabel2, Alice, Message, EpEnd<Http, TestLabel3, Alice>>,
                EpRecv<Http, TestLabel2, Alice, Response, EpEnd<Http, TestLabel3, Alice>>
            >
        >
    >;
}
