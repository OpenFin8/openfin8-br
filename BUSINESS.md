# OpenFin8:BR - Business Requirements & Functionality

> This BRD records the business scope the Brazilian adapter
> delivers; section 9 lists the acceptance criteria.
>
> Audience: product owners, business analysts. For architecture read
> `README.md`; for wire-level detail read `DESIGN.md`.

## 1. Business Purpose

OpenFin8:BR is **Brazilian open finance**: the Open Finance Brasil
surface (Central-Bank mandated; the largest open-finance ecosystem
in the world, ~154M consents). It delivers:

- **The mandated consent-resource model** - BR consents are
  first-class, auditable resources with hard expiry, per-resource
  permissions, and deletion obligations - the strictest consent
  regime in the adapter set, fully expressible over the kernel.
- **PIX payment initiation** - instant payments, the ecosystem's
  centerpiece, through the same one-executor model (bank8:Payments).
- **The recurring-payment model (automatic payments + enrollments)**
  - BR's enrollment manifest concept: a consent-for-payment,
  distinct from UK VRP, riding the kernel's intent payload.
- **The FAPI-BR security posture** - ICP-Brasil mTLS + detached
  JWS signing: what the Central Bank and Brazilian participants
  require.

## 2. Business Scope (What the Service Will Provide)

| # | Capability area | What the business gets |
|---|---|---|
| 1 | Consents | Multi-phase consent lifecycle, per-resource permissions, hard expiry + deletion obligations |
| 2 | Resources | Accounts, balances, transactions, customers, credit cards, loans, financings, overdrafts, invoice financings (+ the phase 4-5 families as capability data) |
| 3 | Payments | PIX consent + initiation + status polling |
| 4 | Automatic payments | Enrollment + JWT manifest recurring consents |
| 5 | Enrollments | Enrollment lifecycle + risk signals |
| 6 | Notifications | The kernel worker under the BR envelope |
| 7 | Admin console | The kernel pages under the BR identity |
| 8 | Events | Family `openfin8:br:*` |
| 9 | Health | Fleet supervision |

**Not owned**: the implementation (openfin8-core), the data plane
(bank8:Ledger), execution (bank8:Payments), ceremony hosting
(Sentin8:Auth), other markets' surfaces (profile isolation).

## 3. Business Capabilities

### 3.1 The consent-resource model

BR consents enumerate *which resources* are shared (per-resource
permissions), expire hard, and carry data-deletion obligations when
expired - enforced at every resource read, not just at grant. The
kernel's `consent_resources` + intent payload + a lifecycle sweep
deliver it without a fork.

### 3.2 PIX over one executor

Payment initiation is consent-checked here, executed by
bank8:Payments - PIX is the flagship instant rail, and the
recurring model rides Payments' scheduled engines.

## 4. Business Events (Outbound)

Family `openfin8:br:*`: consent lifecycle, payment lifecycles,
enrollment events, revocations - consumed by recipient-side
reconciliation.

## 5. Business Rules & Invariants

- Every kernel invariant holds.
- Payment initiation verifies the JWS signature; consent
  per-resource permissions enforce at read; ICP-Brasil cert presence
  is fail-closed; expired consent-resource data is deleted, not
  merely marked.

## 6. Target-User Roles & Use Cases

| Role | Use case |
|---|---|
| Brazilian institution (data holder) | Enable `openfin8:br`; serve the mandated ecosystem |
| Fintech participant (recipient) | Register via the Directory, consume consented resources, initiate PIX |
| Bank customer | Authorizes per-resource consents; revocation honored immediately + deleted data |
| Central Bank / conformance | The BR conformance suite against the adapter |

## 7. KPIs / Success Criteria

- **Conformance**: the Open Finance Brasil conformance suites pass.
- **Market entry cost**: the Profile guarantee - enrollment +
  configuration.
- **Consent rigor**: the per-resource + deletion obligations
  enforced structurally.

## 8. Dependencies & Reliance on Other Services

The kernel + platform set, plus: the **Open Finance Brasil
Directory** (participant registry + JWKS trust) and the
**ICP-Brasil certificate chain** (institutional mTLS, secrets-
referenced per environment).

## 9. Acceptance Criteria (the implementation's definition of done)

- The publication guard passes (vendored with attribution).
- The BR consent-resource model over the kernel (per-resource
  permissions, expiry + deletion sweep).
- The FAPI-BR ceremony green (mTLS + JWS) against a test Directory +
  mock ICP CA.
- PIX initiation + the enrollment recurring model through Payments.
- The e2e journey gains the BR consent + PIX flows.

## 10. Open Business Questions

- Concurrent spec phases in one deployment, or phase-per-environment?
- Does the deletion obligation generalize into the kernel sweep?
- ICP-Brasil dev/e2e: mock CA (UK's fixture pattern) or BR sandbox
  certs?

---

**Architecture & Design:** openfin8:br and its underlying concepts
were designed by [Hamid Jahanzad](https://www.linkedin.com/in/hamidjahanzad/).