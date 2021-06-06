use super::Value;
use mpstthree::{
    binary::struct_trait::{End, Recv, Send},
    functionmpst::{
        close::close_mpst,
        recv::{recv_mpst_a_from_c, recv_mpst_b_from_a, recv_mpst_c_from_b},
        send::{send_mpst_a_to_b, send_mpst_b_to_c, send_mpst_c_to_a},
    },
    role::{a::RoleA, b::RoleB, c::RoleC, end::RoleEnd},
    sessionmpst::SessionMpst,
};
use std::{error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

pub type AtoB = Send<Value, End>;
pub type AtoC = Recv<Value, End>;
pub type QueueA = RoleB<RoleC<RoleEnd>>;
pub type EndpointA = SessionMpst<AtoB, AtoC, QueueA, RoleA<RoleEnd>>;

pub type BtoA = Recv<Value, End>;
pub type BtoC = Send<Value, End>;
pub type QueueB = RoleA<RoleC<RoleEnd>>;
pub type EndpointB = SessionMpst<BtoA, BtoC, QueueB, RoleB<RoleEnd>>;

pub type CtoA = Send<Value, End>;
pub type CtoB = Recv<Value, End>;
pub type QueueC = RoleB<RoleA<RoleEnd>>;
pub type EndpointC = SessionMpst<CtoA, CtoB, QueueC, RoleC<RoleEnd>>;

pub fn ring_a(s: EndpointA) -> Result<()> {
    let x = 2;
    let s = send_mpst_a_to_b(Value(x), s);
    let (Value(y), s) = recv_mpst_a_from_c(s)?;
    assert_eq!(y, 4);
    close_mpst(s)?;
    Ok(())
}

pub fn ring_b(s: EndpointB) -> Result<()> {
    let x = 3;
    let (Value(y), s) = recv_mpst_b_from_a(s)?;
    let s = send_mpst_b_to_c(Value(x), s);
    assert_eq!(y, 2);
    close_mpst(s)?;
    Ok(())
}

pub fn ring_c(s: EndpointC) -> Result<()> {
    let x = 4;
    let (Value(y), s) = recv_mpst_c_from_b(s)?;
    let s = send_mpst_c_to_a(Value(x), s);
    assert_eq!(y, 3);
    close_mpst(s)?;
    Ok(())
}
