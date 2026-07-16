# M — Test-only code leakage (+ off-chain appendix)

### SM-M1 — Test/debug helper not gated by `#[test_only]`   [Critical]
Invariant: every test/debug helper that bypasses normal authorization — `init_for_testing`,
`*_for_testing`, `mint_for_testing`, `destroy_*`, debug setters — is annotated `#[test_only]` (or
lives in a `#[test_only]` module) so it is compiled out of published bytecode.
Detect: functions named `*_for_testing` / `test_*` / `init_for_testing` / debug mint/setters that
lack `#[test_only]` and are `public`/`entry`; `#[test_only]` placed on the module-use but not the
helper, or vice-versa.
_Absence rule:_ `#[test_only]` does NOT survive into bytecode — walk every `public`/`entry`
fn whose name pattern (`*_for_testing`, `test_*`, debug setters) or unauthorized privileged
body suggests test scaffolding. The shape (public + unauthorized + privileged body) is the
signal, not the missing annotation.
Exploit: the helper ships on-chain as an unauthenticated mint / admin-bypass / object-spawn
function → unlimited mint or authority seizure in production.
Source: `HaneulLabs/skills → move-unit-testing/SKILL.md`.

---

## Off-chain appendix (non-blocking — scope is on-chain Move, but flag if encountered)

These are out of primary scope (auditing on-chain Move code) but matter when a full
integration is in view. Flag, don't deep-dive.

- **Type-anchoring.** Object/type queries must use the **original** package ID (struct types stay
  anchored there after upgrades); function calls use the upgraded ID. Mixing them silently
  returns no results. Source: `HaneulLabs/skills → haneul-publish/SKILL.md`, `HaneulLabs/skills → frontend-apps/`.
- **Read-after-write.** `await client.waitForTransaction({ digest })` before reading from another
  node or invalidating caches; a returned digest is not proof of success (check
  `result.$kind === 'FailedTransaction'`). Source: `HaneulLabs/skills → accessing-data/grpc.md`,
  `HaneulLabs/skills → frontend-apps/`.
- **No secrets in the browser.** Private keys, mnemonics, admin/signing keys, gas-station coins,
  and indexer DB credentials never live in frontend code. Source: `HaneulLabs/skills → frontend-apps/limitations.md`.
- **Amount precision.** Wrap large GEUNHWA amounts in `BigInt`; bare JS numbers lose precision above
  2^53 → silent over/underpayment. Source: `HaneulLabs/skills → ptbs/building.md`.
- **Supply chain.** Prefer MVR / pinned `Move.lock` revisions over floating git deps to resist
  named-address hijacking; verify published bytecode matches source. Source:
  `HaneulLabs/skills → haneul-move-project/SKILL.md`, `HaneulLabs/skills → haneul-build-test/SKILL.md`.
- **Network targeting.** Confirm `haneul client active-env` before any mainnet publish/operation.
  Source: `HaneulLabs/skills → haneul-cli/SKILL.md`.
