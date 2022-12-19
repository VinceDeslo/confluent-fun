use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    aws_access_key_id: String,
    aws_secret_access_key: String,
    aws_region: String,
    pub database_protocol: String,
    pub database_url: String,
    pub database_user: String,
    pub database_password: String,
    pub database_name: String,
    pub database_port: String,
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
    
    
    let database_protocol = std::env::var("DATABASE_PROTOCOL")
        .expect("DATABASE_PROTOCOL must be set.");
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    let database_user = std::env::var("DATABASE_USER")
        .expect("DATABASE_USER must be set.");
    let database_password = std::env::var("DATABASE_PASSWORD")
        .expect("DATABASE_PASSWORD must be set.");
    let database_name = std::env::var("DATABASE_NAME")
        .expect("DATABASE_NAME must be set.");    
    let database_port = std::env::var("DATABASE_PORT")
        .expect("DATABASE_PORT must be set.");

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
        database_protocol,
        database_url,
        database_user,
        database_password,
        database_name,
        database_port,
        confluent_bootstrap_server,
        confluent_api_key,
        confluent_api_secret,
    }
}