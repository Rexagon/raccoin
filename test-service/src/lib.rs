#[tarpc::service]
pub trait WorldService {
    async fn hello(name: String) -> String;
}
