use core::str;
use std::{collections::HashMap, sync::Arc, time::Duration};

use crate::RedisClient;
use aws_config::Region;
use aws_sdk_s3::{presigning::PresigningConfig, Client};
use axum::{extract::Query, Json};
use reqwest::StatusCode;

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

pub async fn upload_to_aws(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<String>, StatusCode> {
    let config = aws_config::from_env()
        .region(Region::new("ap-south-1"))
        .load()
        .await;
    let client = Client::new(&config);

    let bucket_name = "affinitys3";

    let username = match params.get("username") {
        Some(username) => username,
        None => {
            eprintln!("Username parameter is missing.");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let filename = match params.get("filename") {
        Some(username) => username,
        None => {
            eprintln!("filename parameter is missing.");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let avatar_object_key = format!("username/{}/{}", username, filename);

    // Generate the presigned URL
    let presigned_request = match client
        .put_object()
        .bucket(bucket_name)
        .key(avatar_object_key.clone())
        .presigned(PresigningConfig::expires_in(Duration::from_secs(6000)).unwrap())
        .await
    {
        Ok(request) => request,
        Err(e) => {
            eprintln!("Failed to create presigned URL: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("Object URI: {}", presigned_request.uri());

    Ok(Json(presigned_request.uri().to_string()))
}
