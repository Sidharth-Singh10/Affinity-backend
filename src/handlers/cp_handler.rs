use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};
use tokio::fs::{create_dir_all, File};
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

use crate::utils::scripts::{compare_with_answer_file, docker_run};
pub async fn code_handler(
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field
            .file_name()
            .unwrap_or("default_filename.txt")
            .to_string();
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());

        //defingn path
        let dir: PathBuf = "./uploads".into();

        if let Err(err) = create_dir_all(&dir).await {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
        }

        let filepath = dir.join(filename.clone());

        let mut file = File::create(&filepath)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        file.write_all(&data)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

        let add = format!("./uploads/{}", filename.clone());

        let args = [add.as_str()];

        match docker_run(&args, filename.clone()).await {
            Ok(stdout) => match compare_with_answer_file(&stdout, &filename).await {
                Ok(true) => {
                    println!("The output matches the answer file.");
                    return Ok("AC");
                }
                Ok(false) => {
                    println!("The output does NOT match the answer file.");
                    return Ok("WA");
                }
                Err(e) => {
                    eprintln!("Error comparing with answer file: {}", e);
                    return Ok("Error comparing with answer file");
                }
            },
            Err(e) => {
                eprintln!("Error running script: {}", e);
                return Ok("Error running docker");
            }
        }
    }

    Ok("File uploaded successfully")
    // Define the path where you want to save the file

    // Save the file

    // Ok("File uploaded successfully".to_string())
}