use aws_sdk_s3::error::ProvideErrorMetadata;

/// Gets the message for an SDK `error`.
///
/// # Note
/// - It is R2's error code and message when R2 sent them. The SDK's own message is too vague,
///   such as "service error".
/// - Otherwise it is the error and each of its causes, such as a network failure.
pub(in crate::system::r2) fn sdk_message<E>(error: &E) -> String
where
    E: ProvideErrorMetadata + std::error::Error,
{
    match (error.code(), error.message()) {
        (Some(code), Some(message)) => format!("{}: {}", code, message),
        (Some(code), None) => code.to_string(),
        _ => {
            let mut message: String = error.to_string();
            let mut cause: Option<&dyn std::error::Error> = error.source();
            while let Some(next) = cause {
                message.push_str(": ");
                message.push_str(&next.to_string());
                cause = next.source();
            }
            message
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sdk_message;
    use aws_sdk_s3::error::{ErrorMetadata, ProvideErrorMetadata};
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    struct Cause;

    impl Display for Cause {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("connection reset")
        }
    }

    impl std::error::Error for Cause {}

    #[derive(Debug)]
    struct FakeError {
        meta: ErrorMetadata,
        cause: Option<Cause>,
    }

    impl Display for FakeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("service error")
        }
    }

    impl std::error::Error for FakeError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.cause
                .as_ref()
                .map(|cause| cause as &(dyn std::error::Error + 'static))
        }
    }

    impl ProvideErrorMetadata for FakeError {
        fn meta(&self) -> &ErrorMetadata {
            &self.meta
        }
    }

    #[test]
    fn code_and_message() {
        let meta: ErrorMetadata = ErrorMetadata::builder()
            .code("NoSuchBucket")
            .message("The bucket does not exist.")
            .build();
        let error: FakeError = FakeError { meta, cause: None };
        assert_eq!(
            sdk_message(&error),
            "NoSuchBucket: The bucket does not exist."
        );
    }

    #[test]
    fn code_only() {
        let meta: ErrorMetadata = ErrorMetadata::builder().code("NoSuchBucket").build();
        let error: FakeError = FakeError { meta, cause: None };
        assert_eq!(sdk_message(&error), "NoSuchBucket");
    }

    #[test]
    fn causes_without_code() {
        let meta: ErrorMetadata = ErrorMetadata::builder().build();
        let error: FakeError = FakeError {
            meta,
            cause: Some(Cause),
        };
        assert_eq!(sdk_message(&error), "service error: connection reset");
    }
}
