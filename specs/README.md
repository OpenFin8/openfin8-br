# OpenFin8:BR - Upstream Specification References

**The Open Finance Brasil specifications may be vendored - with attribution.**

The Open Finance Brasil specifications are published as **public
repositories** (github.com/OpenBanking-Brasil - api-consents,
api-payments, api-automatic-payments, api-enrollments). This
repository MAY include OFBr material (the resource paths in the
published `docs/openapi.json`) with attribution preserved - the CI
drift guard enforces the attribution notice is present.

Optional local reference copies of the upstream repos, gitignored so
this repository does not drift from the published standards:

```
specs/ofbr/{version}/       # your local clone/copy of the OFBr spec repos
```

Operators pull them from <https://github.com/OpenBanking-Brasil> and
point conformance tooling at them. Nothing in this service reads
them at runtime - the served surface is configured via
`OPENFIN8_BR_VERSION`.

Contrast: the FDX (US/CA) adapters must NEVER publish the licensed
FDX specification - see their repositories.
