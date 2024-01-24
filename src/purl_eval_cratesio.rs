pub async fn get_versions(crate_name: &str) -> leptos::error::Result<Vec<String>> {
    match reqwasm::http::Request::get(&format!(
        "https://crates.io/api/v1/crates/{crate_name}/versions"
    ))
    .send()
    .await?
    .json::<CratesioVersionResponse>()
    .await?
    {
        CratesioVersionResponse::SuccessfulResponse { versions } => {
            Ok(versions.iter().map(|v| v.num.clone()).collect())
        }
        CratesioVersionResponse::ErrorResponse { errors } => Err(CratesioError { errors }.into()),
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum CratesioVersionResponse {
    #[serde(rename(serialize = "ser_name"))]
    SuccessfulResponse {
        versions: Vec<CratesioVersion>,
    },
    ErrorResponse {
        errors: Vec<CratesioErrorDetails>,
    },
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CratesioErrorDetails {
    pub details: String,
}

#[derive(thiserror::Error, Clone, Debug)]
pub struct CratesioError {
    pub errors: Vec<CratesioErrorDetails>,
}

impl std::fmt::Display for CratesioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut errors = self
            .errors
            .iter()
            .map(|error| format!("Error: {}", error.details))
            .collect::<Vec<_>>();
        errors.sort();
        write!(f, "{}", errors.join("\n"))
    }
}

#[derive(serde::Deserialize)]
pub struct CratesioVersion {
    //   "audit_actions": [
    //     {
    //       "action": "publish",
    //       "time": "2023-12-18T20:41:19.272737+00:00",
    //       "user": {
    //         "avatar": "https://avatars.githubusercontent.com/u/51479?v=4",
    //         "id": 359,
    //         "login": "seanmonstar",
    //         "name": "Sean McArthur",
    //         "url": "https://github.com/seanmonstar"
    //       }
    //     }
    //   ],

    //   "checksum": "37b1ae8d9ac08420c66222fb9096fc5de435c3c48542bc5336c51892cffafb41",

    //   "crate": "reqwest",
    //   "crate_size": 158448,
    //   "created_at": "2023-12-18T20:41:19.272737+00:00",
    //   "dl_path": "/api/v1/crates/reqwest/0.11.23/download",
    //   "downloads": 1684159,
    //   "features": {
    //     "__internal_proxy_sys_no_cache": [],
    //     "__rustls": [
    //       "hyper-rustls",
    //       "tokio-rustls",
    //       "rustls",
    //       "__tls",
    //       "rustls-pemfile"
    //     ],
    //     "__tls": [],
    //     "blocking": [
    //       "futures-util/io",
    //       "tokio/rt-multi-thread",
    //       "tokio/sync"
    //     ],
    //     "brotli": [
    //       "async-compression",
    //       "async-compression/brotli",
    //       "tokio-util"
    //     ],
    //     "cookies": [
    //       "cookie_crate",
    //       "cookie_store"
    //     ],
    //     "default": [
    //       "default-tls"
    //     ],
    //     "default-tls": [
    //       "hyper-tls",
    //       "native-tls-crate",
    //       "__tls",
    //       "tokio-native-tls"
    //     ],
    //     "deflate": [
    //       "async-compression",
    //       "async-compression/zlib",
    //       "tokio-util"
    //     ],
    //     "gzip": [
    //       "async-compression",
    //       "async-compression/gzip",
    //       "tokio-util"
    //     ],
    //     "http3": [
    //       "rustls-tls-manual-roots",
    //       "h3",
    //       "h3-quinn",
    //       "quinn",
    //       "futures-channel"
    //     ],
    //     "json": [
    //       "serde_json"
    //     ],
    //     "multipart": [
    //       "mime_guess"
    //     ],
    //     "native-tls": [
    //       "default-tls"
    //     ],
    //     "native-tls-alpn": [
    //       "native-tls",
    //       "native-tls-crate/alpn"
    //     ],
    //     "native-tls-vendored": [
    //       "native-tls",
    //       "native-tls-crate/vendored"
    //     ],
    //     "rustls-tls": [
    //       "rustls-tls-webpki-roots"
    //     ],
    //     "rustls-tls-manual-roots": [
    //       "__rustls"
    //     ],
    //     "rustls-tls-native-roots": [
    //       "rustls-native-certs",
    //       "__rustls"
    //     ],
    //     "rustls-tls-webpki-roots": [
    //       "webpki-roots",
    //       "__rustls"
    //     ],
    //     "socks": [
    //       "tokio-socks"
    //     ],
    //     "stream": [
    //       "tokio/fs",
    //       "tokio-util",
    //       "wasm-streams"
    //     ],
    //     "trust-dns": [
    //       "trust-dns-resolver"
    //     ]
    //   },

    //   "id": 990011,

    //   "license": "MIT OR Apache-2.0",

    //   "links": {
    //     "authors": "/api/v1/crates/reqwest/0.11.23/authors",
    //     "dependencies": "/api/v1/crates/reqwest/0.11.23/dependencies",
    //     "version_downloads": "/api/v1/crates/reqwest/0.11.23/downloads"
    //   },

    //   "num": "0.11.23",
    pub num: String,

    //   "published_by": {
    //     "avatar": "https://avatars.githubusercontent.com/u/51479?v=4",
    //     "id": 359,
    //     "login": "seanmonstar",
    //     "name": "Sean McArthur",
    //     "url": "https://github.com/seanmonstar"
    //   },

    //   "readme_path": "/api/v1/crates/reqwest/0.11.23/readme",

    //   "rust_version": "1.63.0",

    //   "updated_at": "2023-12-18T20:41:19.272737+00:00",

    //   "yanked": false
    pub yanked: bool,
}
