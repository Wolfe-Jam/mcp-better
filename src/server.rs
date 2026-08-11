//! BETTER textbook MCP server — tools + Agent Skills (J1).
//!
//! Protocol claim: **2026-07-28** (stdio default · Streamable HTTP via `--http`).
//! List results stamp `ttlMs` + `cacheScope` (SEP-2549) — SDK defaults are unstamped.
//!
//! `confirm_echo` demonstrates SEP-2322 MRTR (`input_required` → client retry).
//! Note: `rmcp` tool-router drops `inputResponses` / `requestState`, so MRTR tools
//! are handled in an explicit `call_tool` override (see `confirm_echo_call`).
//!
//! Skills: extension `io.modelcontextprotocol/skills` via custom methods
//! `skills/list` · `skills/get`; content via `resources/read` + digests.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::service::RequestContext;
use rmcp::{tool, tool_handler, tool_router, ErrorData as McpError, RoleServer, ServerHandler};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::skills::{handle_skills_method, SkillCatalog, SKILLS_EXTENSION_ID};

/// Catalog is static → public cache with a one-minute freshness window.
pub const LIST_TOOLS_TTL_MS: u64 = 60_000;

/// Fixed tool order for stable `tools/list` across calls and process restarts.
/// Public so smokes and the lying companion can name the BETTER contract.
pub const BETTER_TOOL_ORDER: &[&str] = &["health", "echo", "confirm_echo"];

/// Demo-only signing key for textbook `requestState` seals.
/// Production servers must use a secret key (env / HSM) — not a constant in source.
const MRTR_STATE_KEY: &[u8] = b"mcp-better-textbook-mrtr-demo-key-v1!!";

/// Associated data binds the seal to this tool name (integrity, not confidentiality).
const MRTR_AAD: &[u8] = b"mcp-better/confirm_echo";

/// Elicitation map key the client must answer.
const CONFIRM_INPUT_KEY: &str = "confirm";

#[derive(Debug, Clone)]
pub struct BetterServer {
    tool_router: ToolRouter<Self>,
    state_codec: RequestStateCodec,
    skills: SkillCatalog,
}

impl BetterServer {
    pub fn new() -> Self {
        let skills = SkillCatalog::load_lab().unwrap_or_else(|e| {
            // Textbook must ship with embedded skill; panic only if embed is broken.
            panic!("failed to load mcp-better-lab skill: {e}");
        });
        Self {
            tool_router: Self::tool_router(),
            state_codec: RequestStateCodec::new(MRTR_STATE_KEY),
            skills,
        }
    }

    /// Skills catalog (for tests / smokes).
    pub fn skills(&self) -> &SkillCatalog {
        &self.skills
    }

    /// Stamped list result used by the handler and unit tests.
    pub fn stamped_list_tools(&self) -> ListToolsResult {
        let mut tools = self.tool_router.list_all();
        tools.sort_by(|a, b| {
            let ia = BETTER_TOOL_ORDER
                .iter()
                .position(|n| *n == a.name.as_ref())
                .unwrap_or(usize::MAX);
            let ib = BETTER_TOOL_ORDER
                .iter()
                .position(|n| *n == b.name.as_ref())
                .unwrap_or(usize::MAX);
            ia.cmp(&ib).then_with(|| a.name.cmp(&b.name))
        });

        ListToolsResult::with_all_items(tools)
            .with_ttl_ms(LIST_TOOLS_TTL_MS)
            .with_cache_scope(CacheScope::Public)
    }

