use std::env;

pub fn port() -> u16 {
    // Check if it exists first
    match env::var("PORT") {
        Ok(port) => {
            let port = str::parse::<u16>(&port);

            // Then check if it's a correctly formatted u16
            match port {
                Ok(port) => port,
                Err(_) => 3000,
            }
        }
        Err(_) => 3000,
    }
}
