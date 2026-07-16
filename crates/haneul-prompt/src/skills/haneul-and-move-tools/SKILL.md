---
name: haneul-and-move-tools
description: >
  Use to get bytecode for a deployed Haneul package and produce a disassembled working view.
  One GraphQL call fetches every module's raw bytecode bytes; `haneul move disassemble`
  (already on the system, running `haneul prompt`) produces `.asm` files for analysis.
  Trigger on "fetch this package's bytecode", "get me the .mv for package X",
  "disassemble this package", or "I need to read a deployed Haneul package".
---

# Haneul and Move Tools

Get a deployed Haneul package's bytecode and produce `.mv` and `.asm` (disassembly) files.
One Haneul GraphQL call returns raw bytes for every module; `haneul move disassemble` produces
the working view module-by-module.

For what the disassembly conveys (and what's lost in compilation), see
`move-bytecode-comprehension`. The end-to-end procedure is in `fetch-and-disassemble.md`.
