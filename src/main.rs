// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:BR
// SPDX-License-Identifier: Apache-2.0

//! Sentin8 OpenFin8:BR - Brazilian open finance: the Open Finance Brasil
//! surface (consents, resources, PIX payments, automatic
//! payments/enrollments) - the Central-Bank-mandated ecosystem. See
//! `DESIGN.md` (OBARCH-6).
//!
//! This service embeds `openfin8-core` (the jurisdiction-agnostic
//! kernel - OBARCH-1) and supplies the BR profile, the FAPI-BR ceremony (Open Finance Brasil
//! Directory + ICP-Brasil mTLS + detached JWS signing), and the OFBr
//! resource router with the consent-resource + PIX wire.
//!
//! Skeleton - the OBARCH-1 core extraction lands first; this binary
//! then takes the OB wire handlers, the intent-based consent model
//! over core's `awaiting_authorization` state, and its registration
//! data. Auth gains the FAPI 1 Advanced profile flag alongside
//! (sequenced with this adapter).
//!
//! Spec publication (OBARCH-2) is the vendored case: the Open Finance
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

/// Registration stub - the full registration data (policy sets,
/// Shield gates `openfin8:br:*` role-scoped, admin UI pages, SQL
/// migrations, the staged-intent expiry schedule) lands with the
/// OBARCH-1 extraction; the shape follows the `openfin8-us`
/// registration (per-jurisdiction identity).
fn print_registration_stub(section: Option<String>) -> anyhow::Result<()> {
    // Placeholder until the registration module lands (OBARCH-6).
    let _ = section;
    Ok(())
}
