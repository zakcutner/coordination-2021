use super::{Add, Sum};
use mpstthree::{
    binary::struct_trait::{End, Recv, Send},
    functionmpst::{
        close::close_mpst,
        recv::{
            recv_mpst_a_from_b, recv_mpst_a_from_c, recv_mpst_b_from_a, recv_mpst_b_from_c,
            recv_mpst_c_from_a, recv_mpst_c_from_b,
        },
        send::{
            send_mpst_a_to_b, send_mpst_a_to_c, send_mpst_b_to_a, send_mpst_b_to_c,
            send_mpst_c_to_a, send_mpst_c_to_b,
        },
    },
    role::{a::RoleA, b::RoleB, c::RoleC, end::RoleEnd},
    sessionmpst::SessionMpst,
};
use std::{error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

pub type AtoB = Send<Add, Recv<Add, End>>;
pub type AtoC = Send<Add, Recv<Sum, End>>;
pub type QueueA = RoleB<RoleB<RoleC<RoleC<RoleEnd>>>>;
pub type EndpointA = SessionMpst<AtoB, AtoC, QueueA, RoleA<RoleEnd>>;

pub type BtoA = Recv<Add, Send<Add, End>>;
pub type BtoC = Send<Add, Recv<Sum, End>>;
pub type QueueB = RoleA<RoleA<RoleC<RoleC<RoleEnd>>>>;
pub type EndpointB = SessionMpst<BtoA, BtoC, QueueB, RoleB<RoleEnd>>;

pub type CtoA = Recv<Add, Send<Sum, End>>;
pub type CtoB = Recv<Add, Send<Sum, End>>;
pub type QueueC = RoleA<RoleB<RoleA<RoleB<RoleEnd>>>>;
pub type EndpointC = SessionMpst<CtoA, CtoB, QueueC, RoleC<RoleEnd>>;

pub fn adder_a(s: EndpointA) -> Result<()> {
    let x = 2;
    let s = send_mpst_a_to_b(Add(x), s);
    let (Add(y), s) = recv_mpst_a_from_b(s)?;
    let s = send_mpst_a_to_c(Add(y), s);
    let (Sum(z), s) = recv_mpst_a_from_c(s)?;
    assert_eq!(z, 5);
    close_mpst(s)?;
    Ok(())
}

pub fn adder_b(s: EndpointB) -> Result<()> {
    let (Add(y), s) = recv_mpst_b_from_a(s)?;
    let x = 3;
    let s = send_mpst_b_to_a(Add(x), s);
    let s = send_mpst_b_to_c(Add(y), s);
    let (Sum(z), s) = recv_mpst_b_from_c(s)?;
    assert_eq!(z, 5);
    close_mpst(s)?;
    Ok(())
}

pub fn adder_c(s: EndpointC) -> Result<()> {
    let (Add(x), s) = recv_mpst_c_from_a(s)?;
    let (Add(y), s) = recv_mpst_c_from_b(s)?;
    let z = x + y;
    let s = send_mpst_c_to_a(Sum(z), s);
    let s = send_mpst_c_to_b(Sum(z), s);
    close_mpst(s)?;
    Ok(())
}
