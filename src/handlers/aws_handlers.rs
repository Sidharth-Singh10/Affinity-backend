use core::str;
use std::sync::Arc;

use aws_config::Region;
use aws_sdk_s3::Client;

use crate::RedisClient;

pub async fn get_testcase(filename: &String, redis_client: &Arc<RedisClient>) -> Option<String>
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

    // get value from redis first
    let r = redis_client.get_value(&key).await.unwrap();

    match r {
        Some(r) => Some(r),

        None => {
            println!("from aws");
            let something = client
                .get_object()
                .bucket(bucket_name)
                .key(&key)
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
                    let str = str::from_utf8(&all_bytes).unwrap().to_string();

                    // set value into redis
                    redis_client.set_value(&key, str.as_str()).await.unwrap();
                    Some(str)
                }
                Err(e) => {
                    eprintln!("Error occurred: {}", e);
                    None
                }
            }
        }
    }
}
pub async fn get_answer_file(filename: &String, redis_client: &Arc<RedisClient>) -> Option<String>
// -> Result<GetObjectOutput,SdkError<GetObjectError>>
{
    let config = aws_config::from_env()
        .region(Region::new("ap-south-1"))
        .load()
        .await;
    let client = Client::new(&config);
    let bucket_name = "affinitys3";
    let key = format!("answers/{}.txt", filename);
    println!("{}", key);

    // get value from redis first
    let r = redis_client.get_value(&key).await.unwrap();

    match r {
        Some(r) => Some(r),

        None => {
            let something = client
                .get_object()
                .bucket(bucket_name)
                .key(&key)
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
                    let str = str::from_utf8(&all_bytes).unwrap().to_string();

                    // set value into redis
                    redis_client.set_value(&key, str.as_str()).await.unwrap();
                    Some(str)
                }
                Err(e) => {
                    eprintln!("Error occurred: {}", e);
                    None
                }
            }
        }
    }
}
