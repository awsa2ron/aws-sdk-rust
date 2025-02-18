// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UpdateHostOutput {}
impl std::fmt::Debug for UpdateHostOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UpdateHostOutput");
        formatter.finish()
    }
}
/// See [`UpdateHostOutput`](crate::output::UpdateHostOutput).
pub mod update_host_output {

    /// A builder for [`UpdateHostOutput`](crate::output::UpdateHostOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UpdateHostOutput`](crate::output::UpdateHostOutput).
        pub fn build(self) -> crate::output::UpdateHostOutput {
            crate::output::UpdateHostOutput {}
        }
    }
}
impl UpdateHostOutput {
    /// Creates a new builder-style object to manufacture [`UpdateHostOutput`](crate::output::UpdateHostOutput).
    pub fn builder() -> crate::output::update_host_output::Builder {
        crate::output::update_host_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UntagResourceOutput {}
impl std::fmt::Debug for UntagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UntagResourceOutput");
        formatter.finish()
    }
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {

    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TagResourceOutput {}
impl std::fmt::Debug for TagResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TagResourceOutput");
        formatter.finish()
    }
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {

    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListTagsForResourceOutput {
    /// <p>A list of tag key and value pairs associated with the specified resource.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
}
impl ListTagsForResourceOutput {
    /// <p>A list of tag key and value pairs associated with the specified resource.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::model::Tag]> {
        self.tags.as_deref()
    }
}
impl std::fmt::Debug for ListTagsForResourceOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListTagsForResourceOutput");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {

    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    }
    impl Builder {
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>A list of tag key and value pairs associated with the specified resource.</p>
        pub fn tags(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input);
            self.tags = Some(v);
            self
        }
        /// <p>A list of tag key and value pairs associated with the specified resource.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput { tags: self.tags }
        }
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListHostsOutput {
    /// <p>A list of hosts and the details for each host, such as status, endpoint, and provider type.</p>
    #[doc(hidden)]
    pub hosts: std::option::Option<std::vec::Vec<crate::model::Host>>,
    /// <p>A token that can be used in the next <code>ListHosts</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListHostsOutput {
    /// <p>A list of hosts and the details for each host, such as status, endpoint, and provider type.</p>
    pub fn hosts(&self) -> std::option::Option<&[crate::model::Host]> {
        self.hosts.as_deref()
    }
    /// <p>A token that can be used in the next <code>ListHosts</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListHostsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListHostsOutput");
        formatter.field("hosts", &self.hosts);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListHostsOutput`](crate::output::ListHostsOutput).
pub mod list_hosts_output {

    /// A builder for [`ListHostsOutput`](crate::output::ListHostsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) hosts: std::option::Option<std::vec::Vec<crate::model::Host>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `hosts`.
        ///
        /// To override the contents of this collection use [`set_hosts`](Self::set_hosts).
        ///
        /// <p>A list of hosts and the details for each host, such as status, endpoint, and provider type.</p>
        pub fn hosts(mut self, input: crate::model::Host) -> Self {
            let mut v = self.hosts.unwrap_or_default();
            v.push(input);
            self.hosts = Some(v);
            self
        }
        /// <p>A list of hosts and the details for each host, such as status, endpoint, and provider type.</p>
        pub fn set_hosts(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Host>>,
        ) -> Self {
            self.hosts = input;
            self
        }
        /// <p>A token that can be used in the next <code>ListHosts</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token that can be used in the next <code>ListHosts</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListHostsOutput`](crate::output::ListHostsOutput).
        pub fn build(self) -> crate::output::ListHostsOutput {
            crate::output::ListHostsOutput {
                hosts: self.hosts,
                next_token: self.next_token,
            }
        }
    }
}
impl ListHostsOutput {
    /// Creates a new builder-style object to manufacture [`ListHostsOutput`](crate::output::ListHostsOutput).
    pub fn builder() -> crate::output::list_hosts_output::Builder {
        crate::output::list_hosts_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListConnectionsOutput {
    /// <p>A list of connections and the details for each connection, such as status, owner, and provider type.</p>
    #[doc(hidden)]
    pub connections: std::option::Option<std::vec::Vec<crate::model::Connection>>,
    /// <p>A token that can be used in the next <code>ListConnections</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListConnectionsOutput {
    /// <p>A list of connections and the details for each connection, such as status, owner, and provider type.</p>
    pub fn connections(&self) -> std::option::Option<&[crate::model::Connection]> {
        self.connections.as_deref()
    }
    /// <p>A token that can be used in the next <code>ListConnections</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListConnectionsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListConnectionsOutput");
        formatter.field("connections", &self.connections);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListConnectionsOutput`](crate::output::ListConnectionsOutput).
pub mod list_connections_output {

    /// A builder for [`ListConnectionsOutput`](crate::output::ListConnectionsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) connections: std::option::Option<std::vec::Vec<crate::model::Connection>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `connections`.
        ///
        /// To override the contents of this collection use [`set_connections`](Self::set_connections).
        ///
        /// <p>A list of connections and the details for each connection, such as status, owner, and provider type.</p>
        pub fn connections(mut self, input: crate::model::Connection) -> Self {
            let mut v = self.connections.unwrap_or_default();
            v.push(input);
            self.connections = Some(v);
            self
        }
        /// <p>A list of connections and the details for each connection, such as status, owner, and provider type.</p>
        pub fn set_connections(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Connection>>,
        ) -> Self {
            self.connections = input;
            self
        }
        /// <p>A token that can be used in the next <code>ListConnections</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token that can be used in the next <code>ListConnections</code> call. To view all items in the list, continue to call this operation with each subsequent token until no more <code>nextToken</code> values are returned.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListConnectionsOutput`](crate::output::ListConnectionsOutput).
        pub fn build(self) -> crate::output::ListConnectionsOutput {
            crate::output::ListConnectionsOutput {
                connections: self.connections,
                next_token: self.next_token,
            }
        }
    }
}
impl ListConnectionsOutput {
    /// Creates a new builder-style object to manufacture [`ListConnectionsOutput`](crate::output::ListConnectionsOutput).
    pub fn builder() -> crate::output::list_connections_output::Builder {
        crate::output::list_connections_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetHostOutput {
    /// <p>The name of the requested host.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>The status of the requested host.</p>
    #[doc(hidden)]
    pub status: std::option::Option<std::string::String>,
    /// <p>The provider type of the requested host, such as GitHub Enterprise Server.</p>
    #[doc(hidden)]
    pub provider_type: std::option::Option<crate::model::ProviderType>,
    /// <p>The endpoint of the infrastructure represented by the requested host.</p>
    #[doc(hidden)]
    pub provider_endpoint: std::option::Option<std::string::String>,
    /// <p>The VPC configuration of the requested host.</p>
    #[doc(hidden)]
    pub vpc_configuration: std::option::Option<crate::model::VpcConfiguration>,
}
impl GetHostOutput {
    /// <p>The name of the requested host.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The status of the requested host.</p>
    pub fn status(&self) -> std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The provider type of the requested host, such as GitHub Enterprise Server.</p>
    pub fn provider_type(&self) -> std::option::Option<&crate::model::ProviderType> {
        self.provider_type.as_ref()
    }
    /// <p>The endpoint of the infrastructure represented by the requested host.</p>
    pub fn provider_endpoint(&self) -> std::option::Option<&str> {
        self.provider_endpoint.as_deref()
    }
    /// <p>The VPC configuration of the requested host.</p>
    pub fn vpc_configuration(&self) -> std::option::Option<&crate::model::VpcConfiguration> {
        self.vpc_configuration.as_ref()
    }
}
impl std::fmt::Debug for GetHostOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetHostOutput");
        formatter.field("name", &self.name);
        formatter.field("status", &self.status);
        formatter.field("provider_type", &self.provider_type);
        formatter.field("provider_endpoint", &self.provider_endpoint);
        formatter.field("vpc_configuration", &self.vpc_configuration);
        formatter.finish()
    }
}
/// See [`GetHostOutput`](crate::output::GetHostOutput).
pub mod get_host_output {

