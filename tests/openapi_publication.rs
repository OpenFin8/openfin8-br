// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:BR
// SPDX-License-Identifier: Apache-2.0

//! OpenAPI publication drift guard (OBARCH-2 - the vendored case).
//!
//! The Open Finance Brasil specifications are published as public repositories (github.com/OpenBanking-Brasil)
//! so the published spec MAY carry the OFBR resource paths. The
//! invariants are therefore:
//!
//! 1. The OFBR attribution/copyright notice is present in the
//!    spec's info block (the licence requires it to travel with any
//!    copied material).
//! 2. Standard-surface paths (when the wire lands, OBARCH-6) conform
//!    to the OFBR prefixes; nothing masquerades as another
//!    standard's surface (no /fdx/, no /open-banking/ leaks here).
//! 3. The OpenFin8-owned operation floor holds.

use serde_json::Value;

const OFBR_NOTICE_MARKER: &str = "Open Finance Brasil";
const OTHER_STD_PREFIXES: &[&str] = &["/fdx/", "/open-banking/"];

#[test]
fn openapi_publication_policy_holds() {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/openapi.json"))
        .expect("docs/openapi.json - regenerate via --print-registration --section=openapi");
    let spec: Value = serde_json::from_str(&text).expect("valid JSON");

    // Invariant 1: the OFBR notice travels with the vendored spec.
    let description = spec
        .pointer("/info/description")
        .and_then(Value::as_str)
        .unwrap_or_default();
    assert!(
        description.contains(OFBR_NOTICE_MARKER),
        "published spec must carry the OFBR specification attribution notice"
    );

    // Invariant 2: no other standard's surface leaks into this spec.
    let paths = spec
        .get("paths")
        .and_then(Value::as_object)
        .expect("paths object");
    for path in paths.keys() {
        for prefix in OTHER_STD_PREFIXES {
            assert!(
                !path.starts_with(prefix),
                "published spec must not carry another standard's surface path {path}"
            );
        }
    }

    // Invariant 3: OpenFin8-owned operation floor.
    let ops: usize = paths
        .values()
        .map(|v| {
            v.as_object()
                .map(|o| o.keys().filter(|k| *k != "parameters").count())
                .unwrap_or(0)
        })
        .sum();
    assert!(
        ops >= 1,
        "spec floor: expected >= 1 OpenFin8-owned op, got {ops}"
    );
}
