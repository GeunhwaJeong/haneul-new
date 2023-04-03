---
title: Write Smart Contracts with Haneul Move
---

Welcome to the Haneul tutorial for building smart contracts with [Haneul Move](../learn/why-move).
This tutorial provides a brief explanation of the Haneul Move language, and includes concrete examples to demonstrate how you can use Move in Haneul.


## About Haneul Move

Haneul Move is an open source language for writing safe smart contracts. It was originally developed at Facebook to power the [Diem](https://github.com/diem/diem) blockchain. However, Haneul Move was designed as a platform-agnostic language to enable common libraries, tooling, and developer communities across
blockchains with vastly different data and execution models.


The documentation for the original Move language is available in the [Move GitHub](https://github.com/move-language/move) repository and includes a [tutorial](https://github.com/move-language/move/blob/main/language/documentation/tutorial/README.md) and a [book](https://github.com/move-language/move/blob/main/language/documentation/book/src/SUMMARY.md) describing language features in detail. These are invaluable resources to deepen your understanding of the Move language but not strict prerequisites to following the Haneul tutorial, which we strived to make self-contained. Further, Haneul Move differs in some ways from Move, which we explore here.

You can use Haneul Move to define, create, and manage programmable [Haneul objects](../objects.md) representing user-level assets. Haneul's object system is implemented by adding new functionality to Move while also imposing additional restrictions, creating a dialect of Move (a.k.a. *Haneul Move*) that
makes certain parts of the original Move documentation not applicable to smart contract development in Haneul. Consequently, it's best to follow this tutorial and the relevant Move documentation links within.

Before looking at the Move code included with Haneul, let's talk briefly about Move code organization, which applies both to code included with
Haneul and the custom code developers write.


## Move code organization

The main unit of Move code organization (and distribution) is a _package_. A package consists of a set of _modules_ defined in separate
files with the `.move` extension. These files include Move functions and type definitions. A package must include the `Move.toml` manifest file
describing package configuration, such as package metadata and package dependencies. See [Move.toml](manifest.md) for more information about package manifest files in Haneul Move. Packages also include an auto-generated `Move.lock` file. The `Move.lock` file is similar in format to the package manifest, but is not meant for users to edit directly. See [Move.lock](lock-file.md) for more information about the lock file in Haneul Move. 

The minimal package source directory structure looks as follows and contains the manifest file, the lock file, and the `sources` subdirectory where one or more module files are located:

```
my_move_package
├── Move.lock
├── Move.toml
├── sources
    ├── my_module.move
```

See [Package Layout and Manifest Syntax](https://github.com/move-language/move/blob/main/language/documentation/book/src/packages.md#package-layout-and-manifest-syntax) for more information on package layout.

It's now time to look at some Haneul Move code. You can either keep reading for an introductory description of the main Haneul Move language constructs or you can jump straight into the code by [writing a simple Haneul Move package](write-package.md), and checking out additional code [examples](../../explore/examples.md).

## First look at Move source code

The Haneul platform includes the Haneul Framework, which includes the core on-chain libraries that Haneul Move developers  need to bootstrap Haneul operations. In particular, Haneul supports multiple user-defined coin types, which are custom assets the Haneul Move language defines. Haneul Framework code contains the `Coin` module supporting creation and management of custom coins. The `Coin` module is located in the [coin.move](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-framework/packages/haneul-framework/sources/coin.move) file. As you might expect, the manifest file describing how to build the package containing the `Coin` module is located in the corresponding
[Move.toml](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/packages/haneul-framework/Move.toml) file.

Let's see how module definition appears in the `Coin` module file:

```rust
module haneul::coin {
...
}
```

Don't worry about the rest of the module contents for now; you can read more about [modules](https://github.com/move-language/move/blob/main/language/documentation/book/src/modules-and-scripts.md#modules) in the Move book later.

**Important:** In Haneul Move, package names are always in PascalCase, while the address alias is lowercase, for example `haneul = 0x2` and `std = 0x1`. So: `Haneul` = name of the imported package (Haneul = haneul framework), `haneul` = address alias of `0x2`, `haneul::haneul` = module haneul under the address `0x2`, and `haneul::haneul::HANEUL` = type in the module above.

When you define a module, specify the module name (`coin`) preceded by the name of the package where this module resides (`haneul`). The combination of the package name and the module name uniquely identifies a module in Haneul Move source code. The package name is globally unique, but different packages can contain modules with the same name. While module names are not unique, when they combine with their unique package name they result in a unique combination.

For example, if you have a published package "P", you cannot publish an entirely different package also named "P". At the same time you can have module "P1::M1", "P2::M1", and "P1::M2" but not another, say, "P1::M1" in the system at the same time.

While you can't name different packages the same, you can upgrade a package on chain with updated code using the same package name.  

In addition to having a presence at the source code level, as discussed in [Haneul Move code organization](#move-code-organization), a
package in Haneul is also a Haneul object and must have a unique numeric ID in addition to a unique name, which is assigned in the manifest file:

```
[addresses]
haneul = "0x2"
```

### Haneul Move structs

The `Coin` module defines the `Coin` struct type that you can use to represent different types of user-defined coins as Haneul objects:

``` rust
struct Coin<phantom T> has key, store {
    id: UID,
    value: u64
}
```

Haneul Move's struct type is similar to struct types defined in other programming languages, such as C or C++, and contains a name and a set of typed fields. In particular, struct fields can be of a primitive type, such as an integer type, or of a struct type.

You can read more about Move [primitive types](https://github.com/move-language/move/blob/main/language/documentation/book/src/SUMMARY.md#primitive-types) and [structs](https://github.com/move-language/move/blob/main/language/documentation/book/src/structs-and-resources.md) in the Move book.

For a Haneul Move struct type to define a Haneul object type, such as `Coin`, its first field must be `id: UID`, which is a
struct type defined in the [object module](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-framework/packages/haneul-framework/sources/object.move). The Move struct type must also have the `key` ability, which allows Haneul's global storage to persist the object. Abilities of a Move struct are listed after the `has` keyword in the struct definition, and their existence (or lack thereof) helps the compiler enforce various properties on a definition or on instances of a given struct.

You can read more about struct [abilities](https://github.com/move-language/move/blob/main/language/documentation/book/src/abilities.md) in the Move book.

The reason that the `Coin` struct can represent different types of coin is that the struct definition is parameterized with a type parameter. When you create an instance of the `Coin` struct, you can pass it an arbitrary concrete Move type (e.g. another struct type) to distinguish different types of coins from one another.

Learn about Move type parameters known as [generics](https://github.com/move-language/move/blob/main/language/documentation/book/src/generics.md) and the optional [phantom keyword](https://github.com/move-language/move/blob/main/language/documentation/book/src/generics.md#phantom-type-parameters) at your leisure.

In particular, one type of custom coin already defined in Haneul is `Coin<HANEUL>`, which represents a token used to pay for Haneul
computations (more generally known as _gas_) - in this case, the concrete type used to parameterize the `Coin` struct is the `HANEUL` struct in the [HANEUL module](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-framework/packages/haneul-framework/sources/haneul.move):

``` rust
struct HANEUL has drop {}
```

The [Write a Haneul Move Package](write-package.md) topic shows how to define and instantiate custom structs.

### Move functions

Similar to other popular programming languages, the main unit of computation in Move is a function. Let us look at one of the simplest functions defined in the [Coin module](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-framework/packages/haneul-framework/sources/coin.move), that is the `value` function.

``` rust
public fun value<T>(self: &Coin<T>): u64 {
    self.value
}
```

Functions in other modules can call this _public_ function to return the unsigned integer value currently stored in a given
instance of the `Coin` struct. The Move compiler allows direct access to fields of a struct only within the module defining a given struct, as described in [Privileged Struct Operations](https://github.com/move-language/move/blob/main/language/documentation/book/src/structs-and-resources.md#privileged-struct-operations). The body of the function simply retrieves the `value` field from the `Coin` struct instance parameter and returns it. The coin parameter is a read-only reference to the `Coin` struct instance, indicated by the `&` preceding the parameter type. Move's type system enforces an invariant that struct instance arguments passed by read-only references (as opposed to mutable references) cannot be modified in the body of a function.

You can read more about Move [references](https://github.com/move-language/move/blob/main/language/documentation/book/src/references.md#references) in the Move book.

The [Write a Haneul Move Package](write-package.md) topic shows how to call Move functions from other functions and how
to define the new ones.

The Haneul dialect of the Move language also defines _entry functions_. These must satisfy a certain set of properties and you can call them directly from Haneul (e.g., from a Haneul application written in a different language).

#### Entry functions

One of the basic operations in Haneul is a gas object transfer between [addresses](https://github.com/move-language/move/blob/main/language/documentation/book/src/address.md) representing individual users. The gas object transfer implementation in the [HANEUL module](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-framework/packages/haneul-framework/sources/haneul.move) is also an example of the use of an entry function:

```rust
public entry fun transfer(c: coin::Coin<HANEUL>, recipient: address, _ctx: &mut TxContext) {
    ...
}
```

Don't worry about the function body for now - because the function is part of Haneul framework, you can trust
that it will do what it is intended to do.

In general, an entry function must satisfy the following properties:

- Has the `entry` modifier. The visibility does not matter. The function can be `public`, `public(friend)`, or `internal`.
- Has no return value
- (Optional) Has a mutable reference to an instance of the `TxContext` struct defined in the [TxContext module](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-framework/packages/haneul-framework/sources/tx_context.move) as the last parameter.

More concretely, the `transfer` function is `public`, has no return value, and has three parameters:

- `c` - Represents a gas object whose ownership is to be transferred.
- `recipient` - The [address](https://github.com/move-language/move/blob/main/language/documentation/book/src/address.md) of the intended recipient
- `_ctx` - A mutable reference to an instance of the `TxContext` struct (in this particular case, this parameter is not actually used in the function's body as indicated by its name starting with `_`). Because it is unused, the parameter could be removed. The mutable reference to the `TxContext` is optional for entry functions.

[Calling Move code](../cli-client.md#calling-move-code) describes how to call the `transfer` function from the Haneul CLI client.
