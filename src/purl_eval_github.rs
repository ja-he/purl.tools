pub async fn repo_exists_with_version(
    user_or_org_name: &str,
    repo_name: &str,
    version: &str,
) -> leptos::error::Result<bool> {
    let x = reqwasm::http::Request::get(&format!(
        "https://github.com/{user_or_org_name}/{repo_name}/releases/tag/{version}"
    ))
    .send()
    .await?;
    match x.status() {
        200 => Ok(true),
        404 => Ok(false),
        unexpected_status_code => {
            Err(GithubCheckErr::UnexpectedStatusCode(unexpected_status_code).into())
        }
    }
}

pub async fn repo_exists(user_or_org_name: &str, repo_name: &str) -> leptos::error::Result<bool> {
    let x = reqwasm::http::Request::get(&format!(
        "https://github.com/{user_or_org_name}/{repo_name}/"
    ))
    .send()
    .await?;
    match x.status() {
        200 => Ok(true),
        404 => Ok(false),
        unexpected_status_code => {
            Err(GithubCheckErr::UnexpectedStatusCode(unexpected_status_code).into())
        }
    }
}

pub async fn user_or_org_exists(user_or_org_name: &str) -> leptos::error::Result<bool> {
    let x = reqwasm::http::Request::get(&format!("https://github.com/{user_or_org_name}/"))
        .send()
        .await?;
    match x.status() {
        200 => Ok(true),
        404 => Ok(false),
        unexpected_status_code => {
            Err(GithubCheckErr::UnexpectedStatusCode(unexpected_status_code).into())
        }
    }
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum GithubCheckErr {
    #[error("unexpected status code ({})", .0)]
    UnexpectedStatusCode(u16),
}
