# C02 API Basic Rust Implementation

DONE:
- Basic API implementation
- Client Format 
- Two C02 request implementations. 

TODO: 
- Rate Limitation
- Support of future Electricity Maps commercial API



Example: 

```rust
use rust_c02_signal;

async fn do_stuff() -> () {
    let c02client = rust_c02_signal::Client::new("API-KEY".to_string());

    // Get the latest data for a country code
    let resp = c02client.latest_cc("GB".to_string()).await;

    println!("{:?}", resp);

    // Get the latest data for a set of long/lat coords
    let resp = c02client.latest_gc(51.5074, 0.1278).await;

    println!("{:?}", resp);
}
```