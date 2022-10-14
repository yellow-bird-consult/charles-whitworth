use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

mod actors;
use actors::state_actor::StateActor;
use actors::lib_runner::LibRunnerActor;
use actors::messages::{MessageType, StateActorMessage};


async fn handle(_req: Request<Body>, test: &str) -> Result<Response<Body>, Infallible> {
    println!("incoming message from the outside");
    Ok(Response::new(format!("Hello, World {}", test).into()))
}


#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let static_test = "test";

    let (state_tx, state_rx) = mpsc::channel::<StateActorMessage>(1);
    let (runner_tx, runner_rx) = mpsc::channel::<StateActorMessage>(1);
    // let tracker_tx_one = tracker_tx.clone();

    tokio::spawn(async move {
        let state_actor = StateActor::new(state_rx, runner_tx);
        state_actor.run().await;
    });
    tokio::spawn(async move {
        let lib_runner_actor = LibRunnerActor::new(runner_rx, state_tx, 3);
        lib_runner_actor.run().await;
    });

    let server = Server::bind(&addr).serve(make_service_fn(move |_conn| {
        async move { Ok::<_, Infallible>(service_fn(move |req| handle(req, static_test))) }
    }));

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
