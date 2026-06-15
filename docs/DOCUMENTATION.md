# Documentation

YARLint's documentation is made up entirely of rustdoc comments. Considering
this, all new additions to YARLint must have corectly formatted doc comments.

## Modules

Every module should have a module description block in the following format:

```rust
//! [Brief module description ( < a sentence )]
//! 
//! [In-depth description of the module's purpose/functionality]
```

## Functions

Every function within every modules should minimally have a function description
block similar to this example:

```rust
/// [Brief function description ( < a sentence )]
/// 
/// [In-depth description of the function's purpose/functionality]
```

If the function takes arguments, throws errors, or returns something, be sure
to include that information as shown below.

```rust
/// [Brief function description ( < a sentence )]
/// 
/// [In-depth description of the function's purpose/functionality]
/// 
/// # Arguments
/// 
/// * `[argument 1 name]` (`[argument 1 type]`) - [Text about argument]
/// * `[argument 2 name]` (`[argument 2 type]`) - [Text about argument]
/// 
/// # Returns
/// 
/// Returns [description of function's returned value]
/// 
/// # Errors
/// 
/// Returns an error if:
/// - [Error condition]
```

If a function is accompanied by an example, that example should be included in
the rustdoc as a codeblock below the in-depth description and before any other
sections of the doc comment.

```rust
/// [Brief function description ( < a sentence )]
/// 
/// [In-depth description of the function's purpose/functionality]
/// 
/// Example(s):
/// 
/// ```[format name]
/// [example contents]
/// ```
/// 
/// # Arguments
/// 
/// * `[argument 1 name]` (`[argument 1 type]`) - [Text about argument]
/// * `[argument 2 name]` (`[argument 2 type]`) - [Text about argument]
```

## Structs

Structs should be documented similarly to modules but with additionaly
descriptions given for each of their values as shown in the example below

```rust
/// [Brief struct description ( < a sentence )]
/// 
/// [In-depth description of the function's purpose/functionality]
#[attribute]
struct SampleStruct {
    /// [Brief value description]
    value1: String,

    /// [Brief value description]
    #[attribute]
    value2: bool,
}
```

Note that when attributes are present, they should always come after the doc
comment.

## Additional Notes

Keep in mind that rustdoc allows for the use of markdown formatting in doc
comments. The use of markdown formatting is encouraged so long as it improves
the descriptiveness of the documentation being written.

For examples of documentation as used in YARLint already, look at pre-existing
modules in the source, or reference the official rustdoc documentation.