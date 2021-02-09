use super::{Add, Sum};
use rumpsteak_oneshot::{End, Left, Receive, Right, Send, SessionPair};

pub type AdderAToB = Send<Add, Receive<Add, End>>;
pub type AdderAToC = Send<Add, Receive<Sum, End>>;
pub type AdderAQueue = Left<Left<Right<Right<End>>>>;

pub type AdderBToA = Receive<Add, Send<Add, End>>;
pub type AdderBToC = Send<Add, Receive<Sum, End>>;
pub type AdderBQueue = Left<Left<Right<Right<End>>>>;

pub type AdderCToA = Receive<Add, Send<Sum, End>>;
pub type AdderCToB = Receive<Add, Send<Sum, End>>;
pub type AdderCQueue = Left<Right<Left<Right<End>>>>;

pub async fn adder_a(
    s: SessionPair<AdderAToB, AdderAToC, AdderAQueue>,
) -> SessionPair<End, End, End> {
    let x = 2;
    let s = s.send(Add(x));
    let (Add(y), s) = s.receive().await;
    let s = s.send(Add(y));
    let (Sum(z), s) = s.receive().await;
    assert_eq!(z, 5);
    s
}

pub async fn adder_b(
    s: SessionPair<AdderBToA, AdderBToC, AdderBQueue>,
) -> SessionPair<End, End, End> {
    let (Add(y), s) = s.receive().await;
    let x = 3;
    let s = s.send(Add(x));
    let s = s.send(Add(y));
    let (Sum(z), s) = s.receive().await;
    assert_eq!(z, 5);
    s
}

pub async fn adder_c(
    s: SessionPair<AdderCToA, AdderCToB, AdderCQueue>,
) -> SessionPair<End, End, End> {
    let (Add(x), s) = s.receive().await;
    let (Add(y), s) = s.receive().await;
    let z = x + y;
    let s = s.send(Sum(z));
    s.send(Sum(z))
}
