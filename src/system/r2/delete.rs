use crate::system::R2Path;
use crate::Error;
use crate::Operation::Delete;

impl<'a> R2Path<'a> {
    //! Delete

    /// See `FilePath::delete`.
    pub fn delete(&self) -> Result<(), Error> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.delete_async())
    }

    /// See `R2Path::delete`.
    pub async fn delete_async(&self) -> Result<(), Error> {
        let response = Self::get_client(self.account_id)
            .await
            .delete_object()
            .bucket(self.bucket)
            .key(self.key)
            .send()
            .await;
        match response {
            Ok(_response) => Ok(()),
            Err(error) => Err(Error::from_cause(
                self.path.clone(),
                Delete,
                std::io::Error::new(std::io::ErrorKind::Other, error),
            )),
        }
    }
}
