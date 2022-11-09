use crate::structures::{Broadcast, BroadcastMessage, WsMessage};
use actix::Addr;
use actix_multipart::Multipart;
use actix_web::{http, post, web, Error, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use sanitize_filename::sanitize;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[post("upload")]
pub async fn upload(
    mut payload: Multipart,
    header: HttpRequest,
    data: web::Data<Arc<Mutex<Addr<Broadcast>>>>,
) -> Result<HttpResponse, Error> {
    println!("transfer called");
    let broadcaster = &*data.lock().unwrap();
    let size = get_size_from_header(&header);

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize(&filename));
        let mut counter = 0;

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| File::create(filepath)).await.unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            counter += data.len();

            if let Some(size) = size {
                // There's no need for double precision since we're going to round to 2 places anyway
                let percentage_progress = counter as f32 / size as f32 * 100.0;
                println!(
                    "Bytes Transferred: {} ({:.2}%)",
                    counter, percentage_progress
                );
            } else {
                println!("Bytes Transferred: {}", counter);
            }

            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }

        // I can't think of any reason why serialization would fail here
        broadcaster.do_send(BroadcastMessage(
            serde_json::to_string(&WsMessage::AddFile {
                filename: String::from(filename),
                size: size.unwrap_or(0),
            })
            .expect("Serialization Error?!"),
        ));
    }

    println!("File Uploading Complete");

    Ok(HttpResponse::SeeOther()
        .header(http::header::LOCATION, "/")
        .finish())
}

// This must be u64 especially for large files (multi GB) where the byte count exceeds a 32 bit integer
// And there's no need to worry about the size here: i64 would count the exact number of bytes for 8,388,607 TB at minimum
fn get_size_from_header(header: &HttpRequest) -> Option<u64> {
    // Will return None if the property doesn't exist on the header for whatever reason
    let size = match header.headers().get("content-length") {
        Some(size) => size,
        None => return None,
    };

    // Will throw an error if a non-ASCII header is sent
    let size = match size.to_str() {
        Ok(size) => size,
        Err(_) => return None,
    };

    // I can't think of a situation where this would crash, the header should be well-formed
    let size = size.parse::<u64>().unwrap();
    Some(size)
}
