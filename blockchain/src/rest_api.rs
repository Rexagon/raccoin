use serde::{de::DeserializeOwned, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use warp::Filter;

use crate::block::Block;
use crate::blockchain::BlockChain;

type SharedData<T> = Arc<Mutex<BlockChain<T>>>;

pub async fn serve<T, D>(addr: T, shared_data: SharedData<D>)
where
    T: Into<SocketAddr> + 'static,
    D: Serialize + DeserializeOwned + Sync + Send + Clone + 'static,
{
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

    let routes = create_all_route_handlers(&shared_data);

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

fn create_all_route_handlers<D>(
    data: &SharedData<D>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    D: Serialize + DeserializeOwned + Sync + Send + Clone + 'static,
{
    warp::path("blocks").and(get_blocks(data.clone()).or(create_block(data.clone())))
}

fn get_blocks<D>(
    data: SharedData<D>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    D: Serialize + DeserializeOwned + Sync + Send + Clone + 'static,
{
    warp::get().map(move || warp::reply::json(&data.lock().unwrap().blocks))
}

fn create_block<D>(
    data: SharedData<D>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    D: Serialize + DeserializeOwned + Sync + Send + Clone + 'static,
{
    warp::post().and(warp::body::json()).map(move |body: D| {
        let mut blockchain = data.lock().unwrap();

        let block = Block::from_previous(blockchain.last(), body);
        let response = warp::reply::json(&block);
        blockchain.add_block(block);

        response
    })
}
