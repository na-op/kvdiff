use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq)]
pub enum EntryChange {
    Added(String),
    Removed(String),
    Modified { old: String, new: String },
}

pub fn parse_kv(input: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for raw_line in input.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line).trim();
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim().to_string();
            let mut val = v.trim().to_string();
            let bytes = val.as_bytes();
            if bytes.len() >= 2
                && ((bytes[0] == 34 && bytes[bytes.len() - 1] == 34)
                    || (bytes[0] == 39 && bytes[bytes.len() - 1] == 39))
            {
                val = val[1..val.len() - 1].to_string();
            }
            if !key.is_empty() {
                map.insert(key, val);
            }
        }
    }
    map
}

pub fn diff_maps(
    left: &BTreeMap<String, String>,
    right: &BTreeMap<String, String>,
) -> BTreeMap<String, EntryChange> {
    let keys: BTreeSet<&String> = left.keys().chain(right.keys()).collect();
    let mut out = BTreeMap::new();
    for k in keys {
        match (left.get(k), right.get(k)) {
            (None, Some(n)) => {
                out.insert(k.clone(), EntryChange::Added(n.clone()));
            }
            (Some(o), None) => {
                out.insert(k.clone(), EntryChange::Removed(o.clone()));
            }
            (Some(o), Some(n)) if o != n => {
                out.insert(
                    k.clone(),
                    EntryChange::Modified {
                        old: o.clone(),
                        new: n.clone(),
                    },
                );
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_comments_exports_and_quotes() {
        let dq = char::from(34);
        let sq = char::from(39);
        let src = format!("# comment
export PORT=8080
HOST={dq}127.0.0.1{dq}
MODE={sq}prod{sq}
");
        let parsed = parse_kv(&src);
        assert_eq!(parsed.get("PORT").map(String::as_str), Some("8080"));
        assert_eq!(parsed.get("HOST").map(String::as_str), Some("127.0.0.1"));
        assert_eq!(parsed.get("MODE").map(String::as_str), Some("prod"));
    }

    #[test]
    fn detects_added_removed_and_modified_keys() {
        let a = parse_kv("A=1
B=2
C=3
");
        let b = parse_kv("A=1
B=9
D=4
");
        let d = diff_maps(&a, &b);
        assert_eq!(d.len(), 3);
        assert_eq!(d.get("B"), Some(&EntryChange::Modified { old: "2".into(), new: "9".into() }));
        assert_eq!(d.get("C"), Some(&EntryChange::Removed("3".into())));
        assert_eq!(d.get("D"), Some(&EntryChange::Added("4".into())));
    }
}
