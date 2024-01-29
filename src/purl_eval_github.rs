pub async fn repo_exists_with_version(
    user_or_org_name: &str,
    repo_name: &str,
    version: &str,
) -> leptos::error::Result<bool> {
    let resp = reqwasm::http::Request::get(&format!(
        "https://api.github.com/repos/{user_or_org_name}/{repo_name}/releases"
    ))
    .send()
    .await?;

    let releases = match resp.status() {
        200 => Ok::<Option<Vec<GithubRelease>>, leptos::error::Error>(Some(
            resp.json::<Vec<GithubRelease>>().await?,
        )),
        404 => Ok(None),
        unexpected_status_code => {
            return Err(GithubCheckErr::UnexpectedStatusCode(unexpected_status_code).into());
        }
    }?;

    // if we already find it in the releases, we are happy to end the search there
    if let Some(releases) = releases {
        if releases.iter().any(|release| version == release.name) {
            return Ok(true);
        }
    }

    let resp = reqwasm::http::Request::get(&format!(
        "https://api.github.com/repos/{user_or_org_name}/{repo_name}/tags"
    ))
    .send()
    .await?;

    let tags = match resp.status() {
        200 => Ok::<Option<Vec<GithubTag>>, leptos::error::Error>(Some(
            resp.json::<Vec<GithubTag>>().await?,
        )),
        404 => Ok(None),
        unexpected_status_code => {
            return Err(GithubCheckErr::UnexpectedStatusCode(unexpected_status_code).into());
        }
    }?;

    if let Some(tags) = tags {
        if tags.iter().any(|tag| version == tag.name) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub async fn repo_exists(user_or_org_name: &str, repo_name: &str) -> leptos::error::Result<bool> {
    let x = reqwasm::http::Request::get(&format!(
        "https://api.github.com/repos/{user_or_org_name}/{repo_name}"
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
    let x =
        reqwasm::http::Request::get(&format!("https://api.github.com/users/{user_or_org_name}"))
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GithubRelease {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: u64,
    pub author: GithubReleaseAuthor,
    pub node_id: String,
    pub tag_name: String,
    pub target_comitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub assets: Vec<GithubReleaseAsset>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GithubReleaseAsset {
    // intentionally abbreviated
    pub name: String,
    pub url: String,
    pub download_count: u64,
    pub content_type: String,
    pub state: String,
    pub size: u64,
    pub browser_download_url: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GithubReleaseAuthor {
    // intentionally abbreviated
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GithubTag {
    pub name: String,
    pub zipball_url: String,
    pub tarball_url: String,
    pub commit: Commit,
    pub node_id: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Commit {
    pub sha: String,
    pub url: String,
}
