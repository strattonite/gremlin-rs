#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod driver;
pub mod process;
pub mod structure;

#[cfg(test)]
mod tests {
    use super::*;
    use process::*;
    use serde_json::to_string_pretty;
    use std::{collections::HashMap, env, time::Duration};
    use structure::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn client_integration() {
        if let Ok(test_url) = env::var("TEST_URL") {
            let client = timeout(Duration::from_secs(5), driver::Client::new(&test_url, 1000))
                .await
                .unwrap()
                .unwrap();
            println!("created remote client");
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
            let v: Vec<gson::GsonV2> = result.parse().unwrap();
            println!("{}", to_string_pretty(&v).unwrap());

            println!("testing nested query");

            let result = timeout(
                Duration::from_secs(5),
                g.V(())
                    .sample((25,))
                    .group(())
                    .by(__.label())
                    .to_list(&client),
            )
            .await
            .unwrap()
            .unwrap();

            println!("testing nested response parsing");
            let v: HashMap<String, Vec<gson::GsonV2>> = result.parse().unwrap().remove(0);
            for (k, v) in v.iter() {
                println!("{}:\n{}", k, to_string_pretty(&v).unwrap());
            }

            let result = timeout(
                Duration::from_secs(5),
                g.V(())
                    .sample((25,))
                    .group(())
                    .by(__.label())
                    .by(__.propertyMap(()))
                    .to_list(&client),
            )
            .await
            .unwrap()
            .unwrap();

            println!("testing double nested response parsing");
            let v: HashMap<String, HashMap<String, Vec<gson::GsonV2>>> =
                result.parse().unwrap().remove(0);
            for (k, v) in v.iter() {
                println!("{}:\n{}", k, to_string_pretty(&v).unwrap());
            }

            let result = timeout(
                Duration::from_secs(5),
                g.V(())
                    .sample((25,))
                    .group(())
                    .by(__.label())
                    .by(__.propertyMap(()))
                    .to_list(&client),
            )
            .await
            .unwrap()
            .unwrap();

            println!("testing generic response parsing");

            let v: gson::GsonV2 = result.parse().unwrap().remove(0);
            println!("{:?}", v);
        } else {
            println!("integration test not run, missing TEST_URL env var")
        }
    }
}
