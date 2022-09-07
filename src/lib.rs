pub mod driver;
pub mod error;
pub mod process;
pub mod structure;

#[cfg(test)]
mod tests {
    use crate::{structure::Vertex, *};
    use serde_json::to_string_pretty;
    use std::env;
    use tokio::time::{timeout, Duration};

    #[test]
    fn test_traversal_serialization() {
        let mut g = process::traversal::Traversal::new();
        let mut g2 = process::traversal::Traversal::new();

        g.V(&["USER_ID"])
            .addE(&["edge_label"])
            .to(&[g2.V(&["ANOTHER_USER_ID"]).to_owned()])
            .id();

        let bt: process::traversal::Bytecode = g.into();
        let json = to_string_pretty(&bt).unwrap();
        println!("{}", json);
    }

    #[tokio::test]
    async fn integration_test() {
        if let Ok(test_url) = env::var("TEST_URL") {
            let client = driver::Client::new(&test_url).await.unwrap();
            let mut g = process::traversal::Traversal::new();

            println!("testing query execution...");
            let result = timeout(
                Duration::from_secs(5),
                g.V::<String>(&[]).sample(&[1]).execute(&client),
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
