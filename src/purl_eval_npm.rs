pub async fn get_package(package_name: &str) -> leptos::error::Result<Option<NpmPackage>> {
    let resp = reqwasm::http::Request::get(&format!("https://registry.npmjs.org/{package_name}"))
        .send()
        .await?;
    match resp.status() {
        200 => {
            let package: NpmPackage = resp.json().await?;
            Ok(Some(package))
        }
        404 => Ok(None),
        unexpected_status_code => {
            Err(NpmCheckErr::UnexpectedStatusCode(unexpected_status_code).into())
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NpmPackage {}

#[derive(thiserror::Error, Clone, Debug)]
pub enum NpmCheckErr {
    #[error("unexpected status code ({})", .0)]
    UnexpectedStatusCode(u16),
}
