use std::{error::Error, time::Duration};

use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);
    let addr = "127.0.0.1:13265".parse()?;
    let mut server = TcpListener::bind(addr)?;
    let mut client = TcpStream::connect(addr)?;

    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;
    poll.registry()
        .register(&mut client, CLIENT, Interest::WRITABLE | Interest::READABLE)?;
    loop {
        poll.poll(&mut events, None)?;
        println!("Event");
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let connection = server.accept()?;
                    drop(connection);
                }
                CLIENT => {
                    if event.is_writable() {
                        // We can (likely) write to the socket without blocking.
                    }

                    if event.is_readable() {
                        // We can (likely) read from the socket without blocking.
                    }

                    // Since the server just shuts down the connection, let's
                    // just exit from our event loop.
                    return Ok(());
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
}

fn main1() -> Result<(), Box<dyn Error>> {
    let mut events = Events::with_capacity(1024);
    let mut poll = Poll::new()?;

    // Register `event::Source`s with `poll`.

    poll.poll(&mut events, Some(Duration::from_millis(100)))?;

    for event in events.iter() {
        println!("Got an event for {:?}", event.token());
    }
    Ok(())
}
