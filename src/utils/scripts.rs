use serde_json::Value;
use std::io;
use std::process::Stdio;
use tokio::fs;
use tokio::process::Command;

pub async fn docker_run(args: &[&str]) -> io::Result<String> {
    let script_path = "./dockerpyfile.sh";

    let convert_path = "./convert.sh";

    let output2 = Command::new("bash")
        .arg(convert_path)
        .args(args)
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .await?;

    println!("{:?}", output2);

    let output = Command::new("bash")
        .arg(script_path)
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .await?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Assuming stdout contains JSON, let's parse it.
        let json_output: Value = serde_json::from_str(&stdout).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse JSON: {}", e),
            )
        })?;

        // Extract the "stdout" field from the JSON and return it.
        if let Some(stdout_value) = json_output.get("stdout") {
            if let Some(stdout_str) = stdout_value.as_str() {
                return Ok(stdout_str.to_string());
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Missing or invalid 'stdout' field",
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, stderr.to_string()))
    }
}
// pub async fn compare_with_answer_file(stdout: &str, answer_file_path: &str) -> io::Result<bool> {
//     let add = format!("./ans/{}", answer_file_path);

//     let answer = fs::read_to_string(add).await;

//     match answer {
//         Ok(answer) => {
//             Ok(stdout.trim() == answer.trim())
//         },
//         Err(e) => {
//             println!("error reading answer file");
//             String::new()
//         }
//     };
// }
pub async fn compare_with_answer_file(stdout: &str, answer_file_path: &str) -> io::Result<bool> {
    let add = format!("./ans/{}", answer_file_path);

    match fs::read_to_string(add).await {
        Ok(answer) => {
            Ok(stdout.trim() == answer.trim())
        },
        Err(e) => {
            println!("error reading answer file: {}", e);
            Err(e)
        }
    }
}
