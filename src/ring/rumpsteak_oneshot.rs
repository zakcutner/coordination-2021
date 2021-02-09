use super::Value;
use rumpsteak_oneshot::{End, Left, Receive, Right, Send, SessionPair};

pub type RingAToB = Send<Value, End>;
pub type RingAToC = Receive<Value, End>;
pub type RingAQueue = Left<Right<End>>;

pub type RingBToA = Receive<Value, End>;
pub type RingBToC = Send<Value, End>;
pub type RingBQueue = Left<Right<End>>;

pub type RingCToA = Send<Value, End>;
pub type RingCToB = Receive<Value, End>;
pub type RingCQueue = Right<Left<End>>;

pub async fn ring_a(s: SessionPair<RingAToB, RingAToC, RingAQueue>) -> SessionPair<End, End, End> {
    let x = 2;
    let s = s.send(Value(x));
    let (Value(y), s) = s.receive().await;
    assert_eq!(y, 4);
    s
}

pub async fn ring_b(s: SessionPair<RingBToA, RingBToC, RingBQueue>) -> SessionPair<End, End, End> {
    let x = 3;
    let (Value(y), s) = s.receive().await;
    let s = s.send(Value(x));
    assert_eq!(y, 2);
    s
}

pub async fn ring_c(s: SessionPair<RingCToA, RingCToB, RingCQueue>) -> SessionPair<End, End, End> {
    let x = 4;
    let (Value(y), s) = s.receive().await;
    let s = s.send(Value(x));
    assert_eq!(y, 3);
    s
}
