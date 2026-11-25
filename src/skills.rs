//! J1 Agent Skills over MCP (textbook).
//!
//! Extension: `io.modelcontextprotocol/skills`
//! Methods: `skills/list`, `skills/get` (rmcp 3.10 native `ServerHandler` methods)
//! Content: `resources/read` on `skill://…` URIs with sha256 digests.

use std::collections::BTreeMap;
use std::sync::Arc;

use rmcp::model::*;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

/// Extension id advertised in `capabilities.extensions`.
pub const SKILLS_EXTENSION_ID: &str = "io.modelcontextprotocol/skills";

/// Skill frontmatter `name` and path segment.
pub const LAB_SKILL_NAME: &str = "mcp-better-lab";

/// Canonical URI for the lab skill document.
pub const LAB_SKILL_URI: &str = "skill://mcp-better-lab/SKILL.md";
/// What `resources/list` says a SKILL.md is, and what `resources/read` returns (SEP-2640).
pub const SKILL_MD_MIME_TYPE: &str = "text/markdown";

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

    /// Every skill as SEP-2640 `Skill` (frontmatter + file manifest with digest and size).
    pub fn skills(&self) -> Vec<Skill> {
        self.skills.values().map(to_skill).collect()
    }

    /// One skill by the URI of its `SKILL.md`.
    pub fn skill(&self, uri: &str) -> Option<Skill> {
        self.skills.get(uri).map(to_skill)
    }

    pub fn find_resource(&self, uri: &str) -> Option<&SkillResource> {
        for skill in self.skills.values() {
            if let Some(r) = skill.resources.iter().find(|r| r.uri == uri) {
                return Some(r);
            }
        }
        None
    }

    /// One file per skill, by design (#18): each resource is the skill's SKILL.md, so
    /// it takes the skill's name, description and `text/markdown`. A multi-file skill
    /// (`references/`, `scripts/`) needs a name and media type per file, and
    /// `resources/directory/read` if it declares `directoryRead` (SEP-2640).
    pub fn list_resources_meta(&self) -> Vec<Resource> {
        let mut out = Vec::new();
        for skill in self.skills.values() {
            // SEP-2640: a SKILL.md resource carries the skill's frontmatter name and description.
            let field = |k: &str| {
                skill
                    .frontmatter
                    .get(k)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
            };
            for r in &skill.resources {
                out.push(
                    Resource::new(r.uri.clone(), field("name"))
                        .with_description(field("description"))
                        .with_mime_type(SKILL_MD_MIME_TYPE)
                        .with_size(r.bytes.len() as u64),
                );
            }
        }
        out
    }
}

fn to_skill(entry: &SkillEntry) -> Skill {
    let name = entry
        .frontmatter
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let description = entry
        .frontmatter
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let mut frontmatter = SkillFrontmatter::new(name, description);
    for (k, v) in &entry.frontmatter {
        if k != "name" && k != "description" {
            frontmatter = frontmatter.with_field(k.clone(), v.clone());
        }
    }
    let files = entry
        .resources
        .iter()
        .map(|r| {
            rmcp::model::SkillResource::new(r.uri.clone(), r.digest.clone(), r.bytes.len() as u64)
        })
        .collect();
    Skill::new(entry.uri.clone(), frontmatter, SkillResources::Files(files))
}

/// Builds a single-file skill: SKILL.md is its only resource (see `list_resources_meta`).
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

/// Strict frontmatter: `---` … `---` then body. Accepts only one-line `key: value`
/// pairs whose value a YAML parser reads as the same plain string. Anything else
/// (quotes, block scalars, nested maps, lists, numbers, booleans, null, comments)
/// is refused at load: the host reads SKILL.md with a real YAML parser, and
/// `skills/list` must never list frontmatter that differs from what it reads (#17).
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
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let refuse = |why: &str| format!("frontmatter line not supported ({why}): {line}");
        if line.starts_with(char::is_whitespace) {
            return Err(refuse("indented: nested or multi-line value"));
        }
        let (k, v) = line
            .split_once(": ")
            .or_else(|| line.strip_suffix(':').map(|k| (k, "")))
            .ok_or_else(|| refuse("not key: value"))?;
        if k.is_empty()
            || !k
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(refuse("key"));
        }
        let v = v.trim();
        if let Some(why) = not_a_plain_string(v) {
            return Err(refuse(why));
        }
        if map
            .insert(k.to_string(), Value::String(v.to_string()))
            .is_some()
        {
            return Err(refuse("duplicate key"));
        }
    }
    if !map.contains_key("name") || !map.contains_key("description") {
        return Err("frontmatter requires name and description".into());
    }
    Ok((map, body))
}

