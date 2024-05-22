#![allow(unused)]
use anyhow::Result;
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
    let hc = httpc_test::new_client("http://localhost:9000")?; 
    let payload = json!({
        "username": "username",
        "password": "password",
        "is_login": false,
        "app_id": 1,
        "app_type": 1,
        "db": "db"
    });
    let res = hc.do_post("/web/components/navbar", payload).await?;
    res.print().await?;
    Ok(())
}

