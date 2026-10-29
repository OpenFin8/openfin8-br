// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:BR
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 OpenFin8:BR - Brazilian open finance: the Open Finance Brasil
//! surface (consents, resources, PIX payments, automatic
//! payments/enrollments) - the Central-Bank-mandated ecosystem. See
//! `DESIGN.md`.
//!
//! This service embeds `openfin8-core` (the jurisdiction-agnostic
//! kernel) and supplies the BR profile, the FAPI-BR ceremony (Open Finance Brasil
//! Directory + ICP-Brasil mTLS + detached JWS signing), and the OFBr
//! resource router with the consent-resource + PIX wire. See
//! `DESIGN.md` §2-§5 for the wire, the ceremony, and the served
//! surface.
//!
//! Spec publication is the vendored case: the Open Finance
//! Brasil specifications are public repositories, so the published
//! `docs/openapi.json` MAY carry the OFBr resource paths with their
//! attribution preserved (see `src/openapi.rs`).

use utoipa::OpenApi;

mod openapi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sentin8::shell::serve(
        sentin8::shell::Sentin8Shell::new("openfin8:br", "OPENFIN8_BR")
            .with_registration(print_registration_stub)
            .with_own_health()
            .with_openapi(openapi::BROpenApi::openapi()),
    )
    .await
}

/// The registration data (policy sets, Shield gates `openfin8:br:*`
/// role-scoped, admin UI pages, SQL migrations, the schedules) - the
/// per-jurisdiction identity shape follows `openfin8-us/registration.rs`
/// (DESIGN.md §5).
fn print_registration_stub(section: Option<String>) -> anyhow::Result<()> {
    // The full registration module carries it (DESIGN.md §5).
    let _ = section;
    Ok(())
}