    /// SEP-2322 textbook path for `confirm_echo` (full `CallToolRequestParams`).
    ///
    /// Round 1 — no `inputResponses`: `input_required` + sealed `requestState`.  
    /// Round 2 — open seal, require form content `confirm == "CONFIRM"`, complete.
    pub fn confirm_echo_call(
        &self,
        request: CallToolRequestParams,
    ) -> Result<CallToolResponse, McpError> {
        let args = request.arguments.unwrap_or_default();
        let message = args
            .get("message")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                McpError::invalid_params("confirm_echo requires string argument `message`", None)
            })?
            .to_string();

        match request.input_responses.as_ref() {
            None => {
                // Round 1: seal message into requestState; ask client to confirm.
                let sealed = self
                    .state_codec
                    .seal_json_with(
                        &ConfirmState {
                            message: message.clone(),
                        },
                        &SealOptions::new().associated_data(MRTR_AAD),
                    )
                    .map_err(|e| {
                        McpError::internal_error(format!("failed to seal requestState: {e}"), None)
                    })?;

                let mut requests = InputRequests::new();
                requests.insert(
                    CONFIRM_INPUT_KEY.to_string(),
                    InputRequest::Elicitation(ElicitRequest::new(
                        ElicitRequestParams::FormElicitationParams {
                            meta: None,
                            message: "Type CONFIRM to echo the sealed message.".into(),
                            requested_schema: serde_json::from_value(json!({
                                "type": "object",
                                "properties": {
                                    "confirm": {
                                        "type": "string",
                                        "description": "Type CONFIRM (exact) to continue"
                                    }
                                },
                                "required": ["confirm"]
                            }))
                            .map_err(|e| {
                                McpError::internal_error(format!("elicitation schema: {e}"), None)
                            })?,
                        },
                    )),
                );

                Ok(InputRequiredResult::new(Some(requests), Some(sealed)).into())
            }
            Some(responses) => {
                // Round 2: integrity-check requestState; do not trust client bytes.
                let sealed = request.request_state.as_deref().ok_or_else(|| {
                    McpError::invalid_params(
                        "confirm_echo retry requires echoed requestState",
                        None,
                    )
                })?;

                let state: ConfirmState = self
                    .state_codec
                    .open_json_with(sealed, MRTR_AAD)
                    .map_err(|_| {
                        McpError::invalid_params(
                            "requestState failed integrity check (tampered, expired, or wrong tool)",
                            None,
                        )
                    })?;

                // Prefer sealed message over re-sent args (client must not mutate sealed work).
                if state.message != message {
                    // Soft: still use sealed message; surface honesty in result.
                    // Hard reject if args diverge wildly? Prefer sealed as source of truth.
                }
                let message = state.message;

                let elicit = responses.get(CONFIRM_INPUT_KEY).ok_or_else(|| {
                    McpError::invalid_params(
                        format!("missing inputResponses[{CONFIRM_INPUT_KEY}]"),
                        None,
                    )
                })?;

                let action = elicit.get("action").and_then(|v| v.as_str()).unwrap_or("");
                if action != "accept" {
                    return Err(McpError::invalid_params(
                        format!("confirmation not accepted (action={action})"),
                        None,
                    ));
                }

                let confirm = elicit
                    .pointer("/content/confirm")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if confirm != "CONFIRM" {
                    return Err(McpError::invalid_params(
                        "type CONFIRM (exact) in the form to complete confirm_echo",
                        None,
                    ));
                }

                let body = json!({
                    "echo": message,
                    "confirmed": true,
                    "resultType": "complete",
                    "mrtr": "confirm_echo",
                });

                Ok(CallToolResult::success(vec![ContentBlock::text(body.to_string())]).into())
            }
        }
    }
}

impl Default for BetterServer {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tool params ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct EchoParams {
    #[schemars(description = "Text to echo back unchanged")]
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ConfirmEchoParams {
    #[schemars(description = "Text to echo after the client confirms (MRTR mid-call)")]
    pub message: String,
}

/// Payload sealed inside `requestState` (integrity-checked on retry).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ConfirmState {
    message: String,
}

// ─── Tools ──────────────────────────────────────────────────────────────────

#[tool_router]
impl BetterServer {
    /// No-side-effect liveness probe. Not a k8s/HTTP health contract — a normal MCP tool.
    #[tool(description = "Liveness check. Returns ok + server version. No side effects.")]
    fn health(&self) -> String {
        serde_json::json!({
            "status": "ok",
            "server": "mcp-better",
            "version": env!("CARGO_PKG_VERSION"),
            "protocol": "2026-07-28",
            "tier": "BETTER",
        })
        .to_string()
    }

