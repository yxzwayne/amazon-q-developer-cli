// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_generate_completions_output_output_next_token(
    input: &crate::operation::generate_completions::GenerateCompletionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_available_customizations_output_output_next_token(
    input: &crate::operation::list_available_customizations::ListAvailableCustomizationsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_available_models_output_output_next_token(
    input: &crate::operation::list_available_models::ListAvailableModelsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_available_profiles_output_output_next_token(
    input: &crate::operation::list_available_profiles::ListAvailableProfilesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_code_analysis_findings_output_output_next_token(
    input: &crate::operation::list_code_analysis_findings::ListCodeAnalysisFindingsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_events_output_output_next_token(
    input: &crate::operation::list_events::ListEventsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_user_memory_entries_output_output_next_token(
    input: &crate::operation::list_user_memory_entries::ListUserMemoryEntriesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_workspace_metadata_output_output_next_token(
    input: &crate::operation::list_workspace_metadata::ListWorkspaceMetadataOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_available_customizations_output_output_customizations(
    input: crate::operation::list_available_customizations::ListAvailableCustomizationsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Customization>> {
    let input = input.customizations;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_available_models_output_output_models(
    input: crate::operation::list_available_models::ListAvailableModelsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Model>> {
    let input = input.models;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_available_profiles_output_output_profiles(
    input: crate::operation::list_available_profiles::ListAvailableProfilesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Profile>> {
    let input = input.profiles;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_events_output_output_events(
    input: crate::operation::list_events::ListEventsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Event>> {
    let input = input.events;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_user_memory_entries_output_output_memory_entries(
    input: crate::operation::list_user_memory_entries::ListUserMemoryEntriesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::MemoryEntry>> {
    let input = input.memory_entries;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_workspace_metadata_output_output_workspaces(
    input: crate::operation::list_workspace_metadata::ListWorkspaceMetadataOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::WorkspaceMetadata>> {
    let input = input.workspaces;
    ::std::option::Option::Some(input)
}
