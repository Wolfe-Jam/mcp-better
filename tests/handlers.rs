//! The handlers over a real client/server pair (in-process duplex, Discover lifecycle):
//! cache hints on every list and read, declared-vs-answered, and error shapes.

use mcp_better::{BetterServer, CATALOG_TTL_MS, LAB_SKILL_URI};
use rmcp::model::{
    CacheScope, CallToolRequestParams, CallToolResponse, ClientCapabilities, ClientConfig,
    ErrorCode, GetPromptRequestParams, GetSkillRequestParams, Implementation, ProtocolVersion,
    ReadResourceRequestParams, RequestMetaObject, SkillResources,
};
use rmcp::service::{RoleClient, RunningService};
use rmcp::{ClientLifecycleMode, ClientServiceExt, ServiceError, ServiceExt};

async fn connect() -> RunningService<RoleClient, ClientConfig> {
    connect_with(ClientConfig::default()).await
}

async fn connect_with(config: ClientConfig) -> RunningService<RoleClient, ClientConfig> {
    let (server_io, client_io) = tokio::io::duplex(64 * 1024);
    tokio::spawn(async move {
        let (r, w) = tokio::io::split(server_io);
        let running = BetterServer::new()
            .serve((r, w))
            .await
            .expect("server starts");
        let _ = running.waiting().await;
    });
    let (r, w) = tokio::io::split(client_io);
    config
        .serve_with_lifecycle(
            (r, w),
            ClientLifecycleMode::Discover {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
            },
        )
        .await
        .expect("client connects")
}

fn chosen(ttl: Option<u64>, scope: Option<CacheScope>) {
    assert_eq!(ttl, Some(CATALOG_TTL_MS), "ttlMs is the server's choice");
    assert_eq!(
        scope,
        Some(CacheScope::Public),
        "cacheScope is the server's choice"
    );
}

fn error_code(err: ServiceError) -> (ErrorCode, Option<serde_json::Value>) {
    match err {
        ServiceError::McpError(e) => (e.code, e.data),
        other => panic!("expected a JSON-RPC error, got {other:?}"),
    }
}

