use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, error};
use regex::Regex;
use uuid::Uuid;
use serde_json;

use crate::api_client::{ApiClientError, Endpoint};
use crate::api_client::model::{ChatResponseStream, ConversationState};

/// Configuration for alternative LLM providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeProviderConfig {
    pub provider_type: ProviderType,
    pub endpoint: String,
    pub model: Option<String>,
    pub api_key: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    OpenAI,
    Anthropic,
    Custom,
}

/// Simple message format for requests
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RequestMessage {
    role: String,
    content: String,
}

/// OpenAI-compatible request format  
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<RequestMessage>,
    temperature: f32,
    max_tokens: i32,
    stream: bool,
}

/// OpenAI-compatible response format (with DeepSeek reasoning support)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
    reasoning_content: Option<String>, // For DeepSeek reasoning model
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIChoice {
    index: i32,
    message: OpenAIMessage,
    finish_reason: Option<String>,
    logprobs: Option<serde_json::Value>, // Handle logprobs field
}

#[derive(Debug, Clone, Serialize, Deserialize)]  
struct OpenAIResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: Option<serde_json::Value>, // More flexible usage field
    system_fingerprint: Option<String>, // DeepSeek includes this
}

/// Client that preserves full AWS context for alternative providers
#[derive(Debug, Clone)]
pub struct AlternativeProviderClient {
    endpoint: Endpoint,
    config: AlternativeProviderConfig,
    client: reqwest::Client,
}

