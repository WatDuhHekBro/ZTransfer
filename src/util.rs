use std::env::{args, var};

pub fn port() -> u16 {
    let args = args().collect::<Vec<String>>();

    // If the first argument is a valid port number, use that instead of the .env value
    if let Some(port) = args.get(1) {
        let port = str::parse::<u16>(port);

        if let Ok(port) = port {
            return port;
        }
    }

    // Check if it exists first
    match var("PORT") {
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