#[tokio::test]
async fn discover_and_every_list_and_read_carry_chosen_cache_hints() {
    let client = connect().await;

    let discover = client
        .discover(RequestMetaObject::with_client_context(
            ProtocolVersion::V_2026_07_28,
            Implementation::new("handlers-test", "0"),
            ClientCapabilities::default(),
        ))
        .await
        .unwrap();
    chosen(Some(discover.ttl_ms), Some(discover.cache_scope));

    let tools = client.list_tools(None).await.unwrap();
    chosen(tools.ttl_ms, tools.cache_scope);

    let resources = client.list_resources(None).await.unwrap();
    chosen(resources.ttl_ms, resources.cache_scope);

    let templates = client.list_resource_templates(None).await.unwrap();
    chosen(templates.ttl_ms, templates.cache_scope);

    let skills = client.list_skills(None).await.unwrap();
    chosen(skills.ttl_ms, skills.cache_scope);

    let skill = client
        .get_skill(GetSkillRequestParams::new(LAB_SKILL_URI))
        .await
        .unwrap();
    chosen(skill.ttl_ms, skill.cache_scope);

    let read = client
        .read_resource(ReadResourceRequestParams::new(LAB_SKILL_URI))
        .await
        .unwrap();
    chosen(read.ttl_ms, read.cache_scope);

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn skill_manifest_size_matches_the_read_bytes() {
    let client = connect().await;
    let skill = client
        .get_skill(GetSkillRequestParams::new(LAB_SKILL_URI))
        .await
        .unwrap()
        .skill;
    let SkillResources::Files(files) = skill.resources else {
        panic!("lab skill has a file manifest");
    };
    let read = client
        .read_resource(ReadResourceRequestParams::new(LAB_SKILL_URI))
        .await
        .unwrap();
    let text = match &read.contents[0] {
        rmcp::model::ResourceContents::TextResourceContents { text, .. } => text.clone(),
        other => panic!("expected text, got {other:?}"),
    };
    assert_eq!(files[0].size, text.len() as u64);
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn undeclared_prompts_answer_method_not_found() {
    let client = connect().await;
    let caps = client.peer_info().expect("discover").capabilities.clone();
    assert!(caps.prompts.is_none(), "prompts are not declared");
    assert!(caps.completions.is_none(), "completions are not declared");
    let err = client.list_prompts(None).await.unwrap_err();
    assert_eq!(error_code(err).0, ErrorCode::METHOD_NOT_FOUND);
    let err = client
        .get_prompt(GetPromptRequestParams::new("nope"))
        .await
        .unwrap_err();
    assert_eq!(error_code(err).0, ErrorCode::METHOD_NOT_FOUND);
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn skill_md_lists_and_reads_as_the_same_thing() {
    let client = connect().await;
    let listed = client.list_resources(None).await.unwrap().resources;
    let skill = &listed[0];
    assert_eq!(skill.name, "mcp-better-lab", "name = frontmatter name");
    assert_eq!(skill.mime_type.as_deref(), Some("text/markdown"));
    let got = client
        .get_skill(GetSkillRequestParams::new(LAB_SKILL_URI))
        .await
        .unwrap()
        .skill;
    assert_eq!(
        skill.description.as_deref(),
        Some(got.frontmatter.description.as_str()),
        "description = frontmatter description"
    );
    let read = client
        .read_resource(ReadResourceRequestParams::new(LAB_SKILL_URI))
        .await
        .unwrap();
    match &read.contents[0] {
        rmcp::model::ResourceContents::TextResourceContents { mime_type, .. } => {
            assert_eq!(mime_type.as_deref(), Some("text/markdown"));
        }
        other => panic!("expected text, got {other:?}"),
    }
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn undeclared_completions_answer_method_not_found() {
    let client = connect().await;
    let err = client
        .complete_prompt_argument("any", "arg", "", None)
        .await
        .unwrap_err();
    assert_eq!(error_code(err).0, ErrorCode::METHOD_NOT_FOUND);
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn not_found_errors_name_the_uri() {
    let client = connect().await;

    let err = client
        .read_resource(ReadResourceRequestParams::new("skill://nope/SKILL.md"))
        .await
        .unwrap_err();
    assert_eq!(error_code(err).1.unwrap()["uri"], "skill://nope/SKILL.md");

    let err = client
        .get_skill(GetSkillRequestParams::new("skill://nope/SKILL.md"))
        .await
        .unwrap_err();
    assert_eq!(error_code(err).1.unwrap()["uri"], "skill://nope/SKILL.md");

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn every_tool_declares_all_four_annotation_hints() {
    let client = connect().await;
    let tools = client.list_tools(None).await.unwrap();
    assert_eq!(tools.tools.len(), 3);
    for tool in &tools.tools {
        let a = tool
            .annotations
            .as_ref()
            .unwrap_or_else(|| panic!("{} has no annotations", tool.name));
        assert_eq!(a.read_only_hint, Some(true), "{} reads only", tool.name);
        assert_eq!(
            a.destructive_hint,
            Some(false),
            "{} destroys nothing",
            tool.name
        );
        assert_eq!(a.idempotent_hint, Some(true), "{} is idempotent", tool.name);
        assert_eq!(
            a.open_world_hint,
            Some(false),
            "{} stays in-process",
            tool.name
        );
    }
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn confirm_echo_asks_only_clients_that_declared_elicitation() {
    let call = || {
        CallToolRequestParams::new("confirm_echo")
            .with_arguments(rmcp::model::object(serde_json::json!({ "message": "m1" })))
    };

    // No elicitation declared: refused with -32021, naming what is required.
    let client = connect().await;
    let err = client.call_tool_once(call()).await.unwrap_err();
    let (code, data) = error_code(err);
    assert_eq!(code, ErrorCode::MISSING_REQUIRED_CLIENT_CAPABILITY);
    assert!(
        data.as_ref()
            .and_then(|d| d.pointer("/requiredCapabilities/elicitation"))
            .is_some(),
        "error names elicitation: {data:?}"
    );
    client.cancel().await.unwrap();

    // Elicitation declared: round 1 asks.
    let mut config = ClientConfig::default();
    config.capabilities = ClientCapabilities::builder().enable_elicitation().build();
    let client = connect_with(config).await;
    let r1 = client.call_tool_once(call()).await.expect("round 1");
    assert!(
        matches!(r1, CallToolResponse::InputRequired(_)),
        "round 1 is input_required: {r1:?}"
    );
    client.cancel().await.unwrap();
}