    /// A builder for [`GetHostOutput`](crate::output::GetHostOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<std::string::String>,
        pub(crate) provider_type: std::option::Option<crate::model::ProviderType>,
        pub(crate) provider_endpoint: std::option::Option<std::string::String>,
        pub(crate) vpc_configuration: std::option::Option<crate::model::VpcConfiguration>,
    }
    impl Builder {
        /// <p>The name of the requested host.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name of the requested host.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The status of the requested host.</p>
        pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
            self.status = Some(input.into());
            self
        }
        /// <p>The status of the requested host.</p>
        pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status = input;
            self
        }
        /// <p>The provider type of the requested host, such as GitHub Enterprise Server.</p>
        pub fn provider_type(mut self, input: crate::model::ProviderType) -> Self {
            self.provider_type = Some(input);
            self
        }
        /// <p>The provider type of the requested host, such as GitHub Enterprise Server.</p>
        pub fn set_provider_type(
            mut self,
            input: std::option::Option<crate::model::ProviderType>,
        ) -> Self {
            self.provider_type = input;
            self
        }
        /// <p>The endpoint of the infrastructure represented by the requested host.</p>
        pub fn provider_endpoint(mut self, input: impl Into<std::string::String>) -> Self {
            self.provider_endpoint = Some(input.into());
            self
        }
        /// <p>The endpoint of the infrastructure represented by the requested host.</p>
        pub fn set_provider_endpoint(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.provider_endpoint = input;
            self
        }
        /// <p>The VPC configuration of the requested host.</p>
        pub fn vpc_configuration(mut self, input: crate::model::VpcConfiguration) -> Self {
            self.vpc_configuration = Some(input);
            self
        }
        /// <p>The VPC configuration of the requested host.</p>
        pub fn set_vpc_configuration(
            mut self,
            input: std::option::Option<crate::model::VpcConfiguration>,
        ) -> Self {
            self.vpc_configuration = input;
            self
        }
        /// Consumes the builder and constructs a [`GetHostOutput`](crate::output::GetHostOutput).
        pub fn build(self) -> crate::output::GetHostOutput {
            crate::output::GetHostOutput {
                name: self.name,
                status: self.status,
                provider_type: self.provider_type,
                provider_endpoint: self.provider_endpoint,
                vpc_configuration: self.vpc_configuration,
            }
        }
    }
}
impl GetHostOutput {
    /// Creates a new builder-style object to manufacture [`GetHostOutput`](crate::output::GetHostOutput).
    pub fn builder() -> crate::output::get_host_output::Builder {
        crate::output::get_host_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetConnectionOutput {
    /// <p>The connection details, such as status, owner, and provider type.</p>
    #[doc(hidden)]
    pub connection: std::option::Option<crate::model::Connection>,
}
impl GetConnectionOutput {
    /// <p>The connection details, such as status, owner, and provider type.</p>
    pub fn connection(&self) -> std::option::Option<&crate::model::Connection> {
        self.connection.as_ref()
    }
}
impl std::fmt::Debug for GetConnectionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetConnectionOutput");
        formatter.field("connection", &self.connection);
        formatter.finish()
    }
}
/// See [`GetConnectionOutput`](crate::output::GetConnectionOutput).
pub mod get_connection_output {

