use reqwest;

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    // chaining .await will yield our query result
    let client = reqwest::Client::new();
    let result = client
        .get("https://api.spotify.com/v1/search")
        .header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await;
    
    println!("{:?}", result);

}