use anyhow::{Ok, Result};
use serde_json::json;


#[tokio::test]
async fn test_quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9000")?;
    let res = hc.do_get("/users/tests/get").await?;
    res.print().await?;
    Ok(())
}


#[tokio::test] 
async fn test_users_post() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9000")?; 
    let payload = json!({ "name": "test2" });
    let res = hc.do_post("/users/tests/post", payload).await?;
    res.print().await?;
    Ok(())
}


#[tokio::test]
async fn test_fetch_navbar() -> Result<()> {
    let payload = json!({
        "username": "username",
        "password": "password",
        "is_login": false,
        "app_id": 1,
        "app_type": 1,
        "db": "db"
    });

    httpc_test::new_client("http://localhost:9000")?
        .do_post("/web/components/navbar", payload)
        .await?
        .print()
        .await?;
    Ok(())
}


#[tokio::test]
async fn fetch_pokeapi() -> Result<()> {
    let pokemon = "ditto";

    httpc_test::new_client("https://pokeapi.co/api/v2/pokemon")?
        .do_get(format!("/{pokemon}").as_str())
        .await?
        .print()
        .await?;

    Ok(())
}