    /// A builder for [`GetConnectionOutput`](crate::output::GetConnectionOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) connection: std::option::Option<crate::model::Connection>,
    }
    impl Builder {
        /// <p>The connection details, such as status, owner, and provider type.</p>
        pub fn connection(mut self, input: crate::model::Connection) -> Self {
            self.connection = Some(input);
            self
        }
        /// <p>The connection details, such as status, owner, and provider type.</p>
        pub fn set_connection(
            mut self,
            input: std::option::Option<crate::model::Connection>,
        ) -> Self {
            self.connection = input;
            self
        }
        /// Consumes the builder and constructs a [`GetConnectionOutput`](crate::output::GetConnectionOutput).
        pub fn build(self) -> crate::output::GetConnectionOutput {
            crate::output::GetConnectionOutput {
                connection: self.connection,
            }
        }
    }
}
impl GetConnectionOutput {
    /// Creates a new builder-style object to manufacture [`GetConnectionOutput`](crate::output::GetConnectionOutput).
    pub fn builder() -> crate::output::get_connection_output::Builder {
        crate::output::get_connection_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteHostOutput {}
impl std::fmt::Debug for DeleteHostOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteHostOutput");
        formatter.finish()
    }
}
/// See [`DeleteHostOutput`](crate::output::DeleteHostOutput).
pub mod delete_host_output {

    /// A builder for [`DeleteHostOutput`](crate::output::DeleteHostOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteHostOutput`](crate::output::DeleteHostOutput).
        pub fn build(self) -> crate::output::DeleteHostOutput {
            crate::output::DeleteHostOutput {}
        }
    }
}
impl DeleteHostOutput {
    /// Creates a new builder-style object to manufacture [`DeleteHostOutput`](crate::output::DeleteHostOutput).
    pub fn builder() -> crate::output::delete_host_output::Builder {
        crate::output::delete_host_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteConnectionOutput {}
impl std::fmt::Debug for DeleteConnectionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteConnectionOutput");
        formatter.finish()
    }
}
/// See [`DeleteConnectionOutput`](crate::output::DeleteConnectionOutput).
pub mod delete_connection_output {