    /// Pure demo tool — validates argument round-trip without I/O or shell.
    #[tool(description = "Echo a message back. Pure function; no side effects.")]
    fn echo(&self, Parameters(EchoParams { message }): Parameters<EchoParams>) -> String {
        message
    }

    /// Textbook MRTR tool — schema only via router; execution is `call_tool` override
    /// (router context does not carry `inputResponses` / `requestState`).
    #[tool(
        description = "Echo a message after mid-call confirmation (SEP-2322 MRTR). Round 1 returns input_required; retry with inputResponses + echoed requestState. Requires negotiated protocol ≥ 2026-07-28."
    )]
    fn confirm_echo(
        &self,
        Parameters(ConfirmEchoParams { message: _ }): Parameters<ConfirmEchoParams>,
    ) -> String {
        // Unreachable when ServerHandler::call_tool intercepts `confirm_echo`.
        // Kept so tools/list advertises a correct input schema.
        "confirm_echo: use call_tool with MRTR fields (intercepted in ServerHandler)".into()
    }
}

// ─── ServerHandler ──────────────────────────────────────────────────────────

#[tool_handler(router = self.tool_router)]
impl ServerHandler for BetterServer {
    fn get_info(&self) -> ServerInfo {
        let mut extensions = ExtensionCapabilities::new();
        extensions.insert(
            SKILLS_EXTENSION_ID.to_string(),
            serde_json::from_value(json!({})).expect("empty object"),
        );
        let caps = ServerCapabilities::builder()
            .enable_tools()
            .enable_resources()
            .enable_extensions_with(extensions)
            .build();
        let mut info = ServerInfo::new(caps);
        info.server_info = Implementation::new("mcp-better", env!("CARGO_PKG_VERSION"));
        // Advertise 7/28 so peers can negotiate SEP-2322 MRTR.
        info.protocol_version = ProtocolVersion::V_2026_07_28;
        // Transport-neutral: same server runs stdio (default) or Streamable HTTP (--http).
        info.instructions = Some(
            "BETTER textbook MCP server — protocol 2026-07-28. \
             Tools: health (liveness), echo (pure demo), confirm_echo (MRTR textbook). \
             Skills: extension io.modelcontextprotocol/skills — skills/list · skills/get · \
             resources/read skill://… (digests). \
             tools/list stamps ttlMs + cacheScope for Discover-compatible clients. \
             Transports: stdio (default) · Streamable HTTP (--http, local demo). \
             MRTR: confirm_echo needs negotiated ≥ 2026-07-28; older clients get a protocol error on input_required."
                .into(),
        );
        info
    }

