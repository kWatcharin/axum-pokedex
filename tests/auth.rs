use anyhow::{Ok, Result};
use serde_json::json;

fn root_url() -> String { 
    String::from("http://localhost:9000") 
}


// => /auth/loging/validate 
#[tokio::test]
async fn login_validate() -> Result<()> {
    let payload = json!({
        "username": "demo1",
        "password": "welcome"
    });

    httpc_test::new_client(root_url().as_str())?
        .do_post("/auth/login/validate", payload)
        .await?
        .print()
        .await?;

    Ok(())
}
