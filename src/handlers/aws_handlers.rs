use core::str;

use aws_config::Region;
use aws_sdk_s3::Client;

pub async fn get_testcase(filename: String) -> Option<String>
// -> Result<GetObjectOutput,SdkError<GetObjectError>>
{
    let config = aws_config::from_env()
        .region(Region::new("ap-south-1"))
        .load()
        .await;
    let client = Client::new(&config);
    let bucket_name = "affinitys3";
    let key = format!("testcases/{}.txt", filename);
    println!("{}", key);

    let something = client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await;

    match something {
        Ok(something) => {
            let mut body = something.body;

            let mut all_bytes = Vec::new();

            while let Some(bytes) = body.try_next().await.unwrap() {
                // Convert the byte vector to a string (assuming UTF-8 encoding)
                all_bytes.extend_from_slice(&bytes);
            }

            Some(str::from_utf8(&all_bytes).unwrap().to_string())
        }
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            None
        }
    }
}
