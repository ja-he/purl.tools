use std::collections::HashMap;

pub async fn get_package(package_name: &str) -> leptos::error::Result<Option<NpmPackage>> {
    let resp = match reqwasm::http::Request::get(&format!(
        "https://registry.npmjs.org/{package_name}"
    ))
    .send()
    .await
    {
        Ok(resp) => resp,
        Err(e) => {
            log::warn!("got error for NPM package '{package_name}' check, which (anecdotally) seems to indicate that a package does not exist; this is because it seems NPM allows requests for existing packages but CORS-blocks other requests ({e:?})");
            return Ok(None);
        }
    };
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
pub struct NpmPackage {
    // {
    //   "_id": "express",
    #[serde(rename = "_id")]
    pub id: String,

    //   "_rev": "4061-129d0cedb3d9c5111de02c1e1e2a35f6",
    #[serde(rename = "_rev")]
    pub rev: String,

    //   "name": "express",
    pub name: String,

    //   "description": "Fast, unopinionated, minimalist web framework",
    pub description: String,

    //   "dist-tags": {
    //     "latest": "4.18.2",
    //     "next": "5.0.0-beta.1"
    //   },
    // TODO

    //   "versions": { ... },
    pub versions: HashMap<String, NpmVersion>,

    //   "maintainers": [ { "email": "mikeal.rogers@gmail.com", "name": "mikeal" }, ... ],
    //   "author": { "name": "TJ Holowaychuk", "email": "tj@vision-media.ca" },
    //   "time": {
    //     "modified": "2024-01-16T06:36:24.871Z",
    //     "created": "2010-12-29T19:38:25.450Z",
    //     "0.14.0": "2010-12-29T19:38:25.450Z",
    //     ...
    //   },
    //   "repository": {
    //     "type": "git",
    //     "url": "git+https://github.com/expressjs/express.git"
    //   },
    //   "users": {
    //     "422303771": true,
    //     "coverslide": true,
    //     "gevorg": true,
    //     ...
    //   },
    //   "readme": "[markdown string]",
    //   "readmeFilename": "Readme.md",
    //   "homepage": "http://expressjs.com/",
    //   "keywords": [ "express", "framework", "sinatra", "web", "http", "rest", "restful", "router", "app", "api" ],
    //   "contributors": [ { "name": "Aaron Heckmann", "email": "aaron.heckmann+github@gmail.com" }, ... ],
    //   "bugs": { "url": "https://github.com/expressjs/express/issues" },

    //   "license": "MIT"
    pub license: Option<String>,
    // }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NpmVersion {
    // {
    //   "name": "express",
    #[serde(rename = "name")]
    pub package_name: String,

    //   "description": "Sinatra inspired web development framework",
    #[serde(rename = "description")]
    pub package_description_at_version: String,

    //   "version": "0.14.0",
    #[serde(rename = "version")]
    pub version_name: String,

    //   "author":         { "name": "TJ Holowaychuk", "email": "tj@vision-media.ca" },
    //   "contributors": [ { "name": "TJ Holowaychuk", "email": "tj@vision-media.ca" }, ... ],
    //   "keywords": [ "framework", "sinatra", "web", "rest", "restful" ],
    //   "directories": { "lib": "./lib" },
    //   "scripts": { "test": "make test" },
    //   "engines": { "node": ">= 0.1.98" },

    //   "_id": "express@0.14.0",
    #[serde(rename = "_id")]
    pub package_and_version_id: String,

    //   "_nodeSupported": true,
    //   "_npmVersion": "0.2.7-2",
    //   "_nodeVersion": "v0.3.1-pre",
    //   "dist": {
    //     "tarball": "https://registry.npmjs.org/express/-/express-0.14.0.tgz",
    //     "shasum": "7b33a9fb54c605a3be46c1d3dbbc821acf1d2efb",
    //     "integrity": "sha512-ULazYLF3/YqOU5rzkviWJEd4TNZ0j77Nymuqa1+sQe0dhxcsDzKOQK8GemM9S3i8x2Q55GWXhnhRHwYaJIrM1g==",
    //     "signatures": [
    //       {
    //         "keyid": "SHA256:jl3bwswu80PjjokCgh0o2w5c2U4LhQAE57gj9cz1kzA",
    //         "sig": "MEYCIQCGihQqfDiARxIfslKVGe5vzfxCGOh+vhulZER4lO9oBQIhAO2VvApOYRVumT9XVDffvpYimysO/Hm1Qd59+KBublcM"
    //       }
    //     ]
    //   },

    //   "deprecated": "express 0.x series is deprecated"
    pub deprecated: Option<String>,
    // },
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum NpmCheckErr {
    #[error("unexpected status code ({})", .0)]
    UnexpectedStatusCode(u16),
}
