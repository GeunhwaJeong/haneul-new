---
title: Programming Objects Tutorial Series
---

Haneul is a blockchain centered on [objects](../../learn/objects.md). Once you start programming non-trivial [smart contracts](../../build/move/index.md) on Haneul, you will begin dealing with Haneul objects in the code. Haneul includes a rich, comprehensive library and testing framework to allow you to interact with objects in a safe yet flexible way.

This tutorial series walks through all the powerful ways to interact with objects in [Haneul Move](../../learn/haneul-move-diffs.md). At the end, it also explores the designs of a few (close-to-)real-world examples to demonstrate the tradeoffs of using different object types and ownership relationships.

## Prerequisites

Understand:
- [Learn about Haneul](../../learn/about-haneul.md)
- [Smart Contracts with Move](../../build/move/index.md)
- [Haneul Objects](../../learn/objects.md)

Install:
- [Haneul binaries](../install.md#install-haneul-binaries)
- [Haneul source code](../install.md#source-code)

## Chapters

- [Chapter 1: Object Basics](../../build/programming-with-objects/ch1-object-basics.md)
  - Defining Move object types, creating objects, transferring objects.
- [Chapter 2: Using Objects](../../build/programming-with-objects/ch2-using-objects.md)
  - Passing Move objects as arguments, mutating objects, deleting objects.
- [Chapter 3: Immutable Objects](../../build//programming-with-objects/ch3-immutable-objects.md)
  - Freezing an object, using immutable objects.
- [Chapter 4: Object Wrapping](../../build/programming-with-objects/ch4-object-wrapping.md)
  - Wrapping objects in another object.
- [Chapter 5: Dynamic Fields](../../build/programming-with-objects/ch5-dynamic-fields.md)
  - Extending objects with fields that reference other objects, and that you can add, access, and remove dynamically.
- [Chapter 6: Collections](../../build/programming-with-objects/ch6-collections.md)
  - Working with objects in on-chain homogeneous and heterogeneous key-value stores built on top of dynamic fields.

## Limits on transactions, objects, and data

Haneul has some limits on transactions and data used in transactions, such as a maximum size and number of objects used. To view the full list of limits in source code, see [Transaction input limits](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-protocol-config/src/lib.rs#L154).
