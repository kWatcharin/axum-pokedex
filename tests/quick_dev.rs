#![allow(unused)]
use anyhow::{Ok, Result};
use serde_json::{json, Value};


#[tokio::test]
async fn test_quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9000")?;
    let res = hc.do_get("/users/tests/get").await?;
    let status = res.status();

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

