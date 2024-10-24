use serde_json::json;

enum Language {
    JavaScript,
    Clang,
    Python,
    Unknown,
}
// Function to map file extensions to languages
pub fn get_language_from_extension(extension: Option<&str>) -> String {
    let lang = match extension {
        Some("js") => Language::JavaScript,
        Some("cpp") | Some("c") | Some("h") => Language::Clang,
        Some("py") => Language::Python,
        _ => Language::Unknown,
    };

    display_language(&lang)
}

fn display_language(language: &Language) -> String {
    match language {
        Language::JavaScript => String::from("javascript"),
        Language::Clang => String::from("clang"),
        Language::Python => String::from("python"),
        Language::Unknown => String::from("Unknown"),
    }
}

pub fn json_value(
    lang: String,
    filename: String,
    data: String,
    testcase: String,
) -> serde_json::Value {
    let json_value = match lang.as_str() {
        "javascript" => json!({
            "language": "javascript",
            "stdin" : testcase,
            "files": [{
                "name": filename,
                "content": data
            }]
        }),
        "cpp" | "c++" | "clang" => json!({
            "language": "cpp",
            "stdin" : testcase,
            "files": [{
                "name": filename,
                "content": data
            }],
        }),
        "python" => json!({
            "language": "python",
            "stdin" : testcase,
            "files": [{
                "name": filename,
                "content": data
            }]
        }),
        // Handle more languages as needed
        _ => json!({
            "language": lang,
            "files": [{
                "name": filename,
                "content": data
            }]
        }),
    };

    json_value
}
