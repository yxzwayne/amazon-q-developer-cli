// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_usage_limits::_get_usage_limits_input::GetUsageLimitsInputBuilder;
pub use crate::operation::get_usage_limits::_get_usage_limits_output::GetUsageLimitsOutputBuilder;

impl crate::operation::get_usage_limits::builders::GetUsageLimitsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_usage_limits::GetUsageLimitsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_usage_limits::GetUsageLimitsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_usage_limits();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetUsageLimits`.
///
/// API to get current usage limits
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetUsageLimitsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_usage_limits::builders::GetUsageLimitsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_usage_limits::GetUsageLimitsOutput,
        crate::operation::get_usage_limits::GetUsageLimitsError,
    > for GetUsageLimitsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_usage_limits::GetUsageLimitsOutput,
            crate::operation::get_usage_limits::GetUsageLimitsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetUsageLimitsFluentBuilder {
    /// Creates a new `GetUsageLimitsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }

    /// Access the GetUsageLimits as a reference.
    pub fn as_input(&self) -> &crate::operation::get_usage_limits::builders::GetUsageLimitsInputBuilder {
        &self.inner
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_usage_limits::GetUsageLimitsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_usage_limits::GetUsageLimitsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_usage_limits::GetUsageLimits::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_usage_limits::GetUsageLimits::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_usage_limits::GetUsageLimitsOutput,
        crate::operation::get_usage_limits::GetUsageLimitsError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }

    pub(crate) fn config_override(
        mut self,
        config_override: impl ::std::convert::Into<crate::config::Builder>,
    ) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(
        &mut self,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> &mut Self {
        self.config_override = config_override;
        self
    }

    /// The ARN of the Q Developer profile. Required for enterprise customers, optional for Builder
    /// ID users.
    pub fn profile_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.profile_arn(input.into());
        self
    }

    /// The ARN of the Q Developer profile. Required for enterprise customers, optional for Builder
    /// ID users.
    pub fn set_profile_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_profile_arn(input);
        self
    }

    /// The ARN of the Q Developer profile. Required for enterprise customers, optional for Builder
    /// ID users.
    pub fn get_profile_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_profile_arn()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn resource_type(mut self, input: crate::types::ResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_resource_type(mut self, input: ::std::option::Option<crate::types::ResourceType>) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_resource_type(&self) -> &::std::option::Option<crate::types::ResourceType> {
        self.inner.get_resource_type()
    }
}
