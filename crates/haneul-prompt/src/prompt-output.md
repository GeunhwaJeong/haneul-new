# haneul prompt — expert Haneul and Move knowledge for AI agents

`haneul prompt` prints expert Haneul and Move knowledge from embedded skill bundles,
organized into **categories**.

## How to use this

Read the available categories (`haneul prompt categories`), try to match one to the
task, then drill into its bundles. Each skill is a two-tier bundle: `SKILL.md`
routes, reference files hold content. **Read every reference file** before
applying — `--all` loads them in one call.

```sh
haneul prompt categories                    # see the available categories
haneul prompt category <name> --all         # read every bundle's content in one call
haneul prompt category <name>               # read a category's workflow + skill list
haneul prompt category <name> --list        # list bundle and reference file names and sizes (no content)
```

Skills can also be reached directly:

```sh
haneul prompt skills                        # list all skill bundles, flat
haneul prompt skill <bundle> --all          # read SKILL.md + every reference file
haneul prompt skill <bundle>                # read a bundle's SKILL.md
haneul prompt skill <bundle> --list         # list reference file names and sizes (no content)
haneul prompt skill <bundle> --file <ref>   # read a specific reference file
```