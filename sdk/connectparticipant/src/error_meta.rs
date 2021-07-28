// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    ConflictException(crate::error::ConflictException),
    InternalServerException(crate::error::InternalServerException),
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    ThrottlingException(crate::error::ThrottlingException),
    ValidationException(crate::error::ValidationException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CompleteAttachmentUploadError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::CompleteAttachmentUploadError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CompleteAttachmentUploadErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CompleteAttachmentUploadErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CompleteAttachmentUploadErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CompleteAttachmentUploadErrorKind::ServiceQuotaExceededException(
                    inner,
                ) => Error::ServiceQuotaExceededException(inner),
                crate::error::CompleteAttachmentUploadErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CompleteAttachmentUploadErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CompleteAttachmentUploadErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::CreateParticipantConnectionError>> for Error {
    fn from(
        err: smithy_http::result::SdkError<crate::error::CreateParticipantConnectionError>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateParticipantConnectionErrorKind::AccessDeniedException(
                    inner,
                ) => Error::AccessDeniedException(inner),
                crate::error::CreateParticipantConnectionErrorKind::InternalServerException(
                    inner,
                ) => Error::InternalServerException(inner),
                crate::error::CreateParticipantConnectionErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateParticipantConnectionErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateParticipantConnectionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::DisconnectParticipantError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::DisconnectParticipantError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DisconnectParticipantErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DisconnectParticipantErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DisconnectParticipantErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DisconnectParticipantErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DisconnectParticipantErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetAttachmentError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetAttachmentError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetAttachmentErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetAttachmentErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetAttachmentErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetAttachmentErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetAttachmentErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::GetTranscriptError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::GetTranscriptError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetTranscriptErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetTranscriptErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetTranscriptErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetTranscriptErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetTranscriptErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SendEventError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SendEventError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendEventErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::SendEventErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::SendEventErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::SendEventErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::SendEventErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SendMessageError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SendMessageError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendMessageErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::SendMessageErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::SendMessageErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::SendMessageErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::SendMessageErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::StartAttachmentUploadError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::StartAttachmentUploadError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartAttachmentUploadErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::StartAttachmentUploadErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::StartAttachmentUploadErrorKind::ServiceQuotaExceededException(
                    inner,
                ) => Error::ServiceQuotaExceededException(inner),
                crate::error::StartAttachmentUploadErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::StartAttachmentUploadErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::StartAttachmentUploadErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}