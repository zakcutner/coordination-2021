use super::{Copy, Ready};
use crate::sleep;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use rumpsteak::{
    channel::{Bidirectional, Nil},
    session, try_session, End, Message, Receive, Role, Roles, Send,
};
use std::{error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

type Channel = Bidirectional<UnboundedSender<Label>, UnboundedReceiver<Label>>;

#[derive(Roles)]
pub struct Roles(pub S, pub K, pub T);

#[derive(Role)]
#[message(Label)]
pub struct S(#[route(K)] Channel, #[route(T)] Nil);

#[derive(Role)]
#[message(Label)]
pub struct K(#[route(S)] Channel, #[route(T)] Channel);

#[derive(Role)]
#[message(Label)]
pub struct T(#[route(S)] Nil, #[route(K)] Channel);

#[derive(Message)]
pub enum Label {
    Ready(Ready),
    Copy(Copy),
}

#[session]
pub type Source = Receive<K, Ready, Send<K, Copy, Receive<K, Ready, Send<K, Copy, End>>>>;

#[session]
#[rustfmt::skip]
pub type Kernel = Send<S, Ready, Receive<S, Copy, Receive<T, Ready, Send<T, Copy, Send<S, Ready, Receive<S, Copy, Receive<T, Ready, Send<T, Copy, End>>>>>>>>;

#[session]
#[rustfmt::skip]
pub type KernelOptimizedWeak = Send<S, Ready, Receive<S, Copy, Send<S, Ready, Receive<T, Ready, Send<T, Copy, Receive<S, Copy, Receive<T, Ready, Send<T, Copy, End>>>>>>>>;

#[session]
#[rustfmt::skip]
pub type KernelOptimized = Send<S, Ready, Send<S, Ready, Receive<S, Copy, Receive<T, Ready, Send<T, Copy, Receive<S, Copy, Receive<T, Ready, Send<T, Copy, End>>>>>>>>;

#[session]
pub type Sink = Send<K, Ready, Receive<K, Copy, Send<K, Ready, Receive<K, Copy, End>>>>;

pub async fn source<const SLEEP: bool>(role: &mut S) -> Result<()> {
    let (x, y) = (1, 2);
    try_session(role, |s: Source<'_, _>| async {
        let (Ready, s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let s = s.send(Copy(x)).await?;

        let (Ready, s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let s = s.send(Copy(y)).await?;

        Ok(((), s))
    })
    .await
}

pub async fn kernel<const SLEEP: bool>(role: &mut K) -> Result<()> {
    try_session(role, |s: Kernel<'_, _>| async {
        let s = s.send(Ready).await?;
        let (Copy(x), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let (Ready, s) = s.receive().await?;
        let s = s.send(Copy(x)).await?;

        let s = s.send(Ready).await?;
        let (Copy(y), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let (Ready, s) = s.receive().await?;
        let s = s.send(Copy(y)).await?;

        Ok(((), s))
    })
    .await
}

pub async fn kernel_optimized_weak<const SLEEP: bool>(role: &mut K) -> Result<()> {
    try_session(role, |s: KernelOptimizedWeak<'_, _>| async {
        let s = s.send(Ready).await?;
        let (Copy(x), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let s = s.send(Ready).await?;
        let (Ready, s) = s.receive().await?;
        let s = s.send(Copy(x)).await?;

        let (Copy(y), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let (Ready, s) = s.receive().await?;
        let s = s.send(Copy(y)).await?;

        Ok(((), s))
    })
    .await
}

pub async fn kernel_optimized<const SLEEP: bool>(role: &mut K) -> Result<()> {
    try_session(role, |s: KernelOptimized<'_, _>| async {
        let s = s.send(Ready).await?;
        let s = s.send(Ready).await?;

        let (Copy(x), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let (Ready, s) = s.receive().await?;
        let s = s.send(Copy(x)).await?;

        let (Copy(y), s) = s.receive().await?;
        sleep::<SLEEP>().await;
        let (Ready, s) = s.receive().await?;
        let s = s.send(Copy(y)).await?;

        Ok(((), s))
    })
    .await
}

pub async fn sink<const SLEEP: bool>(role: &mut T) -> Result<()> {
    try_session(role, |s: Sink<'_, _>| async {
        let s = s.send(Ready).await?;
        let (Copy(x), s) = s.receive().await?;

        let s = s.send(Ready).await?;
        let (Copy(y), s) = s.receive().await?;

        assert_eq!((x, y), (1, 2));
        Ok(((), s))
    })
    .await
}
