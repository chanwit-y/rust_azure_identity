// use azure_sdk_auth_aad::ClientSecretCredential;

use azure_identity::client_credentials_flow;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use dotenv::dotenv;
use futures::StreamExt;
use std::{env, error::Error, fs::File, io::Write};

// #[tokio::main]
#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    download_blob().await?;

    Ok(())
}

async fn get_token() -> Result<(), Box<dyn Error>> {
    println!("start");

    let client_id = env::var("CLIENT_ID").unwrap();
    let tenant_id = env::var("TENANT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();
    let scope = env::var("SCOPE").unwrap();

    let http_client = azure_core::new_http_client();

    let token = client_credentials_flow::perform(
        http_client.clone(),
        &client_id,
        &client_secret,
        &[&scope],
        &tenant_id,
    )
    .await?;

    println!("{}", token.access_token.secret());
    Ok(())
}

async fn download_blob() -> Result<(), Box<dyn Error>> {
    let account = env::var("STORAGE_ACCOUNT").unwrap();
    let access_key = env::var("STORAGE_ACCESS_KEY").unwrap();
    let container = env::var("STORAGE_CONTAINER").unwrap();

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let service_client = BlobServiceClient::new(account, storage_credentials);

    let blob_client = service_client
        .container_client(&container)
        .blob_client("06_BNAC_JV_Jun 2023 1.pdf");

    // let mut stream = blob_client.get().;

    // https://bpdevfilestore1.blob.core.windows.net/drop-zone/06_BNAC_JV_Jun 2023 1.pdf

    let mut stream = blob_client.get().into_stream();
    let mut file = File::create("./tmp/06_BNAC_JV_Jun 2023 1.pdf").expect("Unable to create file");

    while let Some(value) = stream.next().await {
       let data = value?.data.collect().await?; 

         println!("received {:?} bytes", data.len());

        file.write_all(data.as_ref()).expect("Unable to write data");
    }

    // while let Some(res) = stream.next().await {
    //     let data = res.unwrap().blob;
    //     println!("received {:?} bytes", data);
    //     // println!("received {:?} bytes", data);

    //     // file.write_all(data.as_ref()).expect("Unable to write data");
    // }

    // // only get the first chunk
    // let result = blob_client
    //     .get()
    //     .into_stream()
    //     .next()
    //     .await
    //     .expect("stream failed")?;
    // // println!("{result:?}");

    // println!("{:?}", result);

    // let mut stream = blob_client
    //     .get()
    //     .into_stream();

    // while  let Some(value) = stream.next().await {
    //    let data = value?.data.collect().await?;
    //    println!("received {:?} bytes", data.len());
    // }

    Ok(())
}
