mod block;
mod blockchain;
mod hash;
mod rest_api;
mod rpc;
mod settings;
mod state;

use crate::settings::Settings;
use crate::state::State;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let settings = Settings::new().expect("Unable to parse settings.json");

    let shared_state = State::new(&settings);

    tokio::spawn(rpc::serve(settings.api.rpc, shared_state.clone()));

    rest_api::serve(settings.api.rest, shared_state).await;

    Ok(())
}
