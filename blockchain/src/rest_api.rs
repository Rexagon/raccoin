use serde::Deserialize;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use warp::Filter;

use crate::block::{Block, BlockData};
use crate::blockchain::BlockChain;
use crate::peers_network::{Peer, PeersNetwork};
use crate::state::State;

pub async fn serve(addr: SocketAddr, shared_state: State) {
    let options = warp::options().and(warp::header::<String>("Origin").map(|origin| {
        Ok(warp::http::Response::builder()
            .header("access-control-allow-methods", "HEAD, GET")
            .header("access-control-allow-headers", "Authorization")
            .header("access-control-allow-credentials", "true")
            .header("access-control-allow-max-age", "300")
            .header("access-control-allow-origin", origin)
            .header("vary", "Origin")
            .body(""))
    }));

    let routes = create_all_route_handlers(&shared_state);

    let routes = warp::any()
        .and(options)
        .or(routes)
        .with(warp::reply::with::header(
            "access-control-allow-headers",
            "Content-Type, Accept",
        ))
        .with(warp::reply::with::header(
            "access-control-allow-origin",
            "*",
        ))
        .with(warp::log("info"));

    warp::serve(routes).run(addr).await;
}

fn create_all_route_handlers(
    data: &State,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_blocks(data.blockchain.clone())
        .or(create_block(data.blockchain.clone()))
        .or(get_peers(data.peers.clone()))
        .or(add_peer(data.peers.clone()))
}

fn get_blocks(
    data: Arc<Mutex<BlockChain>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("blocks")
        .and(warp::get())
        .map(move || warp::reply::json(&data.lock().unwrap().blocks))
}

fn create_block(
    data: Arc<Mutex<BlockChain>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("blocks")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: BlockData| {
            let mut blockchain = data.lock().unwrap();

            let block = Block::from_previous(blockchain.last(), body);
            let response = warp::reply::json(&block);
            blockchain.add_block(block);

            response
        })
}

fn get_peers(
    data: Arc<Mutex<PeersNetwork>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("peers").and(warp::get()).map(move || {
        let network = data.lock().unwrap();

        warp::reply::json(&network.peers())
    })
}

fn add_peer(
    data: Arc<Mutex<PeersNetwork>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("peers")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |body: AddPeerRequest| {
            let data = data.clone();

            async move {
                let peer = match Peer::try_create(body.addr).await {
                    Ok(p) => p,
                    Err(_) => return Err(warp::reject::custom(PeerConnectionError)),
                };

                let mut network = data.lock().unwrap();
                network.add(peer);

                Ok(warp::reply::with_status("", warp::http::StatusCode::OK))
            }
        })
}

#[derive(Debug, Deserialize)]
struct AddPeerRequest {
    addr: SocketAddr,
}

#[derive(Debug)]
struct PeerConnectionError;

impl warp::reject::Reject for PeerConnectionError {}
