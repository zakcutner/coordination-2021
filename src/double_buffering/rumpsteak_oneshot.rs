use super::{Copy, Ready};
use rumpsteak_oneshot::{End, Left, Receive, Right, Send, SessionPair};

pub type AdderSToK = Receive<Ready, Send<Copy, Receive<Ready, Send<Copy, End>>>>;
pub type AdderSQueue = Left<Left<Left<Left<End>>>>;

pub type AdderKToS = Send<Ready, Receive<Copy, Send<Ready, Receive<Copy, End>>>>;
pub type AdderKToT = Receive<Ready, Send<Copy, Receive<Ready, Send<Copy, End>>>>;
pub type AdderKQueue = Left<Left<Right<Right<Left<Left<Right<Right<End>>>>>>>>;

pub type AdderTToK = Send<Ready, Receive<Copy, Send<Ready, Receive<Copy, End>>>>;
pub type AdderTQueue = Right<Right<Right<Right<End>>>>;

pub async fn source(s: SessionPair<AdderSToK, End, AdderSQueue>) -> SessionPair<End, End, End> {
    let (x, y) = (1, 2);

    let (Ready, s) = s.receive().await;
    let s = s.send(Copy(x));

    let (Ready, s) = s.receive().await;
    s.send(Copy(y))
}

pub async fn kernel(
    s: SessionPair<AdderKToS, AdderKToT, AdderKQueue>,
) -> SessionPair<End, End, End> {
    let s = s.send(Ready);
    let (Copy(x), s) = s.receive().await;
    let (Ready, s) = s.receive().await;
    let s = s.send(Copy(x));

    let s = s.send(Ready);
    let (Copy(y), s) = s.receive().await;
    let (Ready, s) = s.receive().await;
    s.send(Copy(y))
}

pub async fn sink(s: SessionPair<End, AdderTToK, AdderTQueue>) -> SessionPair<End, End, End> {
    let s = s.send(Ready);
    let (Copy(x), s) = s.receive().await;

    let s = s.send(Ready);
    let (Copy(y), s) = s.receive().await;

    assert_eq!((x, y), (1, 2));
    s
}
