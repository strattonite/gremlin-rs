#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod driver;
pub mod process;
pub mod structure;

#[cfg(test)]
mod tests {
    use super::*;
    use process::*;
    use std::{env, time::Duration};
    use structure::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn client_integration() {
        if let Ok(test_url) = env::var("TEST_URL") {
            let client = driver::Client::new(&test_url, 1000).await.unwrap();

            println!("testing query execution...");
            let result = timeout(
                Duration::from_secs(5),
                g.V(()).sample((1,)).to_list(&client),
            )
            .await
            .unwrap()
            .unwrap();

            println!("{:?}", &result);

            println!("testing response data parsing");
            let v: Vec<Vertex> = result.parse().unwrap();
            println!("{:?}", &v);
        } else {
            println!("integration test not run, missing TEST_URL env var")
        }
    }
}
