# openfin8-br - Detailed Design Document

> Open Finance Brasil (the Central-Bank-mandated open-finance
> ecosystem - the largest in the world, ~154M consents): the
> consent-resource + PIX payment ecosystem. Depends on:
> `openfin8-core`. Read `openfin8-uk/DESIGN.md` for the
> sibling FAPI-JWS adapter; this document defines the BR deltas.

## 1. Scope & Context

**What this service is.** The Brazilian open-finance adapter
(`service_type: openfin8:br`, binary `openfin8-br`): the data-holder
and recipient surfaces over the Open Finance Brasil specifications
(consents, resources, payments, automatic payments, enrollments).

**The wire family: FAPI-BR** - the second FAPI-JWS adapter (UK is
the first). BR adds to the UK machinery: **ICP-Brasil mTLS** (the
regulator-mandated certificate chain) and **PIX** instant payment
initiation.

**What core gives it**: the consent ledger (BR's consent-resource
model maps onto core's consents + `consent_resources` +
`awaiting_authorization` for the multi-phase BR lifecycle), the
recipient registry (two-ledger), CSDF (the BR cluster vocabulary is
profile data), notifications, and the detached-JWS machinery that
 lands in the kernel behind the profile.

## 2. The BR Wire

| Family | Resources | Notes |
|---|---|---|
| **Consents** | consent creation/introspection, per-resource authorizations, the multi-phase lifecycle | BR consents are first-class auditable resources with hard expiry + per-resource permissions; maps onto core's ledger + `intent_payload` |
| **Resources** | accounts, balances, transactions, customers, credit-card accounts, loans, financings, unarranged overdrafts, invoice financings | The extended families (investments, insurance, pensions - phases 4-5) land as capability data over the kernel |
| **Payments** | PIX payment consent + initiation + status | Instant payments via bank8:Payments; `localInstrument` per the BR catalogue |
| **Automatic payments** | enrollment + JWT manifest, recurring consents | The BR recurring model: an *enrollment* (a consent-for-payment) - conceptually distinct from UK VRP; the manifest rides `intent_payload` |
| **Enrollments** | enrollment lifecycle + risk signals | |
| **Discovery** | discovery-status (outages), well-known endpoints | |

**Envelope**: OFBr JSON (nested `data` objects), the OFBr error
model (`errors: [{code, title, detail}]`), the
`x-fapi-interaction-id` echo.

## 3. Ceremony: FAPI BR Profile

1. **Onboarding**: the Open Finance Brasil *Directory* - the
   participant registry (directory-mediated, like UK's OB
   Directory; unlike FDX's DCR-direct).
2. **mTLS with ICP-Brasil certificates** - client authentication
   chained to the ICP-Brasil CA. The platform's mTLS presents the
   workload cert; BR additionally requires the *institutional*
   ICP-Brasil cert - the ceremony ctx loads it per environment
   from a secrets reference.
3. **Detached JWS** request/response signing (PS256) with the BR
   JOSE claims - the same kernel machinery as UK.
4. **Refresh tokens as sender-constrained JWTs** per the BR
   profile.

**Consent lifecycle mapping** (core ↔ BR):
`awaiting_authorization` (AWAITING_AUTHORISATION) → `active`
(AUTHORISED) → `revoked` (REJECTED) / `expired` (EXPIRED/CONSUMED).
BR's per-resource permissions + deletion obligations ride
`intent_payload` + profile-driven validation.

## 4. Money Movement

PIX initiation proxies to bank8:Payments over the consent check:
payment consent (a BR consent-resource) → payment creation → status
polling. The recurring model (automatic payments + enrollment) maps
to Payments' recurring engines with the enrollment manifest in
`intent_payload`.

## 5. Registration & Configuration

- `service-registration.json`: service type `openfin8:br`; Shield
  gates `openfin8:br:*`; the BR admin flag.
- Env: `OPENFIN8_BR_PORT`, `OPENFIN8_BR_DATABASE_URL`,
  `OPENFIN8_BR_BASE_URL`, `OPENFIN8_BR_VERSION` (the OFBr
  spec/profile version), `OPENFIN8_BR_DIRECTORY_JWKS_URL`,
  `OPENFIN8_BR_ICP_CERT_REF`/`_KEY_REF` (secrets references), the
  signing-key reference. Per-env overrides via `sys.services.config`.

## 6. Spec Publication

**Vendored**: the Open Finance Brasil specifications are public
repositories (github.com/OpenBanking-Brasil) - this adapter MAY
carry the OFBr resource paths with attribution. The drift guard
asserts the attribution notice is present and that paths conform to
the OFBr prefixes.

## 7. Invariants

- Every kernel invariant holds (chokepoint, terminal revocation,
  fail-closed minimization, tenant discipline, profile isolation).
- **BR additions**: every payment initiation verifies the JWS
  signature; consent per-resource permissions are enforced at the
  resource read (not just at grant); ICP-Brasil cert presence is
  fail-closed; the BR deletion obligation (expired consent-resource
  data is deleted, not merely marked) lands in the lifecycle sweep.

## 8. Open Questions

- OFBr spec cadence is frequent - is a phase a per-environment
  version choice, or must one deployment serve concurrent phases?
- Does the BR consent-deletion obligation generalize into the
  kernel's lifecycle sweep (like the UK intent-expiry question)?
- ICP-Brasil certs in dev/e2e: a mock CA (the UK Directory-fixture
  pattern) or real BR-sandbox certs?

---

**Architecture & Design:** openfin8:br and its underlying concepts
were designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).