    /// A builder for [`DeleteConnectionOutput`](crate::output::DeleteConnectionOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteConnectionOutput`](crate::output::DeleteConnectionOutput).
        pub fn build(self) -> crate::output::DeleteConnectionOutput {
            crate::output::DeleteConnectionOutput {}
        }
    }
}
impl DeleteConnectionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteConnectionOutput`](crate::output::DeleteConnectionOutput).
    pub fn builder() -> crate::output::delete_connection_output::Builder {
        crate::output::delete_connection_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateHostOutput {
    /// <p>The Amazon Resource Name (ARN) of the host to be created.</p>
    #[doc(hidden)]
    pub host_arn: std::option::Option<std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
}
impl CreateHostOutput {
    /// <p>The Amazon Resource Name (ARN) of the host to be created.</p>
    pub fn host_arn(&self) -> std::option::Option<&str> {
        self.host_arn.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn tags(&self) -> std::option::Option<&[crate::model::Tag]> {
        self.tags.as_deref()
    }
}
impl std::fmt::Debug for CreateHostOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateHostOutput");
        formatter.field("host_arn", &self.host_arn);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`CreateHostOutput`](crate::output::CreateHostOutput).
pub mod create_host_output {

    /// A builder for [`CreateHostOutput`](crate::output::CreateHostOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) host_arn: std::option::Option<std::string::String>,
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the host to be created.</p>
        pub fn host_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.host_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the host to be created.</p>
        pub fn set_host_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.host_arn = input;
            self
        }
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        pub fn tags(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input);
            self.tags = Some(v);
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateHostOutput`](crate::output::CreateHostOutput).
        pub fn build(self) -> crate::output::CreateHostOutput {
            crate::output::CreateHostOutput {
                host_arn: self.host_arn,
                tags: self.tags,
            }
        }
    }
}
impl CreateHostOutput {
    /// Creates a new builder-style object to manufacture [`CreateHostOutput`](crate::output::CreateHostOutput).
    pub fn builder() -> crate::output::create_host_output::Builder {
        crate::output::create_host_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateConnectionOutput {
    /// <p>The Amazon Resource Name (ARN) of the connection to be created. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note>
    /// <p>The ARN is never reused if the connection is deleted.</p>
    /// </note>
    #[doc(hidden)]
    pub connection_arn: std::option::Option<std::string::String>,
    /// <p>Specifies the tags applied to the resource.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
}
impl CreateConnectionOutput {
    /// <p>The Amazon Resource Name (ARN) of the connection to be created. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note>
    /// <p>The ARN is never reused if the connection is deleted.</p>
    /// </note>
    pub fn connection_arn(&self) -> std::option::Option<&str> {
        self.connection_arn.as_deref()
    }
    /// <p>Specifies the tags applied to the resource.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::model::Tag]> {
        self.tags.as_deref()
    }
}
impl std::fmt::Debug for CreateConnectionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateConnectionOutput");
        formatter.field("connection_arn", &self.connection_arn);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`CreateConnectionOutput`](crate::output::CreateConnectionOutput).
pub mod create_connection_output {

    /// A builder for [`CreateConnectionOutput`](crate::output::CreateConnectionOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) connection_arn: std::option::Option<std::string::String>,
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the connection to be created. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note>
        /// <p>The ARN is never reused if the connection is deleted.</p>
        /// </note>
        pub fn connection_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.connection_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the connection to be created. The ARN is used as the connection reference when the connection is shared between AWS services.</p> <note>
        /// <p>The ARN is never reused if the connection is deleted.</p>
        /// </note>
        pub fn set_connection_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.connection_arn = input;
            self
        }
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>Specifies the tags applied to the resource.</p>
        pub fn tags(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input);
            self.tags = Some(v);
            self
        }
        /// <p>Specifies the tags applied to the resource.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateConnectionOutput`](crate::output::CreateConnectionOutput).
        pub fn build(self) -> crate::output::CreateConnectionOutput {
            crate::output::CreateConnectionOutput {
                connection_arn: self.connection_arn,
                tags: self.tags,
            }
        }
    }
}
impl CreateConnectionOutput {
    /// Creates a new builder-style object to manufacture [`CreateConnectionOutput`](crate::output::CreateConnectionOutput).
    pub fn builder() -> crate::output::create_connection_output::Builder {
        crate::output::create_connection_output::Builder::default()
    }
}