    /// Override generated list_tools so BETTER stamps cache hints (Gap 4 / C3).
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(self.stamped_list_tools())
    }

    /// Intercept `confirm_echo` so MRTR fields are not dropped by ToolCallContext.
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        if request.name.as_ref() == "confirm_echo" {
            return self.confirm_echo_call(request);
        }
        let tcc = ToolCallContext::new(self, request, context);
        self.tool_router.call(tcc).await
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: self.skills.list_resources_meta(),
            ..Default::default()
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, McpError> {
        let uri = request.uri.as_ref();
        let res = self.skills.find_resource(uri).ok_or_else(|| {
            McpError::invalid_params(format!("unknown resource uri: {uri}"), None)
        })?;
        Ok(ReadResourceResult::new(vec![ResourceContents::text(
            res.text.as_ref(),
            res.uri.clone(),
        )])
        .into())
    }

    /// `skills/list` and `skills/get` — rmcp has no first-class skills methods yet.
    async fn on_custom_request(
        &self,
        request: CustomRequest,
        _context: RequestContext<RoleServer>,
    ) -> Result<CustomResult, McpError> {
        let CustomRequest { method, params, .. } = request;
        handle_skills_method(&self.skills, method.as_str(), params.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Map;

    #[test]
    fn list_tools_stamped_public_ttl() {
        let server = BetterServer::new();
        let result = server.stamped_list_tools();

        assert_eq!(result.ttl_ms, Some(LIST_TOOLS_TTL_MS));
        assert_eq!(result.cache_scope, Some(CacheScope::Public));
        assert!(result.ttl_ms.unwrap() > 0);
    }

    #[test]
    fn list_tools_stable_order() {
        let server = BetterServer::new();
        let a = server.stamped_list_tools();
        let b = server.stamped_list_tools();
        let names_a: Vec<_> = a.tools.iter().map(|t| t.name.as_ref()).collect();
        let names_b: Vec<_> = b.tools.iter().map(|t| t.name.as_ref()).collect();
        assert_eq!(names_a, names_b);
        assert_eq!(names_a, vec!["health", "echo", "confirm_echo"]);
    }

    #[test]
    fn only_advertised_tools() {
        let server = BetterServer::new();
        let result = server.stamped_list_tools();
        assert_eq!(result.tools.len(), 3);
        assert!(server.tool_router.has_route("health"));
        assert!(server.tool_router.has_route("echo"));
        assert!(server.tool_router.has_route("confirm_echo"));
        assert!(!server.tool_router.has_route("shell"));
    }

    /// Same-process: many lists must not drift (prompt-cache / client cache).
    #[test]
    fn list_tools_stable_across_many_calls() {
        let server = BetterServer::new();
        let first: Vec<String> = server
            .stamped_list_tools()
            .tools
            .iter()
            .map(|t| t.name.to_string())
            .collect();
        assert_eq!(
            first,
            vec![
                "health".to_string(),
                "echo".to_string(),
                "confirm_echo".to_string()
            ]
        );
        for i in 0..20 {
            let names: Vec<String> = server
                .stamped_list_tools()
                .tools
                .iter()
                .map(|t| t.name.to_string())
                .collect();
            assert_eq!(names, first, "order drifted at list call {i}");
        }
    }

    /// New server instances (stand-in for process restart) keep the same catalog order.
    #[test]
    fn list_tools_order_survives_new_instances() {
        let a = BetterServer::new().stamped_list_tools();
        let b = BetterServer::new().stamped_list_tools();
        let names_a: Vec<_> = a.tools.iter().map(|t| t.name.as_ref()).collect();
        let names_b: Vec<_> = b.tools.iter().map(|t| t.name.as_ref()).collect();
        assert_eq!(names_a, BETTER_TOOL_ORDER);
        assert_eq!(names_b, BETTER_TOOL_ORDER);
        assert_eq!(a.ttl_ms, Some(LIST_TOOLS_TTL_MS));
        assert_eq!(b.ttl_ms, Some(LIST_TOOLS_TTL_MS));
    }

    fn confirm_args(message: &str) -> CallToolRequestParams {
        let mut args = Map::new();
        args.insert("message".into(), json!(message));
        CallToolRequestParams::new("confirm_echo").with_arguments(args)
    }

    #[test]
    fn confirm_echo_round1_input_required() {
        let server = BetterServer::new();
        let resp = server
            .confirm_echo_call(confirm_args("hello-mrtr"))
            .unwrap();
        match resp {
            CallToolResponse::InputRequired(ir) => {
                assert!(ir.result_type.is_input_required());
                assert!(ir.request_state.is_some());
                let reqs = ir.input_requests.as_ref().expect("input_requests");
                assert!(reqs.contains_key(CONFIRM_INPUT_KEY));
            }
            other => panic!("expected InputRequired, got {other:?}"),
        }
    }

    #[test]
    fn confirm_echo_round2_complete_after_confirm() {
        let server = BetterServer::new();
        let r1 = server
            .confirm_echo_call(confirm_args("hello-mrtr"))
            .unwrap();
        let CallToolResponse::InputRequired(ir) = r1 else {
            panic!("round1");
        };
        let sealed = ir.request_state.expect("sealed state");

        let mut responses = InputResponses::new();
        responses.insert(
            CONFIRM_INPUT_KEY.to_string(),
            json!({
                "action": "accept",
                "content": { "confirm": "CONFIRM" }
            }),
        );

        let req = confirm_args("hello-mrtr")
            .with_input_responses(responses)
            .with_request_state(sealed);

        let r2 = server.confirm_echo_call(req).unwrap();
        match r2 {
            CallToolResponse::Complete(result) => {
                let text = format!("{:?}", result.content);
                assert!(text.contains("hello-mrtr") || text.contains("confirmed"));
            }
            other => panic!("expected Complete, got {other:?}"),
        }
    }

    #[test]
    fn confirm_echo_rejects_tampered_state() {
        let server = BetterServer::new();
        let r1 = server.confirm_echo_call(confirm_args("secret")).unwrap();
        let CallToolResponse::InputRequired(_) = r1 else {
            panic!("round1");
        };

        let mut responses = InputResponses::new();
        responses.insert(
            CONFIRM_INPUT_KEY.to_string(),
            json!({ "action": "accept", "content": { "confirm": "CONFIRM" } }),
        );
        let req = confirm_args("secret")
            .with_input_responses(responses)
            .with_request_state("not-a-valid-seal");

        let err = server.confirm_echo_call(req).unwrap_err();
        let msg = format!("{err:?}");
        assert!(
            msg.contains("integrity") || msg.contains("requestState") || msg.contains("invalid"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn skills_extension_advertised() {
        let server = BetterServer::new();
        let info = server.get_info();
        let caps = info.capabilities;
        let ext = caps.extensions.expect("extensions");
        assert!(ext.contains_key(SKILLS_EXTENSION_ID));
        assert!(caps.resources.is_some());
    }

    #[test]
    fn skills_list_get_and_read_digest_match() {
        use crate::skills::{handle_skills_method, LAB_SKILL_URI};
        let server = BetterServer::new();
        let list = handle_skills_method(server.skills(), "skills/list", None).unwrap();
        let list_val = list.0;
        let skills = list_val["skills"].as_array().unwrap();
        assert_eq!(skills.len(), 1);
        let digest = skills[0]["resources"][0]["digest"].as_str().unwrap();
        let get = handle_skills_method(
            server.skills(),
            "skills/get",
            Some(&json!({"uri": LAB_SKILL_URI})),
        )
        .unwrap();
        assert_eq!(get.0["uri"], LAB_SKILL_URI);
        assert_eq!(get.0["resources"][0]["digest"], digest);
        let res = server.skills().find_resource(LAB_SKILL_URI).unwrap();
        assert_eq!(res.digest, digest);
        // resources/read body must match digest (same bytes as catalog)
        assert_eq!(
            format!("sha256:{}", {
                use sha2::{Digest, Sha256};
                Sha256::digest(res.text.as_bytes())
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<String>()
            }),
            digest
        );
    }

    #[test]
    fn confirm_echo_rejects_wrong_confirm_text() {
        let server = BetterServer::new();
        let r1 = server.confirm_echo_call(confirm_args("x")).unwrap();
        let CallToolResponse::InputRequired(ir) = r1 else {
            panic!("round1");
        };
        let sealed = ir.request_state.unwrap();

        let mut responses = InputResponses::new();
        responses.insert(
            CONFIRM_INPUT_KEY.to_string(),
            json!({ "action": "accept", "content": { "confirm": "yes" } }),
        );
        let req = confirm_args("x")
            .with_input_responses(responses)
            .with_request_state(sealed);

        assert!(server.confirm_echo_call(req).is_err());
    }
}
