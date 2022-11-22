use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    aws_api_key: String,
    aws_api_secret: String,
    confluent_bootstrap_server: String,
    confluent_api_key: String,
    confluent_api_secret: String,
}

pub fn load_config() -> Config {
    dotenv().ok();

    let aws_api_key = std::env::var("AWS_API_KEY")
        .expect("AWS_API_KEY must be set."); 
    let aws_api_secret = std::env::var("AWS_API_SECRET")
        .expect("AWS_API_SECRET must be set."); 
    let confluent_bootstrap_server = std::env::var("CONFLUENT_BOOTSTRAP_SERVER")
        .expect("CONFLUENT_BOOTSTRAP_SERVER must be set."); 
    let confluent_api_key = std::env::var("CONFLUENT_API_KEY")
        .expect("CONFLUENT_API_KEY must be set."); 
    let confluent_api_secret = std::env::var("CONFLUENT_API_SECRET")
        .expect("CONFLUENT_API_SECRET must be set."); 

    Config {
        aws_api_key,
        aws_api_secret,
        confluent_bootstrap_server,
        confluent_api_key,
        confluent_api_secret,
    }
}