use super::{
    config_model::{Database, DotEnvyConfig, NisitSecret, Server, StaffSecret},
    stage::Stage,
};
use anyhow::Result;

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT must be set")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT must be set")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT must be set")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::from_string(&stage_str).unwrap_or_default()
}

pub fn get_staff_secret() -> Result<StaffSecret> {
    dotenvy::dotenv().ok();

    Ok(StaffSecret {
        secret: std::env::var("JWT_STAFF_SECRET").expect("JWT_STAFF_SECRET must be set"),
        refresh_secret: std::env::var("JWT_STAFF_REFRESH_SECRET")
            .expect("JWT_STAFF_REFRESH_SECRET must be set"),
    })
}

pub fn get_nisit_secret() -> Result<NisitSecret> {
    dotenvy::dotenv().ok();

    Ok(NisitSecret {
        secret: std::env::var("JWT_NISIT_SECRET").expect("JWT_NISIT_SECRET must be set"),
        refresh_secret: std::env::var("JWT_NISIT_REFRESH_SECRET")
            .expect("JWT_NISIT_REFRESH_SECRET must be set"),
    })
}
