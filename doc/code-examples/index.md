---
title: Haneul Code Examples
---

This is a placeholder topic for Haneul code examples. When you add a new sample, create a section for the example, add a description and then the example.

## Code example title

This code example demonstrates how to do something in Haneul.

```rust
module haneul::dynamic_field {

public fun add<Name: copy + drop + store, Value: store>(
  object: &mut UID,
  name: Name,
  value: Value,
);

}
```

To add code formatting, specify the language for the example, such as *rust* in the preceding example.

