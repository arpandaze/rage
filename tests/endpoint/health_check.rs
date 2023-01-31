use crate::helpers::spawn_app;

#[actix_rt::test]
async fn health_check_works() {
    let base_address = spawn_app().await;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{base_address}/utils/health"))
        .send()
        .await
        .unwrap();

    assert!(res.status() == reqwest::StatusCode::OK);
}
