use crate::StoragePath;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, Config};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime};

/// The global Cloudflare R2 `Client` map. `(account_id -> client)`.
static R2_CLIENTS: Lazy<DashMap<String, Client>> = Lazy::new(DashMap::new);

/// The runtime.
pub(in crate::system::r2) static RUNTIME: LazyLock<Runtime> =
    LazyLock::new(|| Builder::new_multi_thread().enable_all().build().unwrap());

/// A Cloudflare R2 path.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct R2Path<'a> {
    pub(crate) path: &'a StoragePath,
    pub(crate) account_id: &'a str,
    pub(crate) bucket: &'a str,
    pub(crate) key: &'a str,
}

impl<'a> R2Path<'a> {
    //! Constants

    /// The `https://` path prefix. (all paths must use `https`)
    pub const HTTPS_PREFIX: &'static str = "https://";

    /// The Cloudflare R2 prefix.
    pub const R2_PREFIX: &'static str = ".r2.cloudflarestorage.com/";
}

impl<'a> R2Path<'a> {
    //! Parse

    /// Parses the parts.
    ///
    /// Returns `Some(account_id, bucket, key)`.
    /// Returns `None` if the `path` is not a Cloudflare R2 path.
    fn parse_parts(path: &str) -> Option<(&str, &str, &str)> {
        let s: &str = path;
        if let Some(s) = s.strip_prefix(Self::HTTPS_PREFIX) {
            if let Some(dot) = s.as_bytes().iter().position(|c| *c == b'.') {
                let (account_id, s) = s.split_at(dot);
                if let Some(s) = s.strip_prefix(Self::R2_PREFIX) {
                    if let Some(slash) = s.as_bytes().iter().position(|c| *c == b'/') {
                        let bucket: &str = &s[..slash];
                        let key: &str = &s[(slash + 1)..];
                        return Some((account_id, bucket, key));
                    }
                }
            }
        }
        None
    }

    /// Gets the base length of the Cloudflare R2 `path`.
    ///
    /// Returns `Some(base_len)`.
    /// Returns `None` if the Cloudflare R2 path is invalid.
    pub fn base_len(path: &str) -> Option<usize> {
        if let Some((account_id, bucket, _key)) = Self::parse_parts(path) {
            Some(
                Self::HTTPS_PREFIX.len()
                    + account_id.len()
                    + Self::R2_PREFIX.len()
                    + bucket.len()
                    + 1,
            )
        } else {
            None
        }
    }

    /// Parses a Cloudflare R2 path from the `path`.
    pub fn from(path: &'a StoragePath) -> Option<Self> {
        if let Some((account_id, bucket, key)) = Self::parse_parts(path.path()) {
            Some(Self {
                path,
                account_id,
                bucket,
                key,
            })
        } else {
            None
        }
    }
}

impl<'a> R2Path<'a> {
    //! Path

    /// Gets the full path with the given object `key`.
    pub fn path_with_object_key(&self, key: &str) -> String {
        format!(
            "{}{}{}{}/{}",
            Self::HTTPS_PREFIX,
            self.account_id,
            Self::R2_PREFIX,
            self.bucket,
            key
        )
    }
}

impl<'a> R2Path<'a> {
    //! Client

    /// Gets the client for the `account_id`.
    pub(crate) async fn get_client(account_id: &str) -> Client {
        if let Some(client) = R2_CLIENTS.get(account_id) {
            client.value().clone()
        } else {
            let client: Client = Self::create_client(account_id).await;
            R2_CLIENTS.insert(account_id.to_string(), client.clone());
            client
        }
    }

    /// Creates a client for the `account_id`.
    async fn create_client(account_id: &str) -> Client {
        let endpoint: String = format!("https://{}.r2.cloudflarestorage.com", account_id);

        let config: Config = aws_sdk_s3::config::Builder::from(
            &aws_config::load_defaults(BehaviorVersion::latest()).await,
        )
        .endpoint_url(endpoint)
        .region(Region::new("auto"))
        .build();

        Client::from_conf(config)
    }
}