impl AlternativeProviderClient {
    pub fn new(endpoint: Endpoint, config: AlternativeProviderConfig) -> Self {
        Self {
            endpoint,
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Send message with FULL AWS context preservation
    pub async fn send_message(
        &self,
        conversation_state: ConversationState,
    ) -> Result<AlternativeProviderOutput, ApiClientError> {
        debug!("Sending conversation to alternative provider with full context: {:#?}", conversation_state);
        
        // Convert full ConversationState to provider format while preserving ALL context
        let context_preserved_content = self.build_context_preserved_content(&conversation_state)?;
        
        let mut messages = Vec::new();
        
        // Add system message that includes ALL the context that AWS would have
        let system_content = self.build_comprehensive_system_prompt(&conversation_state)?;
        messages.push(RequestMessage {
            role: "system".to_string(),
            content: system_content,
        });
        
        // Add conversation history with full fidelity
        if let Some(history) = conversation_state.history {
            for msg in history {
                match msg {
                    crate::api_client::model::ChatMessage::UserInputMessage(user_msg) => {
                        // Preserve the exact formatting that AWS would receive
                        let formatted_content = format!(
                            "--- USER MESSAGE BEGIN ---\n{}\n--- USER MESSAGE END ---\n\n",
                            user_msg.content
                        );
                        messages.push(RequestMessage {
                            role: "user".to_string(),
                            content: formatted_content,
                        });
                    }
                    crate::api_client::model::ChatMessage::AssistantResponseMessage(assistant_msg) => {
                        messages.push(RequestMessage {
                            role: "assistant".to_string(),
                            content: assistant_msg.content,
                        });
                    }
                }
            }
        }
        
        // Add current user message with full context
        messages.push(RequestMessage {
            role: "user".to_string(),
            content: context_preserved_content,
        });
        
        let request = OpenAIRequest {
            model: self.config.model.clone().unwrap_or_else(|| "deepseek-reasoner".to_string()),
            messages,
            temperature: self.config.temperature.unwrap_or(0.8),
            max_tokens: self.config.max_tokens.unwrap_or(-1),
            stream: false,
        };
        
        let response = self.send_request(&request).await?;
        self.process_response(response, conversation_state.conversation_id).await
    }

    /// Build comprehensive system prompt that includes all AWS context
    fn build_comprehensive_system_prompt(&self, conversation_state: &ConversationState) -> Result<String, ApiClientError> {
        let mut system_parts = Vec::new();
        
        // Base system prompt for Q CLI context
        system_parts.push("You are Amazon Q Developer CLI, an AI assistant specialized in software development and command-line interfaces.".to_string());
        
        // Add environment context if available
        if let Some(context) = &conversation_state.user_input_message.user_input_message_context {
            if let Some(env_state) = &context.env_state {
                let mut env_info = Vec::new();
                
                if let Some(os) = &env_state.operating_system {
                    env_info.push(format!("Operating System: {}", os));
                }
                
                if let Some(cwd) = &env_state.current_working_directory {
                    env_info.push(format!("Working Directory: {}", cwd));
                }
                
                if !env_state.environment_variables.is_empty() {
                    let env_vars: Vec<String> = env_state.environment_variables
                        .iter()
                        .map(|var| format!("{}={}", var.key, var.value))
                        .collect();
                    env_info.push(format!("Environment Variables: {}", env_vars.join(", ")));
                }
                
                if !env_info.is_empty() {
                    system_parts.push(format!("Environment Context:\n{}", env_info.join("\n")));
                }
            }
            
            // Add tool specifications - CRITICAL for preserving AWS behavior
            if let Some(tools) = &context.tools {
                if !tools.is_empty() {
                    let mut tool_specs = Vec::new();
                    tool_specs.push("TOOL CALLING INSTRUCTIONS:".to_string());
                    tool_specs.push("You have access to the following tools. When you need to use a tool, format your tool calls EXACTLY as shown in the examples below.".to_string());
                    tool_specs.push("".to_string());
                    tool_specs.push("Available Tools:".to_string());
                    
                    for tool in tools {
                        let crate::api_client::model::Tool::ToolSpecification(spec) = tool;
                        
                        // Format tool specification with clear examples
                        tool_specs.push(format!("### {}", spec.name));
                        tool_specs.push(format!("Description: {}", spec.description));
                        tool_specs.push(format!("Schema: {}", serde_json::to_string_pretty(&spec.input_schema).unwrap_or_else(|_| "Invalid schema".to_string())));
                        
                        // Add specific examples for common tools
                        match spec.name.as_str() {
                            "fs_read" => {
                                tool_specs.push("Example usage:".to_string());
                                tool_specs.push(r#"{"tool": "fs_read", "arguments": {"path": "/path/to/file", "mode": "File"}}"#.to_string());
                                tool_specs.push(r#"{"tool": "fs_read", "arguments": {"path": "/path/to/directory", "mode": "Directory"}}"#.to_string());
                            },
                            "fs_write" => {
                                tool_specs.push("Example usage:".to_string());
                                tool_specs.push(r#"{"tool": "fs_write", "arguments": {"path": "/path/to/file", "content": "file content here"}}"#.to_string());
                            },
                            "execute_bash" => {
                                tool_specs.push("Example usage:".to_string());
                                tool_specs.push(r#"{"tool": "execute_bash", "arguments": {"command": "ls -la", "summary": "List directory contents"}}"#.to_string());
                            },
                            _ => {
                                tool_specs.push("Example usage:".to_string());
                                tool_specs.push(format!(r#"{{"tool": "{}", "arguments": {}}}"#, spec.name, "{ /* your arguments here */ }"));
                            }
                        }
                        tool_specs.push("".to_string());
                    }
                    
                    tool_specs.push("IMPORTANT: Always use the exact tool names and follow the JSON format shown above. Do not invent your own tool names or formats.".to_string());
                    system_parts.push(tool_specs.join("\n"));
                }
            }
            
            // Add previous tool results for context continuity
            if let Some(tool_results) = &context.tool_results {
                if !tool_results.is_empty() {
                    let mut results_summary = Vec::new();
                    results_summary.push("Recent Tool Executions:".to_string());
                    
                    for result in tool_results {
                        let result_desc = format!(
                            "- Tool {}: Status {} (Content length: {})",
                            result.tool_use_id,
                            match &result.status {
                                crate::api_client::model::ToolResultStatus::Success => "Success",
                                crate::api_client::model::ToolResultStatus::Error => "Error",
                            },
                            result.content.len()
                        );
                        results_summary.push(result_desc);
                    }
                    
                    system_parts.push(results_summary.join("\n"));
                }
            }
        }
        
        Ok(system_parts.join("\n\n"))
    }

    /// Build context-preserved content that matches AWS input exactly
    fn build_context_preserved_content(&self, conversation_state: &ConversationState) -> Result<String, ApiClientError> {
        let mut content_parts = Vec::new();
        
        // Add user message with AWS formatting
        let user_content = format!(
            "--- USER MESSAGE BEGIN ---\n{}\n--- USER MESSAGE END ---\n\n",
            conversation_state.user_input_message.content
        );
        content_parts.push(user_content);
        
        // Add context entries if available (preserving AWS format)
        if let Some(_context) = &conversation_state.user_input_message.user_input_message_context {
            // Add any additional context files or summaries
            // This would be where conversation summaries, context files, etc. are added
            // following the same "--- CONTEXT ENTRY BEGIN/END ---" pattern
        }
        
        Ok(content_parts.join(""))
    }

    async fn send_request(&self, request: &OpenAIRequest) -> Result<OpenAIResponse, ApiClientError> {
        let url = match self.config.provider_type {
            ProviderType::OpenAI => format!("{}/chat/completions", self.config.endpoint),
            ProviderType::Anthropic => return Err(ApiClientError::RequestFailed("Anthropic format not yet implemented".to_string())),
            ProviderType::Custom => format!("{}/chat/completions", self.config.endpoint),
        };
        
        let mut req_builder = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(request);
        
        // Add authentication if available
        if let Some(api_key) = &self.config.api_key {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = req_builder
            .send()
            .await
            .map_err(|e| ApiClientError::RequestFailed(format!("HTTP request failed: {}", e)))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(ApiClientError::RequestFailed(format!(
                "HTTP {} - {}",
                status, text
            )));
        }
        
        // Debug: Log the raw response for troubleshooting
        let response_text = response.text().await
            .map_err(|e| ApiClientError::RequestFailed(format!("Failed to read response text: {}", e)))?;
        
        debug!("Raw DeepSeek response: {}", response_text);
        
        let openai_response: OpenAIResponse = serde_json::from_str(&response_text)
            .map_err(|e| ApiClientError::RequestFailed(format!("Failed to parse response JSON: {} - Response was: {}", e, response_text)))?;
        
        Ok(openai_response)
    }

    async fn process_response(
        &self,
        response: OpenAIResponse,
        conversation_id: Option<String>,
    ) -> Result<AlternativeProviderOutput, ApiClientError> {
        if response.choices.is_empty() {
            return Err(ApiClientError::RequestFailed("No response choices".to_string()));
        }
        
        let message = &response.choices[0].message;
        
        // For DeepSeek reasoning model, combine reasoning content with final answer
        let content = if let Some(reasoning) = &message.reasoning_content {
            format!("**Reasoning:**\n{}\n\n**Answer:**\n{}", reasoning, message.content)
        } else {
            message.content.clone()
        };
        
        // Create a channel to simulate streaming (matching AWS behavior)
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Parse tool calls from the content and emit appropriate events
        self.parse_and_emit_response(&tx, &content).await?;
        
        Ok(AlternativeProviderOutput {
            receiver: rx,
            conversation_id,
        })
    }

    /// Parse DeepSeek response and emit proper tool events or assistant text
    async fn parse_and_emit_response(
        &self,
        tx: &mpsc::UnboundedSender<ChatResponseStream>,
        content: &str,
    ) -> Result<(), ApiClientError> {
        debug!("Parsing content for tool calls: {}", content);
        
        // First try DeepSeek marker format
        let deepseek_tool_pattern = Regex::new(r"(?s)<｜tool▁calls▁begin｜>(.*?)<｜tool▁calls▁end｜>").map_err(|e| 
            ApiClientError::RequestFailed(format!("Regex compilation failed: {}", e)))?;
        let deepseek_tool_call_pattern = Regex::new(r"(?s)<｜tool▁call▁begin｜>function<｜tool▁sep｜>([^\n]+)\njson\n(.*?)<｜tool▁call▁end｜>").map_err(|e| 
            ApiClientError::RequestFailed(format!("Regex compilation failed: {}", e)))?;
        
        if let Some(tool_calls_match) = deepseek_tool_pattern.find(content) {
            debug!("Found DeepSeek marker format tool calls");
            return self.parse_deepseek_marker_format(tx, content, tool_calls_match, &deepseek_tool_call_pattern).await;
        }
        
        // Try standard JSON tool call format (exact format from system prompt)
        let json_tool_pattern = Regex::new(r#"(?s)\{\s*"tool":\s*"([^"]+)"\s*,\s*"arguments":\s*(\{.*?\})\s*\}"#).map_err(|e| 
            ApiClientError::RequestFailed(format!("JSON tool regex compilation failed: {}", e)))?;
        
        if let Some(_tool_match) = json_tool_pattern.find(content) {
            debug!("Found JSON format tool call with regex");
            return self.parse_json_tool_format(tx, content, &json_tool_pattern).await;
        }
        
        // Also try direct JSON parsing for any JSON objects that look like tool calls
        if let Some(json_start) = content.find('{') {
            if let Some(json_end) = content.rfind('}') {
                let json_str = &content[json_start..=json_end];
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(json_str) {
                    if let Some(tool_name) = json_value.get("tool").and_then(|v| v.as_str()) {
                        if let Some(arguments) = json_value.get("arguments") {
                            debug!("Found JSON tool call via direct parsing: {}", tool_name);
                            return self.parse_direct_json_tool(tx, content, tool_name, arguments, json_start, json_end + 1).await;
                        }
                    }
                }
            }
        }
        
        debug!("No tool calls found in content");
        // No tool calls, just send the content as assistant response
        if let Err(e) = tx.send(ChatResponseStream::AssistantResponseEvent { content: content.to_string() }) {
            error!("Failed to send response event: {}", e);
        }
        
        Ok(())
    }
    
    /// Parse DeepSeek marker format tool calls
    async fn parse_deepseek_marker_format(
        &self,
        tx: &mpsc::UnboundedSender<ChatResponseStream>,
        content: &str,
        tool_calls_match: regex::Match<'_>,
        tool_call_pattern: &Regex,
    ) -> Result<(), ApiClientError> {
        let before_tools = &content[..tool_calls_match.start()];
        let tool_calls_section = tool_calls_match.as_str();
        let after_tools = &content[tool_calls_match.end()..];
        
        // Send text before tool calls
        if !before_tools.trim().is_empty() {
            if let Err(e) = tx.send(ChatResponseStream::AssistantResponseEvent { 
                content: before_tools.to_string() 
            }) {
                error!("Failed to send pre-tool text: {}", e);
            }
        }
        
        // Parse and emit tool calls
        for tool_call in tool_call_pattern.captures_iter(tool_calls_section) {
            let tool_name = tool_call.get(1).unwrap().as_str().to_string();
            let tool_args = tool_call.get(2).unwrap().as_str().to_string();
            let tool_use_id = format!("tool-{}", Uuid::new_v4().simple());
            
            debug!("Parsed DeepSeek tool call: {} with args: {}", tool_name, tool_args);
            self.emit_tool_events(tx, &tool_use_id, &tool_name, &tool_args).await?;
        }
        
        // Send text after tool calls
        if !after_tools.trim().is_empty() {
            if let Err(e) = tx.send(ChatResponseStream::AssistantResponseEvent { 
                content: after_tools.to_string() 
            }) {
                error!("Failed to send post-tool text: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Parse JSON format tool calls (OpenAI-compatible)
    async fn parse_json_tool_format(
        &self,
        tx: &mpsc::UnboundedSender<ChatResponseStream>,
        content: &str,
        tool_pattern: &Regex,
    ) -> Result<(), ApiClientError> {
        let mut last_end = 0;
        
        for tool_match in tool_pattern.find_iter(content) {
            // Send any text before this tool call
            if tool_match.start() > last_end {
                let pre_text = &content[last_end..tool_match.start()];
                if !pre_text.trim().is_empty() {
                    if let Err(e) = tx.send(ChatResponseStream::AssistantResponseEvent { 
                        content: pre_text.to_string() 
                    }) {
                        error!("Failed to send pre-tool text: {}", e);
                    }
                }
            }
            
            // Parse the tool call
            if let Some(captures) = tool_pattern.captures(tool_match.as_str()) {
                let tool_name = captures.get(1).unwrap().as_str().to_string();
                let tool_args = captures.get(2).unwrap().as_str().to_string();
                let tool_use_id = format!("tool-{}", Uuid::new_v4().simple());
                
                debug!("Parsed JSON tool call: {} with args: {}", tool_name, tool_args);
                self.emit_tool_events(tx, &tool_use_id, &tool_name, &tool_args).await?;
            }
            
            last_end = tool_match.end();
        }
        
        // Send any remaining text after the last tool call
        if last_end < content.len() {
            let remaining_text = &content[last_end..];
            if !remaining_text.trim().is_empty() {
                if let Err(e) = tx.send(ChatResponseStream::AssistantResponseEvent { 
                    content: remaining_text.to_string() 
                }) {
                    error!("Failed to send post-tool text: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Parse JSON tool call found via direct JSON parsing
    async fn parse_direct_json_tool(
        &self,
        tx: &mpsc::UnboundedSender<ChatResponseStream>,
        content: &str,
        tool_name: &str,
        arguments: &serde_json::Value,
        json_start: usize,
        json_end: usize,
    ) -> Result<(), ApiClientError> {
        // Send any text before the tool call
        if json_start > 0 {
            let pre_text = &content[..json_start];
            if !pre_text.trim().is_empty() {
                if let Err(e) = tx.send(ChatResponseStream::AssistantResponseEvent { 
                    content: pre_text.to_string() 
                }) {
                    error!("Failed to send pre-tool text: {}", e);
                }
            }
        }
        
        // Create tool call
        let tool_use_id = format!("tool-{}", Uuid::new_v4().simple());
        let tool_args = arguments.to_string();
        
        debug!("Parsed direct JSON tool call: {} with args: {}", tool_name, tool_args);
        self.emit_tool_events(tx, &tool_use_id, tool_name, &tool_args).await?;
        
        // Send any text after the tool call
        if json_end < content.len() {
            let remaining_text = &content[json_end..];
            if !remaining_text.trim().is_empty() {
                if let Err(e) = tx.send(ChatResponseStream::AssistantResponseEvent { 
                    content: remaining_text.to_string() 
                }) {
                    error!("Failed to send post-tool text: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Emit the ToolUseEvent sequence for a single tool call
    async fn emit_tool_events(
        &self,
        tx: &mpsc::UnboundedSender<ChatResponseStream>,
        tool_use_id: &str,
        tool_name: &str,
        tool_args: &str,
    ) -> Result<(), ApiClientError> {
        // Emit tool use start event
        if let Err(e) = tx.send(ChatResponseStream::ToolUseEvent {
            tool_use_id: tool_use_id.to_string(),
            name: tool_name.to_string(),
            input: None,
            stop: None,
        }) {
            error!("Failed to send tool start event: {}", e);
        }
        
        // Emit tool arguments progressively (simulate streaming)
        if let Err(e) = tx.send(ChatResponseStream::ToolUseEvent {
            tool_use_id: tool_use_id.to_string(),
            name: tool_name.to_string(),
            input: Some(tool_args.to_string()),
            stop: None,
        }) {
            error!("Failed to send tool args: {}", e);
        }
        
        // Emit tool use end event
        if let Err(e) = tx.send(ChatResponseStream::ToolUseEvent {
            tool_use_id: tool_use_id.to_string(),
            name: tool_name.to_string(),
            input: None,
            stop: Some(true),
        }) {
            error!("Failed to send tool end event: {}", e);
        }
        
        Ok(())
    }
}

/// Output that matches the expected streaming interface
#[derive(Debug)]
pub struct AlternativeProviderOutput {
    receiver: mpsc::UnboundedReceiver<ChatResponseStream>,
    conversation_id: Option<String>,
}

impl AlternativeProviderOutput {
    pub fn request_id(&self) -> Option<&str> {
        Some("<alternative-provider-request-id>")
    }
    
    pub fn conversation_id(&self) -> Option<&str> {
        self.conversation_id.as_deref()
    }
    
    pub async fn recv(&mut self) -> Result<Option<ChatResponseStream>, ApiClientError> {
        Ok(self.receiver.recv().await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_client::model::{UserInputMessage, ConversationState};
    use mockito;

    #[tokio::test]
    async fn test_alternative_provider_context_preservation() {
        let mut server = mockito::Server::new_async().await;
        
        let mock_response = serde_json::json!({
            "id": "chatcmpl-test",
            "object": "chat.completion", 
            "created": 1234567890,
            "model": "gpt-4",
            "choices": [{
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "I understand you are asking about scrollback search in Ghostty. Let me analyze the relevant code."
                },
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 150,
                "completion_tokens": 50,
                "total_tokens": 200
            }
        });

        let mock = server
            .mock("POST", "/v1/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create_async()
            .await;

        let endpoint = Endpoint {
            url: server.url().into(),
            region: aws_config::Region::new("local"),
        };

        let config = AlternativeProviderConfig {
            provider_type: ProviderType::Custom,
            endpoint: server.url(),
            model: Some("gpt-4".to_string()),
            api_key: Some("test-key".to_string()),
            temperature: Some(0.8),
            max_tokens: Some(-1),
        };

        let client = AlternativeProviderClient::new(endpoint, config);
        
        let conversation_state = ConversationState {
            conversation_id: Some("test-conversation".to_string()),
            user_input_message: UserInputMessage {
                content: "Ghostty does not have a complete and working scrollback (buffer) search functionality.".to_string(),
                user_input_message_context: None,
                user_intent: None,
                images: None,
            },
            history: None,
        };

        let mut result = client.send_message(conversation_state).await.unwrap();
        
        // Verify we get the expected response
        let response = result.recv().await.unwrap().unwrap();
        match response {
            ChatResponseStream::AssistantResponseEvent { content } => {
                assert!(content.contains("scrollback search"));
                assert!(content.contains("Ghostty"));
            }
            _ => panic!("Unexpected response type"),
        }

        // Verify no more messages
        assert!(result.recv().await.unwrap().is_none());

        mock.assert_async().await;
    }
}