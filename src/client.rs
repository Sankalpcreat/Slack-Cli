use anyhow::{Context, Result};
use reqwest::blocking::{Client, multipart};
use serde_json::Value;
use std::collections::HashMap;

const BASE_URL: &str = "https://slack.com/api";

pub fn call(token: &str, method: &str, params: Option<&HashMap<String, String>>) -> Result<Vec<u8>> {
    call_opt(Some(token), method, params)
}

pub fn call_opt(
    token: Option<&str>,
    method: &str,
    params: Option<&HashMap<String, String>>,
) -> Result<Vec<u8>> {
    let url = format!("{}/{}", BASE_URL, method);
    let client = Client::new();
    let mut form = Vec::new();
    if let Some(p) = params {
        for (k, v) in p {
            form.push((k.clone(), v.clone()));
        }
    }
    let mut req = client.post(&url);
    if let Some(t) = token {
        req = req.header("Authorization", format!("Bearer {}", t));
    }
    let resp = req.form(&form).send().context("request failed")?;
    let body = resp.bytes().context("read body")?;
    let dec: Value = serde_json::from_slice(&body).context("parse json")?;
    if !dec.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
        let err = dec.get("error").unwrap_or(&Value::Null);
        anyhow::bail!("slack api error: {}", err);
    }
    Ok(serde_json::to_vec_pretty(&dec)?)
}

pub fn upload_file(
    token: &str,
    channel_id: &str,
    file_path: &str,
    initial_comment: Option<&str>,
) -> Result<Vec<u8>> {
    let meta = std::fs::metadata(file_path).context("file metadata")?;
    let base_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("file");

    let mut params = HashMap::new();
    params.insert("length".to_string(), meta.len().to_string());
    params.insert("filename".to_string(), base_name.to_string());

    let get_resp = call(token, "files.getUploadURLExternal", Some(&params))?;
    let get_dec: Value = serde_json::from_slice(&get_resp)?;
    let upload_url = get_dec
        .get("upload_url")
        .and_then(|v| v.as_str())
        .context("missing upload_url")?;
    let file_id = get_dec
        .get("file_id")
        .and_then(|v| v.as_str())
        .context("missing file_id")?;

    let file = std::fs::File::open(file_path).context("open file")?;
    let part = multipart::Part::reader(file).file_name(base_name.to_string());
    let form = multipart::Form::new().part("file", part);

    let client = Client::new();
    let resp = client
        .post(upload_url)
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .context("upload failed")?;

    if !resp.status().is_success() {
        let body = resp.text().unwrap_or_default();
        anyhow::bail!("upload failed: {}", body);
    }

    let files_json = serde_json::json!([{"id": file_id, "title": base_name}]);
    let mut complete_params = HashMap::new();
    complete_params.insert("files".to_string(), files_json.to_string());
    complete_params.insert("channel_id".to_string(), channel_id.to_string());
    if let Some(c) = initial_comment {
        complete_params.insert("initial_comment".to_string(), c.to_string());
    }

    call(token, "files.completeUploadExternal", Some(&complete_params))
}

pub fn download_file(token: &str, file_id: &str, output_path: Option<&str>) -> Result<String> {
    let mut params = HashMap::new();
    params.insert("file".to_string(), file_id.to_string());
    let info_resp = call(token, "files.info", Some(&params))?;
    let info: Value = serde_json::from_slice(&info_resp)?;
    let file = info.get("file").context("no file in response")?;
    let url = file
        .get("url_private_download")
        .and_then(|v| v.as_str())
        .or_else(|| file.get("url_private").and_then(|v| v.as_str()))
        .context("no download URL")?;

    let client = Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .context("download request")?;

    if !resp.status().is_success() {
        anyhow::bail!("download failed {}", resp.status());
    }

    let out_path = output_path
        .map(|s| s.to_string())
        .or_else(|| file.get("name").and_then(|v| v.as_str()).map(String::from))
        .unwrap_or_else(|| file_id.to_string());

    let body = resp.bytes().context("download body")?;
    std::fs::write(&out_path, &body).context("write file")?;
    Ok(out_path)
}

pub fn set_user_photo(token: &str, image_path: &str) -> Result<Vec<u8>> {
    let base_name = std::path::Path::new(image_path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("image");
    let file = std::fs::File::open(image_path).context("open image")?;
    let part = multipart::Part::reader(file).file_name(base_name.to_string());
    let form = multipart::Form::new().part("image", part);

    let url = format!("{}/users.setPhoto", BASE_URL);
    let client = Client::new();
    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .multipart(form)
        .send()
        .context("set photo request")?;

    let body = resp.bytes().context("read body")?;
    let dec: Value = serde_json::from_slice(&body).context("parse json")?;
    if !dec.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
        let err = dec.get("error").unwrap_or(&Value::Null);
        anyhow::bail!("slack api error: {}", err);
    }
    Ok(serde_json::to_vec_pretty(&dec)?)
}
