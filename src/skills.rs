//! J1 Agent Skills over MCP (textbook).
//!
//! Extension: `io.modelcontextprotocol/skills`
//! Methods: `skills/list`, `skills/get` (via custom request)
//! Content: `resources/read` on `skill://…` URIs with sha256 digests.

use std::collections::BTreeMap;
use std::sync::Arc;

use rmcp::model::*;
use rmcp::ErrorData as McpError;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

/// Extension id advertised in `capabilities.extensions`.
pub const SKILLS_EXTENSION_ID: &str = "io.modelcontextprotocol/skills";

/// Skill frontmatter `name` and path segment.
pub const LAB_SKILL_NAME: &str = "mcp-better-lab";

/// Canonical URI for the lab skill document.
pub const LAB_SKILL_URI: &str = "skill://mcp-better-lab/SKILL.md";

/// Embedded skill body (crate-relative; always available for install + tests).
const EMBEDDED_LAB_SKILL: &str = include_str!("../skills/mcp-better-lab/SKILL.md");

#[derive(Debug, Clone)]
pub struct SkillResource {
    pub uri: String,
    pub digest: String,
    pub bytes: Arc<[u8]>,
    pub text: Arc<str>,
}

#[derive(Debug, Clone)]
pub struct SkillEntry {
    pub uri: String,
    pub frontmatter: Map<String, Value>,
    pub resources: Vec<SkillResource>,
}

#[derive(Debug, Clone)]
pub struct SkillCatalog {
    skills: BTreeMap<String, SkillEntry>,
}

impl SkillCatalog {
    /// Load the textbook lab skill (embedded; optional disk override later).
    pub fn load_lab() -> Result<Self, String> {
        let entry = load_skill_from_markdown(EMBEDDED_LAB_SKILL)?;
        if entry.frontmatter.get("name").and_then(|v| v.as_str()) != Some(LAB_SKILL_NAME) {
            return Err(format!("frontmatter name must be {LAB_SKILL_NAME}"));
        }
        // URI path final segment must equal frontmatter name.
        let path_name = entry
            .uri
            .strip_prefix("skill://")
            .and_then(|rest| rest.split('/').next())
            .unwrap_or("");
        if path_name != LAB_SKILL_NAME {
            return Err(format!(
                "URI skill name {path_name:?} must equal frontmatter name {LAB_SKILL_NAME}"
            ));
        }
        let mut skills = BTreeMap::new();
        skills.insert(entry.uri.clone(), entry);
        Ok(Self { skills })
    }

    pub fn list_entries(&self) -> Vec<Value> {
        self.skills.values().map(skill_entry_json).collect()
    }

    pub fn get_by_uri(&self, uri: &str) -> Option<Value> {
        self.skills.get(uri).map(skill_entry_json)
    }

    pub fn find_resource(&self, uri: &str) -> Option<&SkillResource> {
        for skill in self.skills.values() {
            if let Some(r) = skill.resources.iter().find(|r| r.uri == uri) {
                return Some(r);
            }
        }
        None
    }

    pub fn list_resources_meta(&self) -> Vec<Resource> {
        let mut out = Vec::new();
        for skill in self.skills.values() {
            for r in &skill.resources {
                let name = r.uri.rsplit('/').next().unwrap_or("SKILL.md").to_string();
                out.push(
                    Resource::new(r.uri.clone(), name)
                        .with_description("Agent Skill document (mcp-better textbook)")
                        .with_mime_type("text/markdown")
                        .with_size(r.bytes.len() as u64),
                );
            }
        }
        out
    }
}

fn skill_entry_json(entry: &SkillEntry) -> Value {
    let resources: Vec<Value> = entry
        .resources
        .iter()
        .map(|r| {
            json!({
                "uri": r.uri,
                "digest": r.digest,
            })
        })
        .collect();
    json!({
        "uri": entry.uri,
        "frontmatter": Value::Object(entry.frontmatter.clone()),
        "resources": resources,
    })
}

