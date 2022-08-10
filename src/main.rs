#[macro_use]
extern crate serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!("https://{host}/client/get-treatments-with-config?key={key}&split-names={splits}",
        host = "split-evaluator.darkbloom.org",
	key = "dmartin",
        splits = "new_onboarding,multivariant_demo");
  
    let body = reqwest::get(&request_url)
        .await?
        .text()
        .await?;

    let obj = json!(body);
    println!("{}", serde_json::to_string_pretty(&obj).unwrap());

    Ok(())
}
