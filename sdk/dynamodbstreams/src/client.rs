// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(std::fmt::Debug)]
pub(crate) struct Handle<C = aws_hyper::DynConnector> {
    client: aws_hyper::Client<C>,
    conf: crate::Config,
}

#[derive(Clone, std::fmt::Debug)]
pub struct Client<C = aws_hyper::DynConnector> {
    handle: std::sync::Arc<Handle<C>>,
}
impl<C> Client<C> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_env() -> Self {
        Self::from_conf(crate::Config::builder().build())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl<C> Client<C>
where
    C: aws_hyper::SmithyConnector,
{
    pub fn describe_stream(&self) -> fluent_builders::DescribeStream<C> {
        fluent_builders::DescribeStream::new(self.handle.clone())
    }
    pub fn get_records(&self) -> fluent_builders::GetRecords<C> {
        fluent_builders::GetRecords::new(self.handle.clone())
    }
    pub fn get_shard_iterator(&self) -> fluent_builders::GetShardIterator<C> {
        fluent_builders::GetShardIterator::new(self.handle.clone())
    }
    pub fn list_streams(&self) -> fluent_builders::ListStreams<C> {
        fluent_builders::ListStreams::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct DescribeStream<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::describe_stream_input::Builder,
    }
    impl<C> DescribeStream<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeStreamOutput,
            smithy_http::result::SdkError<crate::error::DescribeStreamError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name (ARN) for the stream.</p>
        pub fn stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.stream_arn(input);
            self
        }
        pub fn set_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_stream_arn(input);
            self
        }
        /// <p>The maximum number of shard objects to return. The upper limit is 100.</p>
        pub fn limit(mut self, input: i32) -> Self {
            self.inner = self.inner.limit(input);
            self
        }
        pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_limit(input);
            self
        }
        /// <p>The shard ID of the first item that this operation will evaluate. Use the value that was
        /// returned for <code>LastEvaluatedShardId</code> in the previous operation. </p>
        pub fn exclusive_start_shard_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.exclusive_start_shard_id(input);
            self
        }
        pub fn set_exclusive_start_shard_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_exclusive_start_shard_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetRecords<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_records_input::Builder,
    }
    impl<C> GetRecords<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetRecordsOutput,
            smithy_http::result::SdkError<crate::error::GetRecordsError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>A shard iterator that was retrieved from a previous GetShardIterator operation. This iterator can be used to access the stream records in this shard.</p>
        pub fn shard_iterator(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.shard_iterator(input);
            self
        }
        pub fn set_shard_iterator(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_shard_iterator(input);
            self
        }
        /// <p>The maximum number of records to return from the shard. The upper limit is 1000.</p>
        pub fn limit(mut self, input: i32) -> Self {
            self.inner = self.inner.limit(input);
            self
        }
        pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_limit(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetShardIterator<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_shard_iterator_input::Builder,
    }
    impl<C> GetShardIterator<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetShardIteratorOutput,
            smithy_http::result::SdkError<crate::error::GetShardIteratorError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The Amazon Resource Name (ARN) for the stream.</p>
        pub fn stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.stream_arn(input);
            self
        }
        pub fn set_stream_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_stream_arn(input);
            self
        }
        /// <p>The identifier of the shard. The iterator will be returned for this shard ID.</p>
        pub fn shard_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.shard_id(input);
            self
        }
        pub fn set_shard_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_shard_id(input);
            self
        }
        /// <p>Determines how the shard iterator is used to start reading stream records from the shard:</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>AT_SEQUENCE_NUMBER</code> - Start reading exactly from the position denoted by a
        /// specific sequence number.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>AFTER_SEQUENCE_NUMBER</code> - Start reading right after the position denoted by a
        /// specific sequence number.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>TRIM_HORIZON</code> - Start reading at the last (untrimmed) stream record, which is
        /// the oldest record in the shard. In DynamoDB Streams, there is a 24 hour limit on data retention.
        /// Stream records whose age exceeds this limit are subject to removal (trimming) from the
        /// stream.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>LATEST</code> - Start reading just after the most recent stream record in the
        /// shard, so that you always read the most recent data in the shard.</p>
        /// </li>
        /// </ul>
        pub fn shard_iterator_type(mut self, input: crate::model::ShardIteratorType) -> Self {
            self.inner = self.inner.shard_iterator_type(input);
            self
        }
        pub fn set_shard_iterator_type(
            mut self,
            input: std::option::Option<crate::model::ShardIteratorType>,
        ) -> Self {
            self.inner = self.inner.set_shard_iterator_type(input);
            self
        }
        /// <p>The sequence number of a stream record in the shard from which to start reading.</p>
        pub fn sequence_number(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.sequence_number(input);
            self
        }
        pub fn set_sequence_number(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_sequence_number(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListStreams<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_streams_input::Builder,
    }
    impl<C> ListStreams<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListStreamsOutput,
            smithy_http::result::SdkError<crate::error::ListStreamsError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>If this parameter is provided, then only the streams associated with this table name are returned.</p>
        pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.table_name(input);
            self
        }
        pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_table_name(input);
            self
        }
        /// <p>The maximum number of streams to return. The upper limit is 100.</p>
        pub fn limit(mut self, input: i32) -> Self {
            self.inner = self.inner.limit(input);
            self
        }
        pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_limit(input);
            self
        }
        /// <p>The ARN (Amazon Resource Name) of the first item that this operation will evaluate. Use the
        /// value that was returned for <code>LastEvaluatedStreamArn</code> in the previous operation.
        /// </p>
        pub fn exclusive_start_stream_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.exclusive_start_stream_arn(input);
            self
        }
        pub fn set_exclusive_start_stream_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_exclusive_start_stream_arn(input);
            self
        }
    }
}