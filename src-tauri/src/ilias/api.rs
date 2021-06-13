use lazy_static::lazy_static;
use reqwest::{header::CONTENT_TYPE, Client, ClientBuilder, Body};
use scraper::{Html, Selector};
use tauri::async_runtime::RwLock;
use urlencoding::encode;
use reqwest::redirect::Policy;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncWriteExt};

pub const ILIAS_URL: &str = "https://ilias.uni-freiburg.de";

pub type SharedToken = RwLock<Option<String>>;

#[derive(Debug)]
pub struct IliasApi {
    client: Client,
    token: SharedToken,
}

#[derive(Debug)]
pub enum IliasError {
    CreationFailed,
    ConnectionFailed,
    ParsingFailed,
    InvalidCredentials,
    Unauthorized,
    UnsupportedFileFormat,
    IOOperationFailed,
    RenameFailed,
}

impl IliasError {
    pub fn message(&self) -> String {
        match self {
            IliasError::CreationFailed => "failed to create https client".to_string(),
            IliasError::ConnectionFailed => "failed to connect to ilias".to_string(),
            IliasError::ParsingFailed => "failed to parse dom elements".to_string(),
            IliasError::InvalidCredentials => "invalid credentials".to_string(),
            IliasError::Unauthorized => "action not authorized".to_string(),
            IliasError::UnsupportedFileFormat => "file format is not supported".to_string(),
            IliasError::IOOperationFailed => { "failed to perform some io operation".to_string() }
            IliasError::RenameFailed => { "could not rename some file".to_string() }
        }
    }
}

lazy_static! {
  pub static ref CONTEXT: Selector = Selector::parse("#LoginForm_context").unwrap();
  pub static ref INPUTS: Selector = Selector::parse("input").unwrap();
}

impl IliasApi {
    pub fn new() -> Result<Self, IliasError> {
        Ok(IliasApi {
            client: ClientBuilder::new()
                .cookie_store(true)
                .http1_title_case_headers()
                .build()
                .map_err(|_| IliasError::CreationFailed)?,
            token: RwLock::new(None),
        })
    }


    pub async fn login(&self, username: String, password: String) -> Result<(), IliasError> {
        let (url, context_body) = {
            let body = self.client
                .get(format!("{}/shib_login.php?target=", ILIAS_URL).as_str())
                .send()
                .await
                .map_err(|_| IliasError::ConnectionFailed)?;
            (
                body.url().as_str().to_owned(),
                body
                    .text()
                    .await
                    .map_err(|_| IliasError::ConnectionFailed)?,
            )
        };


        let context = Html::parse_document(&context_body)
            .select(&CONTEXT)
            .next()
            .ok_or(IliasError::ParsingFailed)?
            .value()
            .attr("value")
            .ok_or(IliasError::ParsingFailed)?
            .to_owned();

        let login_body = self.client
            .post(url)
            .body(format!(
                "LoginForm%5Bcontext%5D={}&LoginForm%5Busername%5D={}&LoginForm%5Bpassword%5D={}&yt0=Login",
                encode(&context),
                username,
                password
            ))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .await
            .map_err(|_| IliasError::ConnectionFailed)?
            .text()
            .await
            .map_err(|_| IliasError::ConnectionFailed)?;


        let (relay_state, saml_response) = {
            let dom = Html::parse_document(&login_body);
            let mut inputs = dom.select(&INPUTS);
            (
                inputs
                    .next()
                    .ok_or(IliasError::ParsingFailed)?
                    .value()
                    .attr("value")
                    .ok_or(IliasError::ParsingFailed)?
                    .to_owned(),
                inputs
                    .next()
                    .ok_or(IliasError::ParsingFailed)?
                    .value()
                    .attr("value")
                    .ok_or(IliasError::ParsingFailed)?
                    .to_owned(),
            )
        };

        let client = reqwest::Client::builder()
            .redirect(Policy::custom(|attempt| {
                if attempt.previous().len() > 1 {
                    attempt.stop()
                } else {
                    attempt.follow()
                }
            }))
            .cookie_store(true)
            .build()
            .unwrap();


        let session_id = client
            .post(format!("{}/Shibboleth.sso/SAML2/POST", ILIAS_URL))
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(format!(
                "RelayState={}&SAMLResponse={}",
                encode(&relay_state),
                encode(&saml_response)
            ))
            .send()
            .await
            .map_err(|_| IliasError::ConnectionFailed)?
            .cookies()
            .find(|c| c.name() == "PHPSESSID")
            .ok_or(IliasError::InvalidCredentials)?
            .value()
            .to_string();


        let mut session = self
            .token
            .write()
            .await;

        *session = Some(session_id);

        Ok(())
    }

    pub async fn get_html(&self, uri: &str) -> Result<Html, IliasError> {
        let token = self.token.read().await.as_ref().ok_or(IliasError::Unauthorized)?.to_string();
        let res = self.client
            .get(format!("{}/{}", ILIAS_URL, uri))
            .header(
                "cookie",
                format!("PHPSESSID={}", token),
            )
            .body(Body::from(""))
            .send()
            .await
            .map_err(|_| IliasError::ConnectionFailed)?;

        return if !res.status().is_success() {
            Err(IliasError::Unauthorized)
        } else {
            Ok(Html::parse_document(res.text().await.map_err(|_| IliasError::ParsingFailed)?.as_str()))
        };
    }

    pub async fn download_file(&self, uri: String, mut path: PathBuf, name: String) -> Result<(), IliasError> {
        let token = self.token.read().await.as_ref().ok_or(IliasError::Unauthorized)?.to_string();

        let client = reqwest::Client::new();
        let mut res = client.get(uri)
            .header(
                "cookie",
                format!("PHPSESSID={}", token),
            )
            .send()
            .await
            .map_err(|_| IliasError::ConnectionFailed)?;

        let ext = res
            .headers()
            .get("content-type")
            .ok_or(IliasError::UnsupportedFileFormat)?
            .to_str().map_err(|_| IliasError::UnsupportedFileFormat)?
            .split('/')
            .nth(1)
            .ok_or(IliasError::UnsupportedFileFormat)?
            .to_string();


        fs::create_dir_all(&path).await.map_err(|_| IliasError::IOOperationFailed)?;

        let mut final_path = path.clone();
        final_path.push(format!("{}.{}", name.clone(), ext.as_str()));


        path.push(format!(".{}.{}", name.clone(), ext.as_str()));

        if path.exists() {
            fs::remove_file(path.as_path()).await.map_err(|_| IliasError::IOOperationFailed)?;
        }

        let mut dest = fs::File::create(path.as_path()).await.map_err(|_| IliasError::IOOperationFailed)?;

        while let Some(chunk) = res.chunk().await.map_err(|_| IliasError::IOOperationFailed)? {
            dest.write(&chunk).await.map_err(|_| IliasError::IOOperationFailed)?;
        }

        dest.flush().await.map_err(|_| IliasError::IOOperationFailed)?;

        fs::rename(path.as_path(), final_path.as_path()).await.map_err(|_| IliasError::RenameFailed)?;

        println!("finished downloading {}", &name);

        Ok(())
    }
}
