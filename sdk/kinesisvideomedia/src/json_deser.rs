// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_generic_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<smithy_types::Error, smithy_json::deserialize::Error> {
    crate::json_errors::parse_generic_error(response)
}

pub fn deser_structure_client_limit_exceeded_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::client_limit_exceeded_exception::Builder,
) -> Result<crate::error::client_limit_exceeded_exception::Builder, smithy_json::deserialize::Error>
{
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_connection_limit_exceeded_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::connection_limit_exceeded_exception::Builder,
) -> Result<
    crate::error::connection_limit_exceeded_exception::Builder,
    smithy_json::deserialize::Error,
> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_invalid_argument_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::invalid_argument_exception::Builder,
) -> Result<crate::error::invalid_argument_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_invalid_endpoint_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::invalid_endpoint_exception::Builder,
) -> Result<crate::error::invalid_endpoint_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_not_authorized_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::not_authorized_exception::Builder,
) -> Result<crate::error::not_authorized_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn deser_structure_resource_not_found_exceptionjson_err(
    input: &[u8],
    mut builder: crate::error::resource_not_found_exception::Builder,
) -> Result<crate::error::resource_not_found_exception::Builder, smithy_json::deserialize::Error> {
    let mut tokens_owned =
        smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(input))
            .peekable();
    let tokens = &mut tokens_owned;
    smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                        );
                    }
                    _ => smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            _ => {
                return Err(smithy_json::deserialize::Error::custom(
                    "expected object key or end object",
                ))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(smithy_json::deserialize::Error::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}