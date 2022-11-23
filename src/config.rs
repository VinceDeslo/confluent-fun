use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_region: String,
    aws_rds_connection: String,
    confluent_bootstrap_server: String,
    confluent_api_key: String,
    confluent_api_secret: String,
}

pub fn load_config() -> Config {
    dotenv().ok();

    let aws_access_key_id = std::env::var("AWS_ACCESS_KEY_ID")
        .expect("AWS_ACCESS_KEY_ID must be set."); 
    let aws_secret_access_key = std::env::var("AWS_SECRET_ACCESS_KEY")
        .expect("AWS_SECRET_ACCESS_KEY must be set."); 
    let aws_region = std::env::var("AWS_REGION")
        .expect("AWS_REGION must be set.");
    let aws_rds_connection = std::env::var("AWS_RDS_CONNECTION")
        .expect("AWS_RDS_CONNECTION must be set.");
    let confluent_bootstrap_server = std::env::var("CONFLUENT_BOOTSTRAP_SERVER")
        .expect("CONFLUENT_BOOTSTRAP_SERVER must be set."); 
    let confluent_api_key = std::env::var("CONFLUENT_API_KEY")
        .expect("CONFLUENT_API_KEY must be set."); 
    let confluent_api_secret = std::env::var("CONFLUENT_API_SECRET")
        .expect("CONFLUENT_API_SECRET must be set."); 

    Config {
        aws_access_key_id,
        aws_secret_access_key,
        aws_region,
        aws_rds_connection,
        confluent_bootstrap_server,
        confluent_api_key,
        confluent_api_secret,
    }
}