use std::{path::Path, process::Stdio, sync::Arc};

use axum::{extract::Multipart, response::IntoResponse, Extension, Json};
use reqwest::StatusCode;
use serde_json::Value;
use tokio::{io::AsyncWriteExt, process::Command};

use crate::{
    handlers::aws_handlers::{get_answer_file, get_testcase},
    utils::code_extensions::{get_language_from_extension, json_value},
    RedisClient,
};

pub async fn code_handler(
    Extension(redis_client): Extension<Arc<RedisClient>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        // file name and content from the multipart field
        let name = field.file_name().unwrap().to_string();
        let data = field.text().await.unwrap();

        let extension = Path::new(&name).extension().and_then(|ext| ext.to_str());
        let filename_without_ext = Path::new(&name)
            .file_stem()
            .unwrap() // Get the Option<OsStr>
            .to_string_lossy()
            .to_string();

        let language = get_language_from_extension(extension);

        let extension = match extension {
            Some(ext) => ext,
            None => {
                eprintln!("No valid extension found for file: {}", name);
                return Json("Invalid file extension").into_response();
            }
        };

        let filename = format!("main.{}", extension);
        let runner_image = format!("glot/{}:latest", language);

        let testcase = get_testcase(&filename_without_ext, &redis_client)
            .await
            .unwrap_or_else(|| "".to_string());

        //  JSON object using the escaped code in "content"
        let json_value = json_value(language, filename, data, testcase);

        //  `Command` to run `docker`
        let mut docker_process = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-i")
            .arg("--read-only")
            .arg("--tmpfs")
            .arg("/tmp:rw,noexec,nosuid,size=65536k")
            .arg("--tmpfs")
            .arg("/home/glot:rw,exec,nosuid,uid=1000,gid=1000,size=131072k")
            .arg("-u")
            .arg("glot")
            .arg("-w")
            .arg("/home/glot")
            .arg(runner_image)
            .stdin(Stdio::piped()) // Pipe stdin for passing the JSON input
            .stdout(Stdio::piped()) // Capture stdout
            .stderr(Stdio::piped()) // Capture stderr
            .spawn()
            .expect("Failed to start Docker process");

        //  JSON input to the Docker container's stdin
        if let Some(stdin) = docker_process.stdin.as_mut() {
            stdin
                .write_all(json_value.to_string().as_bytes())
                .await
                .expect("Failed to write to stdin");
        }

        //  output from stdout and stderr
        let output = docker_process
            .wait_with_output()
            .await
            .expect("Failed to wait for Docker process");

        let stdout_str = String::from_utf8_lossy(&output.stdout);
        //  JSON from the stdout string
        let json_value: Value = match serde_json::from_str(&stdout_str) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                return Json("Error in output").into_response(); // Improved error handling
            }
        };

        let stdout = json_value.get("stdout").unwrap_or(&Value::Null);
        let stderr = json_value.get("stderr").unwrap_or(&Value::Null);
        let error = json_value.get("error").unwrap_or(&Value::Null);

        let stdout_str = serde_json::to_string_pretty(&stdout)
            .unwrap()
            .trim_matches('"')
            .trim()
            .to_string();
        let _stderr_str = serde_json::to_string_pretty(&stderr)
            .unwrap()
            .trim()
            .to_string();
        let _error_str = serde_json::to_string_pretty(&error)
            .unwrap()
            .trim()
            .to_string();

        // Get answer file
        let answer = get_answer_file(&filename_without_ext, &redis_client).await;

        match answer {
            Some(expected_answer) => {
                if expected_answer.trim() == stdout_str {
                    println!("Test case passed for {}", filename_without_ext);
                    return Json("Success").into_response();
                } else {
                    let fail = format!("Failed:Expected: {}, Got: {}", expected_answer, stdout_str);
                    return Json(fail).into_response(); // Indicating failure more clearly
                }
            }
            None => {
                println!("No answer found for {}", filename_without_ext);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Database query failed"),
                )
                    .into_response();
            }
        }
    }

    // default response if no fields were processed
    Json("No fields processed").into_response()
}
