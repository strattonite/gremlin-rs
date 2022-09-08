#![allow(non_snake_case)]

pub mod driver;
pub mod error;
pub mod process;
pub mod structure;

#[cfg(test)]
mod tests {
    use crate::{process::traversal::Bytecode, structure::Vertex, *};
    use serde_json::to_string_pretty;
    use std::env;
    use tokio::time::{timeout, Duration};

    #[test]
    fn test_traversal_serialization() {
        let g = process::traversal::Traversal::new();
        let mut __ = process::traversal::Traversal::new();
        let bt: Bytecode = g
            .V(())
            .addE("user")
            .to(__.V(()).hasLabel(("user", "workout")))
            .into();
        let json = to_string_pretty(&bt).unwrap();
        println!("{}", json);
    }

    #[tokio::test]
    async fn integration_test() {
        if let Ok(test_url) = env::var("TEST_URL") {
            let client = driver::Client::new(&test_url, 1000).await.unwrap();
            let g = process::traversal::Traversal::new();

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
