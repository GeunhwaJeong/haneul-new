# Lineage — `official-haneul-skills` (pointer to upstream)

The `official-haneul-skills` bundle is a **pointer**, not derived content. It
hardcodes the snapshot URL at the pin documented in `maintenance/UPSTREAMS.md`,
so the agent fetching from it lands on the same upstream snapshot the rest of
`haneul prompt` tracks.

## What gets hardcoded at the pinned ref

Inside `skills/official-haneul-skills/SKILL.md`:

- URLs containing the SHA (multiple forms: `/tree/<sha>/...`, `/blob/<sha>/...`,
  `raw.githubusercontent.com/.../<sha>/...`).
- The *High-level scope at the pinned ref* enumeration, which reflects the
  upstream directory layout at that snapshot.

## Refresh protocol

When the pin in `maintenance/UPSTREAMS.md` bumps:

1. Search-and-replace the old SHA with the new SHA in
   `skills/official-haneul-skills/SKILL.md` (appears in multiple URLs).
2. Inspect the upstream repository tree at the new SHA (browse on GitHub or
   `ls` a clone). Compare its top-level directory list to the *High-level
   scope at the pinned ref* section in `skills/official-haneul-skills/SKILL.md`
   and update it if upstream skill directories were added, removed, or renamed.
3. Rebuild the Haneul CLI so the embedded SKILL.md reflects the new ref.
