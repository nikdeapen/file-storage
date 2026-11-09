use crate::system::r2::sdk_message;
use crate::{Error, Operation, StoragePath};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::{Client, Config};
use dashmap::DashMap;
use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime};

/// The global Cloudflare R2 `Client` map. `(account_id -> client)`.
static R2_CLIENTS: LazyLock<DashMap<String, Client>> = LazyLock::new(DashMap::new);

/// Runs the SDK's async calls behind the sync API.
pub(in crate::system::r2) static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
});

/// A Cloudflare R2 path.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub(crate) struct R2Path<'a> {
    pub(in crate::system::r2) path: &'a StoragePath,
    pub(in crate::system::r2) account_id: &'a str,
    pub(in crate::system::r2) bucket: &'a str,
    pub(in crate::system::r2) key: &'a str,
}

impl<'a> R2Path<'a> {
    //! Constants

    /// The `https://` path prefix. (all paths must use `https`)
    const HTTPS_PREFIX: &'static str = "https://";

    /// The Cloudflare R2 host after the account id.
    const R2_HOST: &'static str = ".r2.cloudflarestorage.com";

    /// The Cloudflare R2 host after the account id, with the `/` that ends it.
    const R2_PREFIX: &'static str = ".r2.cloudflarestorage.com/";
}

impl<'a> R2Path<'a> {
    //! Parse

    /// Checks if the `path` is a URL on the Cloudflare R2 host.
    ///
    /// # Note
    /// - It only checks the scheme and host. `parse` can still reject the path.
    pub(crate) fn is_r2_url(path: &str) -> bool {
        let rest: &str = match path.split_once("://") {
            Some(("http" | "https", rest)) => rest,
            _ => return false,
        };
        let host: &str = rest.split_once('/').map_or(rest, |(host, _)| host);
        host.ends_with(Self::R2_HOST)
    }

    /// Gets the base length of the Cloudflare R2 `path`.
    ///
    /// # Note
    /// - It is `None` if the `path` is not a valid Cloudflare R2 path.
    pub(crate) fn base_len(path: &str) -> Option<usize> {
        Self::parse_parts(path).map(|(_, _, key)| path.len() - key.len())
    }

    /// Splits the `path` into its account id, bucket and key.
    ///
    /// # Note
    /// - It is `None` if the `path` is not a valid Cloudflare R2 path.
    fn parse_parts(path: &str) -> Option<(&str, &str, &str)> {
        let s: &str = path.strip_prefix(Self::HTTPS_PREFIX)?;
        let (account_id, s) = s.split_at(s.find('.')?);
        let s: &str = s.strip_prefix(Self::R2_PREFIX)?;
        let (bucket, key) = s.split_once('/')?;
        if Self::is_account_id(account_id) && Self::is_bucket(bucket) {
            Some((account_id, bucket, key))
        } else {
            None
        }
    }

    /// Checks if the `account_id` can be an R2 account id.
    ///
    /// # Note
    /// - The account id goes into the endpoint's host. Anything but letters and digits could send
    ///   the signed requests to another host.
    fn is_account_id(account_id: &str) -> bool {
        !account_id.is_empty() && account_id.bytes().all(|b| b.is_ascii_alphanumeric())
    }

    /// Checks if the `bucket` can be an R2 bucket name.
    ///
    /// # Note
    /// - R2 bucket names hold only lowercase letters, digits and hyphens.
    fn is_bucket(bucket: &str) -> bool {
        !bucket.is_empty()
            && bucket
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    }
}

impl<'a> R2Path<'a> {
    //! Construction

    /// Creates the [R2Path].
    ///
    /// # Note
    /// - It is `None` if the `path` is not a valid Cloudflare R2 path.
    pub(crate) fn new(path: &'a StoragePath) -> Option<Self> {
        Self::parse_parts(path.path()).map(|(account_id, bucket, key)| Self {
            path,
            account_id,
            bucket,
            key,
        })
    }
}

impl<'a> R2Path<'a> {
    //! Client

    /// Gets the client for the `account_id`.
    pub(crate) async fn get_client(account_id: &str) -> Client {
        if let Some(client) = R2_CLIENTS.get(account_id) {
            return client.value().clone();
        }
        let client: Client = Self::create_client(account_id).await;
        R2_CLIENTS
            .entry(account_id.to_string())
            .or_insert(client)
            .clone()
    }

    /// Creates a client for the `account_id`.
    ///
    /// # Note
    /// - Every account's client loads the same default AWS credentials. An R2 token belongs to
    ///   one account. Only that account can be reached.
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

impl<'a> R2Path<'a> {
    //! Errors

    /// Creates an error for the `operation` from an SDK `error`.
    pub(in crate::system::r2) fn sdk_error<E>(self, operation: Operation, error: E) -> Error
    where
        E: ProvideErrorMetadata + std::error::Error,
    {
        Error::from_message(self.path.clone(), operation, sdk_message(&error))
    }
}

#[cfg(test)]
mod tests {
    use crate::R2Path;

    #[test]
    fn parse_parts() {
        let path: &str = "https://account.r2.cloudflarestorage.com/bucket/folder/file.txt";
        let parts: Option<(&str, &str, &str)> = R2Path::parse_parts(path);
        assert_eq!(parts, Some(("account", "bucket", "folder/file.txt")));

        let path: &str = "https://account.r2.cloudflarestorage.com/bucket/";
        assert_eq!(R2Path::parse_parts(path), Some(("account", "bucket", "")));
    }

    #[test]
    fn parse_parts_not_r2() {
        let paths: [&str; 8] = [
            // The bucket must end with a slash.
            "https://account.r2.cloudflarestorage.com/bucket",
            "https://account.example.com/bucket/",
            "http://account.r2.cloudflarestorage.com/bucket/",
            "/unix/path",
            // The account id must not name another host.
            "https://evil/x.r2.cloudflarestorage.com/bucket/",
            "https://.r2.cloudflarestorage.com/bucket/",
            // The bucket must be a valid R2 bucket name.
            "https://account.r2.cloudflarestorage.com/Bucket/",
            "https://account.r2.cloudflarestorage.com//",
        ];
        for path in paths {
            assert_eq!(R2Path::parse_parts(path), None, "{path}");
        }
    }

    #[test]
    fn is_r2_url() {
        for path in [
            "https://account.r2.cloudflarestorage.com/bucket/",
            "https://account.r2.cloudflarestorage.com/bucket",
            "https://account.r2.cloudflarestorage.com",
            "http://account.r2.cloudflarestorage.com/bucket/",
            "https://.r2.cloudflarestorage.com/bucket/",
            "https://account.r2.cloudflarestorage.com/Bucket/",
        ] {
            assert!(R2Path::is_r2_url(path), "{path}");
        }
        for path in [
            "https://example.com/bucket/",
            "https://evil/x.r2.cloudflarestorage.com/bucket/",
            "ftp://account.r2.cloudflarestorage.com/bucket/",
            "/unix/path",
            "",
        ] {
            assert!(!R2Path::is_r2_url(path), "{path}");
        }
    }

    #[test]
    fn base_len() {
        let path: &str = "https://account.r2.cloudflarestorage.com/bucket/file.txt";
        assert_eq!(R2Path::base_len(path), Some(path.len() - "file.txt".len()));
        assert_eq!(R2Path::base_len("/unix/path"), None);
    }
}