fn load_skill_from_markdown(md: &str) -> Result<SkillEntry, String> {
    let (frontmatter, _body) = parse_frontmatter(md)?;
    let name = frontmatter
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "frontmatter missing name".to_string())?
        .to_string();
    if name.is_empty() {
        return Err("frontmatter name empty".into());
    }
    let uri = format!("skill://{name}/SKILL.md");
    let bytes = md.as_bytes();
    let digest = format!("sha256:{}", hex_sha256(bytes));
    let resource = SkillResource {
        uri: uri.clone(),
        digest,
        bytes: Arc::from(bytes.to_vec().into_boxed_slice()),
        text: Arc::from(md),
    };
    Ok(SkillEntry {
        uri,
        frontmatter,
        resources: vec![resource],
    })
}

/// Minimal YAML-ish frontmatter: `---` … `---` then body. Only `key: value` lines.
fn parse_frontmatter(md: &str) -> Result<(Map<String, Value>, String), String> {
    let md = md.trim_start_matches('\u{feff}');
    let rest = md
        .strip_prefix("---")
        .ok_or_else(|| "missing opening frontmatter ---".to_string())?;
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    let (yaml, body) = rest
        .split_once("\n---")
        .ok_or_else(|| "missing closing frontmatter ---".to_string())?;
    let body = body.strip_prefix('\n').unwrap_or(body).to_string();
    let mut map = Map::new();
    for line in yaml.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = line
            .split_once(':')
            .ok_or_else(|| format!("bad frontmatter line: {line}"))?;
        let k = k.trim().to_string();
        let v = v.trim().to_string();
        map.insert(k, Value::String(v));
    }
    if !map.contains_key("name") || !map.contains_key("description") {
        return Err("frontmatter requires name and description".into());
    }
    Ok((map, body))
}

fn hex_sha256(bytes: &[u8]) -> String {
    let hash = Sha256::digest(bytes);
    hash.iter().map(|b| format!("{b:02x}")).collect()
}

/// Handle `skills/list` and `skills/get` custom methods.
pub fn handle_skills_method(
    catalog: &SkillCatalog,
    method: &str,
    params: Option<&Value>,
) -> Result<CustomResult, McpError> {
    match method {
        "skills/list" => {
            let skills = catalog.list_entries();
            Ok(CustomResult::new(json!({
                "skills": skills,
            })))
        }
        "skills/get" => {
            let uri = params
                .and_then(|p| p.get("uri"))
                .and_then(|u| u.as_str())
                .ok_or_else(|| McpError::invalid_params("skills/get requires params.uri", None))?;
            let entry = catalog.get_by_uri(uri).ok_or_else(|| {
                McpError::invalid_params(format!("unknown skill uri: {uri}"), None)
            })?;
            Ok(CustomResult::new(entry))
        }
        other => Err(McpError::new(
            ErrorCode::METHOD_NOT_FOUND,
            other.to_string(),
            None,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lab_skill_loads_with_matching_digest() {
        let cat = SkillCatalog::load_lab().expect("load");
        let list = cat.list_entries();
        assert_eq!(list.len(), 1);
        let entry = &list[0];
        assert_eq!(entry["uri"], LAB_SKILL_URI);
        assert_eq!(entry["frontmatter"]["name"], LAB_SKILL_NAME);
        let digest = entry["resources"][0]["digest"].as_str().unwrap();
        assert!(digest.starts_with("sha256:"));
        let res = cat.find_resource(LAB_SKILL_URI).unwrap();
        assert_eq!(res.digest, digest);
        let recomputed = format!("sha256:{}", hex_sha256(res.bytes.as_ref()));
        assert_eq!(recomputed, digest);
    }

    #[test]
    fn skills_get_unknown_errors() {
        let cat = SkillCatalog::load_lab().unwrap();
        let err = handle_skills_method(
            &cat,
            "skills/get",
            Some(&json!({"uri": "skill://nope/SKILL.md"})),
        )
        .unwrap_err();
        let s = format!("{err:?}");
        assert!(s.contains("unknown") || s.contains("invalid") || s.contains("nope"));
    }
}
