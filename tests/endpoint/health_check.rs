#[actix_rt::test]
async fn health_check_works() {
    let client = reqwest::Client::new();

    let res = client
        .get("http://localhost:8000/utils/health")
        .send()
        .await
        .unwrap();

    assert!(res.status() == reqwest::StatusCode::OK);
}
