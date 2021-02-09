use super::Value;
use crate::sleep;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use rumpsteak::{session, try_session, End, Message, Receive, Role, Roles, Send};
use std::{error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

type Sender = UnboundedSender<Label>;
type Receiver = UnboundedReceiver<Label>;

#[derive(Roles)]
pub struct Roles(pub A, pub B, pub C);

#[derive(Role)]
#[message(Label)]
pub struct A(#[route(B)] Sender, #[route(C)] Receiver);

#[derive(Role)]
#[message(Label)]
pub struct B(#[route(A)] Receiver, #[route(C)] Sender);

#[derive(Role)]
#[message(Label)]
pub struct C(#[route(A)] Sender, #[route(B)] Receiver);

#[derive(Message)]
pub enum Label {
    Value(Value),
}

#[session]
pub type RingA = Send<B, Value, Receive<C, Value, End>>;

#[session]
pub type RingB = Receive<A, Value, Send<C, Value, End>>;

#[session]
pub type RingBOptimized = Send<C, Value, Receive<A, Value, End>>;

#[session]
pub type RingC = Receive<B, Value, Send<A, Value, End>>;

#[session]
pub type RingCOptimized = Send<A, Value, Receive<B, Value, End>>;

pub async fn ring_a<const SLEEP: bool>(role: &mut A) -> Result<()> {
    let x = 2;
    try_session(role, |s: RingA<'_, _>| async {
        let s = s.send(Value(x)).await?;
        let (Value(y), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        assert_eq!(y, 4);
        Ok(((), s))
    })
    .await
}

pub async fn ring_b<const SLEEP: bool>(role: &mut B) -> Result<()> {
    let x = 3;
    try_session(role, |s: RingB<'_, _>| async {
        let (Value(y), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let s = s.send(Value(x)).await?;
        assert_eq!(y, 2);
        Ok(((), s))
    })
    .await
}

pub async fn ring_b_optimized<const SLEEP: bool>(role: &mut B) -> Result<()> {
    let x = 3;
    try_session(role, |s: RingBOptimized<'_, _>| async {
        let s = s.send(Value(x)).await?;
        let (Value(y), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        assert_eq!(y, 2);
        Ok(((), s))
    })
    .await
}

pub async fn ring_c<const SLEEP: bool>(role: &mut C) -> Result<()> {
    let x = 4;
    try_session(role, |s: RingC<'_, _>| async {
        let (Value(y), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let s = s.send(Value(x)).await?;
        assert_eq!(y, 3);
        Ok(((), s))
    })
    .await
}

pub async fn ring_c_optimized<const SLEEP: bool>(role: &mut C) -> Result<()> {
    let x = 4;
    try_session(role, |s: RingCOptimized<'_, _>| async {
        let s = s.send(Value(x)).await?;
        let (Value(y), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        assert_eq!(y, 3);
        Ok(((), s))
    })
    .await
}
