// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutContactInformationOutput {}
impl std::fmt::Debug for PutContactInformationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutContactInformationOutput");
        formatter.finish()
    }
}
/// See [`PutContactInformationOutput`](crate::output::PutContactInformationOutput).
pub mod put_contact_information_output {

    /// A builder for [`PutContactInformationOutput`](crate::output::PutContactInformationOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PutContactInformationOutput`](crate::output::PutContactInformationOutput).
        pub fn build(self) -> crate::output::PutContactInformationOutput {
            crate::output::PutContactInformationOutput {}
        }
    }
}
impl PutContactInformationOutput {
    /// Creates a new builder-style object to manufacture [`PutContactInformationOutput`](crate::output::PutContactInformationOutput).
    pub fn builder() -> crate::output::put_contact_information_output::Builder {
        crate::output::put_contact_information_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetContactInformationOutput {
    /// <p>Contains the details of the primary contact information associated with an Amazon Web Services account.</p>
    #[doc(hidden)]
    pub contact_information: std::option::Option<crate::model::ContactInformation>,
}
impl GetContactInformationOutput {
    /// <p>Contains the details of the primary contact information associated with an Amazon Web Services account.</p>
    pub fn contact_information(&self) -> std::option::Option<&crate::model::ContactInformation> {
        self.contact_information.as_ref()
    }
}
impl std::fmt::Debug for GetContactInformationOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetContactInformationOutput");
        formatter.field("contact_information", &self.contact_information);
        formatter.finish()
    }
}
/// See [`GetContactInformationOutput`](crate::output::GetContactInformationOutput).
pub mod get_contact_information_output {

    /// A builder for [`GetContactInformationOutput`](crate::output::GetContactInformationOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) contact_information: std::option::Option<crate::model::ContactInformation>,
    }
    impl Builder {
        /// <p>Contains the details of the primary contact information associated with an Amazon Web Services account.</p>
        pub fn contact_information(mut self, input: crate::model::ContactInformation) -> Self {
            self.contact_information = Some(input);
            self
        }
        /// <p>Contains the details of the primary contact information associated with an Amazon Web Services account.</p>
        pub fn set_contact_information(
            mut self,
            input: std::option::Option<crate::model::ContactInformation>,
        ) -> Self {
            self.contact_information = input;
            self
        }
        /// Consumes the builder and constructs a [`GetContactInformationOutput`](crate::output::GetContactInformationOutput).
        pub fn build(self) -> crate::output::GetContactInformationOutput {
            crate::output::GetContactInformationOutput {
                contact_information: self.contact_information,
            }
        }
    }
}
impl GetContactInformationOutput {
    /// Creates a new builder-style object to manufacture [`GetContactInformationOutput`](crate::output::GetContactInformationOutput).
    pub fn builder() -> crate::output::get_contact_information_output::Builder {
        crate::output::get_contact_information_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutAlternateContactOutput {}
impl std::fmt::Debug for PutAlternateContactOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutAlternateContactOutput");
        formatter.finish()
    }
}
/// See [`PutAlternateContactOutput`](crate::output::PutAlternateContactOutput).
pub mod put_alternate_contact_output {

    /// A builder for [`PutAlternateContactOutput`](crate::output::PutAlternateContactOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PutAlternateContactOutput`](crate::output::PutAlternateContactOutput).
        pub fn build(self) -> crate::output::PutAlternateContactOutput {
            crate::output::PutAlternateContactOutput {}
        }
    }
}
impl PutAlternateContactOutput {
    /// Creates a new builder-style object to manufacture [`PutAlternateContactOutput`](crate::output::PutAlternateContactOutput).
    pub fn builder() -> crate::output::put_alternate_contact_output::Builder {
        crate::output::put_alternate_contact_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteAlternateContactOutput {}
impl std::fmt::Debug for DeleteAlternateContactOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteAlternateContactOutput");
        formatter.finish()
    }
}
/// See [`DeleteAlternateContactOutput`](crate::output::DeleteAlternateContactOutput).
pub mod delete_alternate_contact_output {

    /// A builder for [`DeleteAlternateContactOutput`](crate::output::DeleteAlternateContactOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteAlternateContactOutput`](crate::output::DeleteAlternateContactOutput).
        pub fn build(self) -> crate::output::DeleteAlternateContactOutput {
            crate::output::DeleteAlternateContactOutput {}
        }
    }
}
impl DeleteAlternateContactOutput {
    /// Creates a new builder-style object to manufacture [`DeleteAlternateContactOutput`](crate::output::DeleteAlternateContactOutput).
    pub fn builder() -> crate::output::delete_alternate_contact_output::Builder {
        crate::output::delete_alternate_contact_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetAlternateContactOutput {
    /// <p>A structure that contains the details for the specified alternate contact.</p>
    #[doc(hidden)]
    pub alternate_contact: std::option::Option<crate::model::AlternateContact>,
}
impl GetAlternateContactOutput {
    /// <p>A structure that contains the details for the specified alternate contact.</p>
    pub fn alternate_contact(&self) -> std::option::Option<&crate::model::AlternateContact> {
        self.alternate_contact.as_ref()
    }
}
impl std::fmt::Debug for GetAlternateContactOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetAlternateContactOutput");
        formatter.field("alternate_contact", &self.alternate_contact);
        formatter.finish()
    }
}
/// See [`GetAlternateContactOutput`](crate::output::GetAlternateContactOutput).
pub mod get_alternate_contact_output {

    /// A builder for [`GetAlternateContactOutput`](crate::output::GetAlternateContactOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) alternate_contact: std::option::Option<crate::model::AlternateContact>,
    }
    impl Builder {
        /// <p>A structure that contains the details for the specified alternate contact.</p>
        pub fn alternate_contact(mut self, input: crate::model::AlternateContact) -> Self {
            self.alternate_contact = Some(input);
            self
        }
        /// <p>A structure that contains the details for the specified alternate contact.</p>
        pub fn set_alternate_contact(
            mut self,
            input: std::option::Option<crate::model::AlternateContact>,
        ) -> Self {
            self.alternate_contact = input;
            self
        }
        /// Consumes the builder and constructs a [`GetAlternateContactOutput`](crate::output::GetAlternateContactOutput).
        pub fn build(self) -> crate::output::GetAlternateContactOutput {
            crate::output::GetAlternateContactOutput {
                alternate_contact: self.alternate_contact,
            }
        }
    }
}
impl GetAlternateContactOutput {
    /// Creates a new builder-style object to manufacture [`GetAlternateContactOutput`](crate::output::GetAlternateContactOutput).
    pub fn builder() -> crate::output::get_alternate_contact_output::Builder {
        crate::output::get_alternate_contact_output::Builder::default()
    }
}
