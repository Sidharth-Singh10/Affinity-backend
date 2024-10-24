use std::{path::Path, process::Stdio};

use axum::extract::Multipart;
use tokio::{io::AsyncWriteExt, process::Command};

use crate::{handlers::aws_handlers::get_testcase, utils::code_extensions::{get_language_from_extension, json_value}};

pub async fn code_handler(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.file_name().unwrap().to_string();
        let data = field.text().await.unwrap();

        let extension = Path::new(&name).extension().and_then(|ext| ext.to_str());
        let filename_without_ext = Path::new(&name).file_stem().unwrap()  // Get the Option<OsStr>
        .to_string_lossy() 
        .to_string(); 
    
        let language = get_language_from_extension(extension);

        let filename = format!("main.{}", extension.unwrap());
        let runner_image = format!("glot/{}:latest", language);

        let testcase = get_testcase(filename_without_ext).await.unwrap_or_else(||{"".to_string()});

        // Create a JSON object using the escaped code in "content"
        let json_value = json_value(language, filename, data, testcase);

        // Create a `Command` to run `docker`
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


        // Write the JSON input to the Docker container's stdin
        if let Some(stdin) = docker_process.stdin.as_mut() {
            stdin
                .write_all(json_value.to_string().as_bytes())
                .await
                .expect("Failed to write to stdin");
        }

        // Capture the output from stdout and stderr
        let output = docker_process
            .wait_with_output()
            .await
            .expect("Failed to read output");

        // Print stdout and stderr
        println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
    }
}
