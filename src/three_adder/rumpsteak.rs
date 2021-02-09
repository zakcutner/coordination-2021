use super::{Add, Sum};
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use rumpsteak::{
    channel::Bidirectional, session, try_session, End, Message, Receive, Role, Roles, Send,
};
use std::{error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

type Channel = Bidirectional<UnboundedSender<Label>, UnboundedReceiver<Label>>;

#[derive(Roles)]
pub struct Roles(pub A, pub B, pub C);

#[derive(Role)]
#[message(Label)]
pub struct A(#[route(B)] Channel, #[route(C)] Channel);

#[derive(Role)]
#[message(Label)]
pub struct B(#[route(A)] Channel, #[route(C)] Channel);

#[derive(Role)]
#[message(Label)]
pub struct C(#[route(A)] Channel, #[route(B)] Channel);

#[derive(Message)]
pub enum Label {
    Add(Add),
    Sum(Sum),
}

#[session]
pub type AdderA = Send<B, Add, Receive<B, Add, Send<C, Add, Receive<C, Sum, End>>>>;

#[session]
pub type AdderB = Receive<A, Add, Send<A, Add, Send<C, Add, Receive<C, Sum, End>>>>;

#[session]
pub type AdderC = Receive<A, Add, Receive<B, Add, Send<A, Sum, Send<B, Sum, End>>>>;

pub async fn adder_a(role: &mut A) -> Result<()> {
    let x = 2;
    try_session(role, |s: AdderA<'_, _>| async {
        let s = s.send(Add(x)).await?;
        let (Add(y), s) = s.receive().await?;
        let s = s.send(Add(y)).await?;
        let (Sum(z), s) = s.receive().await?;
        assert_eq!(z, 5);
        Ok(((), s))
    })
    .await
}

pub async fn adder_b(role: &mut B) -> Result<()> {
    try_session(role, |s: AdderB<'_, _>| async {
        let (Add(y), s) = s.receive().await?;
        let x = 3;
        let s = s.send(Add(x)).await?;
        let s = s.send(Add(y)).await?;
        let (Sum(z), s) = s.receive().await?;
        assert_eq!(z, 5);
        Ok(((), s))
    })
    .await
}

pub async fn adder_c(role: &mut C) -> Result<()> {
    try_session(role, |s: AdderC<'_, _>| async {
        let (Add(x), s) = s.receive().await?;
        let (Add(y), s) = s.receive().await?;
        let z = x + y;
        let s = s.send(Sum(z)).await?;
        Ok(((), s.send(Sum(z)).await?))
    })
    .await
}
