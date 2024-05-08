#[tokio::main]
pub async fn main() {
    let _ = msa::flow::do_auth().await.unwrap();
}