/// Why YAML would read `v` as something other than the same plain string, if it would.
fn not_a_plain_string(v: &str) -> Option<&'static str> {
    let Some(first) = v.chars().next() else {
        return Some("empty value");
    };
    // `-`, `?` and `:` start a plain string only when a non-space follows.
    let lone = v.len() == 1 || v[1..].starts_with(' ');
    if "\"'|>[]{}&*!%@`#,".contains(first) || ("-?:".contains(first) && lone) {
        return Some("starts with a YAML indicator");
    }
    if v.contains(": ") || v.ends_with(':') {
        return Some("colon-space in value");
    }
    if v.contains(" #") {
        return Some("inline comment");
    }
    let lower = v.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "~" | "null" | "true" | "false" | "yes" | "no" | "on" | "off" | "y" | "n"
    ) {
        return Some("null or boolean");
    }
    let num = lower.trim_start_matches(['+', '-']);
    if num.parse::<f64>().is_ok()
        || num.starts_with("0x")
        || num.starts_with("0o")
        || num.starts_with(".inf")
        || num == ".nan"
    {
        return Some("number");
    }
    None
}

fn hex_sha256(bytes: &[u8]) -> String {
    let hash = Sha256::digest(bytes);
    hash.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lab_skill_loads_with_matching_digest() {
        let cat = SkillCatalog::load_lab().expect("load");
        let list = cat.skills();
        assert_eq!(list.len(), 1);
        let entry = &list[0];
        assert_eq!(entry.uri, LAB_SKILL_URI);
        assert_eq!(entry.frontmatter.name, LAB_SKILL_NAME);
        let SkillResources::Files(files) = &entry.resources else {
            panic!("lab skill has a file manifest");
        };
        let digest = files[0].digest.as_str();
        assert!(digest.starts_with("sha256:"));
        let res = cat.find_resource(LAB_SKILL_URI).unwrap();
        assert_eq!(files[0].size, res.bytes.len() as u64);
        assert_eq!(res.digest, digest);
        let recomputed = format!("sha256:{}", hex_sha256(res.bytes.as_ref()));
        assert_eq!(recomputed, digest);
    }

    /// #18: the catalog is single-file by design. A skill with more files must not
    /// slip in under the SKILL.md labels; this fails first if one is added.
    #[test]
    fn every_skill_is_a_single_skill_md() {
        let cat = SkillCatalog::load_lab().unwrap();
        for skill in cat.skills.values() {
            assert_eq!(skill.resources.len(), 1, "{}", skill.uri);
            assert!(skill.resources[0].uri.ends_with("/SKILL.md"));
        }
    }

    fn fm(lines: &str) -> Result<Map<String, Value>, String> {
        parse_frontmatter(&format!("---\n{lines}\n---\nbody\n")).map(|(m, _)| m)
    }

    /// #17: plain one-line values are read exactly as a YAML parser reads them.
    #[test]
    fn frontmatter_accepts_plain_one_line_strings() {
        let m =
            fm("name: lab\ndescription: Use it when testing — claim equals wire.\nlicense: MIT")
                .expect("plain strings");
        assert_eq!(m["name"], "lab");
        assert_eq!(m["description"], "Use it when testing — claim equals wire.");
        assert_eq!(m["license"], "MIT");
    }

    /// #17: anything a YAML parser would read differently is refused at load, not misread.
    #[test]
    fn frontmatter_refuses_what_it_cannot_read_exactly() {
        let base = "name: lab\ndescription: d";
        for (case, extra) in [
            ("double-quoted", "license: \"MIT\""),
            ("single-quoted", "license: 'MIT'"),
            ("folded block", "notes: >\n  two\n  lines"),
            ("literal block", "notes: |\n  two"),
            ("nested map", "metadata:\n  author: x"),
            ("empty value", "metadata:"),
            ("number", "version: 1.0"),
            ("boolean", "beta: true"),
            ("null", "owner: ~"),
            ("flow list", "tags: [a, b]"),
            ("inline comment", "license: MIT # see file"),
            ("colon-space in value", "description2: a: b"),
            ("duplicate key", "name: again"),
        ] {
            let got = fm(&format!("{base}\n{extra}"));
            assert!(got.is_err(), "{case} should be refused, got {got:?}");
        }
    }

    #[test]
    fn skills_get_unknown_is_none() {
        let cat = SkillCatalog::load_lab().unwrap();
        assert!(cat.skill("skill://nope/SKILL.md").is_none());
    }
}
