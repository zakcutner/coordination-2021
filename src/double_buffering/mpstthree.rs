use super::{Copy, Ready};
use mpstthree::{
    binary::{End, Recv, Send},
    functionmpst::{
        close::close_mpst,
        recv::{recv_mpst_a_to_b, recv_mpst_b_to_a, recv_mpst_b_to_c, recv_mpst_c_to_b},
        send::{send_mpst_a_to_b, send_mpst_b_to_a, send_mpst_b_to_c, send_mpst_c_to_b},
    },
    role::{a::RoleA, b::RoleB, c::RoleC, end::RoleEnd},
    sessionmpst::SessionMpst,
};
use std::{error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

pub type AtoB = Recv<Ready, Send<Copy, Recv<Ready, Send<Copy, End>>>>;
pub type AtoC = End;
pub type QueueA = RoleB<RoleB<RoleB<RoleB<RoleEnd>>>>;
pub type EndpointA = SessionMpst<AtoB, AtoC, QueueA, RoleA<RoleEnd>>;

pub type BtoA = Send<Ready, Recv<Copy, Send<Ready, Recv<Copy, End>>>>;
pub type BtoC = Recv<Ready, Send<Copy, Recv<Ready, Send<Copy, End>>>>;
pub type QueueB = RoleA<RoleA<RoleC<RoleC<RoleA<RoleA<RoleC<RoleC<RoleEnd>>>>>>>>;
pub type EndpointB = SessionMpst<BtoA, BtoC, QueueB, RoleB<RoleEnd>>;

pub type CtoA = End;
pub type CtoB = Send<Ready, Recv<Copy, Send<Ready, Recv<Copy, End>>>>;
pub type QueueC = RoleB<RoleB<RoleB<RoleB<RoleEnd>>>>;
pub type EndpointC = SessionMpst<CtoA, CtoB, QueueC, RoleC<RoleEnd>>;

pub fn source(s: EndpointA) -> Result<()> {
    let (x, y) = (1, 2);

    let (Ready, s) = recv_mpst_a_to_b(s)?;
    let s = send_mpst_a_to_b(Copy(x), s);

    let (Ready, s) = recv_mpst_a_to_b(s)?;
    let s = send_mpst_a_to_b(Copy(y), s);

    close_mpst(s)
}

pub fn kernel(s: EndpointB) -> Result<()> {
    let s = send_mpst_b_to_a(Ready, s);
    let (Copy(x), s) = recv_mpst_b_to_a(s)?;
    let (Ready, s) = recv_mpst_b_to_c(s)?;
    let s = send_mpst_b_to_c(Copy(x), s);

    let s = send_mpst_b_to_a(Ready, s);
    let (Copy(y), s) = recv_mpst_b_to_a(s)?;
    let (Ready, s) = recv_mpst_b_to_c(s)?;
    let s = send_mpst_b_to_c(Copy(y), s);

    close_mpst(s)
}

pub fn sink(s: EndpointC) -> Result<()> {
    let s = send_mpst_c_to_b(Ready, s);
    let (Copy(x), s) = recv_mpst_c_to_b(s)?;

    let s = send_mpst_c_to_b(Ready, s);
    let (Copy(y), s) = recv_mpst_c_to_b(s)?;

    assert_eq!((x, y), (1, 2));
    close_mpst(s)
}
