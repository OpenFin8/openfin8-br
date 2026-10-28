// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the OpenFin8:BR
// SPDX-License-Identifier: Apache-2.0

//! OpenAPI spec for OpenFin8:BR.
//!
//! OBARCH-2 - the VENDORED case: the Open Banking Standard is
//! published by Open Banking Limited under the MIT Open Licence
//! (openbanking.org.uk/open-licence), so this adapter MAY carry the
//! OB resource paths in its published spec **with the OBL copyright
//! notice preserved**. The utoipa aggregate below therefore grows
//! to include the OB resource annotations as the wire lands
//! (OBARCH-6), and the OBL notice travels in the spec's info block.
//!
//! The drift guard (`tests/openapi_publication.rs`) asserts the
//! notice is present and that OB paths conform to the versioned
//! prefix (`/open-banking/v{maj}.{min}/...`) - the inverse of the
//! US/CA policy.

use utoipa::OpenApi;

/// The OBL MIT Open Licence notice that MUST accompany any copied
/// Open Banking Standard material (spec paths derived from the R/W
/// API). Rendered into the published spec's info block - wired into
/// the utoipa description when the OB paths land (OBARCH-6); the
/// drift guard tests against the marker string.
#[allow(dead_code)]
pub const OFBR_SPEC_NOTICE: &str =
    "The Open Finance Brasil specifications are published as public \
repositories (github.com/OpenBanking-Brasil); resource paths here \
derive from them with attribution preserved.";

/// OpenFin8:BR OpenAPI surface: OpenFin8-owned routes plus (as the
/// wire lands) the OB resource paths under the OBL notice.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "openfin8-br",
        description = "Sentin8 OpenFin8:BR - Brazilian open finance (Open Finance Brasil: consent resources, PIX payments, enrollments). The Open Finance Brasil \
specifications are public (github.com/OpenBanking-Brasil); resource paths are carried with attribution preserved. \
Read/Write API v3.1.x/v4.0.1). The Open Banking Standard is \
published by Open Banking Limited under the MIT Open Licence \
(https://www.openbanking.org.uk/open-licence); portions of the \
Read/Write API surface included here are used under that licence \
with the OBL copyright notice preserved.",
        license(name = "MIT OR Apache-2.0", identifier = "MIT OR Apache-2.0"),
        version = "0.1.0"
    ),
    paths(),
    components(schemas()),
    security()
)]
pub struct BROpenApi;
