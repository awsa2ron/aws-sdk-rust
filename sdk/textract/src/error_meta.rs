// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    BadDocumentException(crate::error::BadDocumentException),
    DocumentTooLargeException(crate::error::DocumentTooLargeException),
    HumanLoopQuotaExceededException(crate::error::HumanLoopQuotaExceededException),
    IdempotentParameterMismatchException(crate::error::IdempotentParameterMismatchException),
    InternalServerError(crate::error::InternalServerError),
    InvalidJobIdException(crate::error::InvalidJobIdException),
    InvalidKmsKeyException(crate::error::InvalidKmsKeyException),
    InvalidParameterException(crate::error::InvalidParameterException),
    InvalidS3ObjectException(crate::error::InvalidS3ObjectException),
    LimitExceededException(crate::error::LimitExceededException),
    ProvisionedThroughputExceededException(crate::error::ProvisionedThroughputExceededException),
    ThrottlingException(crate::error::ThrottlingException),
    UnsupportedDocumentException(crate::error::UnsupportedDocumentException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::BadDocumentException(inner) => inner.fmt(f),
            Error::DocumentTooLargeException(inner) => inner.fmt(f),
            Error::HumanLoopQuotaExceededException(inner) => inner.fmt(f),
            Error::IdempotentParameterMismatchException(inner) => inner.fmt(f),
            Error::InternalServerError(inner) => inner.fmt(f),
            Error::InvalidJobIdException(inner) => inner.fmt(f),
            Error::InvalidKmsKeyException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::InvalidS3ObjectException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ProvisionedThroughputExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::UnsupportedDocumentException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::AnalyzeDocumentError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::AnalyzeDocumentError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AnalyzeDocumentErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::AnalyzeDocumentErrorKind::BadDocumentException(inner) => {
                    Error::BadDocumentException(inner)
                }
                crate::error::AnalyzeDocumentErrorKind::DocumentTooLargeException(inner) => {
                    Error::DocumentTooLargeException(inner)
                }
                crate::error::AnalyzeDocumentErrorKind::HumanLoopQuotaExceededException(inner) => {
                    Error::HumanLoopQuotaExceededException(inner)
                }
                crate::error::AnalyzeDocumentErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::AnalyzeDocumentErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::AnalyzeDocumentErrorKind::InvalidS3ObjectException(inner) => {
                    Error::InvalidS3ObjectException(inner)
                }
                crate::error::AnalyzeDocumentErrorKind::ProvisionedThroughputExceededException(
                    inner,
                ) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::AnalyzeDocumentErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::AnalyzeDocumentErrorKind::UnsupportedDocumentException(inner) => {
                    Error::UnsupportedDocumentException(inner)
                }
                crate::error::AnalyzeDocumentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DetectDocumentTextError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DetectDocumentTextError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DetectDocumentTextErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DetectDocumentTextErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::DetectDocumentTextErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::DetectDocumentTextErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::DetectDocumentTextErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::DetectDocumentTextErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::DetectDocumentTextErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::DetectDocumentTextErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DetectDocumentTextErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::DetectDocumentTextErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetDocumentAnalysisError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetDocumentAnalysisError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetDocumentAnalysisErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetDocumentAnalysisErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::GetDocumentAnalysisErrorKind::InvalidJobIdException(inner) => Error::InvalidJobIdException(inner),
                crate::error::GetDocumentAnalysisErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::GetDocumentAnalysisErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::GetDocumentAnalysisErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::GetDocumentAnalysisErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetDocumentAnalysisErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetDocumentTextDetectionError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::GetDocumentTextDetectionError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetDocumentTextDetectionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::GetDocumentTextDetectionErrorKind::InvalidJobIdException(inner) => Error::InvalidJobIdException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetDocumentTextDetectionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartDocumentAnalysisError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartDocumentAnalysisError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartDocumentAnalysisErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::StartDocumentAnalysisErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::StartDocumentAnalysisErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::StartDocumentAnalysisErrorKind::IdempotentParameterMismatchException(inner) => Error::IdempotentParameterMismatchException(inner),
                crate::error::StartDocumentAnalysisErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::StartDocumentAnalysisErrorKind::InvalidKmsKeyException(inner) => Error::InvalidKmsKeyException(inner),
                crate::error::StartDocumentAnalysisErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::StartDocumentAnalysisErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::StartDocumentAnalysisErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::StartDocumentAnalysisErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::StartDocumentAnalysisErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::StartDocumentAnalysisErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::StartDocumentAnalysisErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartDocumentTextDetectionError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartDocumentTextDetectionError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartDocumentTextDetectionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::BadDocumentException(inner) => Error::BadDocumentException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::DocumentTooLargeException(inner) => Error::DocumentTooLargeException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::IdempotentParameterMismatchException(inner) => Error::IdempotentParameterMismatchException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::InternalServerError(inner) => Error::InternalServerError(inner),
                crate::error::StartDocumentTextDetectionErrorKind::InvalidKmsKeyException(inner) => Error::InvalidKmsKeyException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::InvalidS3ObjectException(inner) => Error::InvalidS3ObjectException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::ProvisionedThroughputExceededException(inner) => Error::ProvisionedThroughputExceededException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::UnsupportedDocumentException(inner) => Error::UnsupportedDocumentException(inner),
                crate::error::StartDocumentTextDetectionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}