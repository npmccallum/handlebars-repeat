[![Workflow Status](https://github.com/npmccallum/handlebars-repeat/workflows/test/badge.svg)](https://github.com/npmccallum/handlebars-repeat/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/npmccallum/handlebars-repeat.svg)](https://isitmaintained.com/project/npmccallum/handlebars-repeat "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/npmccallum/handlebars-repeat.svg)](https://isitmaintained.com/project/npmccallum/handlebars-repeat "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# handlebars-repeat

This crate provides a [handlebars] helper function which repeats a block
a given number of times (the `count`). For example:

```notrust
{{#repeat 3}}
hi
{{/repeat}}
```

Produces:

```notrust
hi
hi
hi
```

### Local Variables

Within the repeated block, there are three local variables in addition to
the standard context:

1. `@index` is an integer indicating the index of the current repetition.
2. `@first` is a boolean indicating whether this is the first repetation.
3. `@last` is a boolean indicating whether this is the last repetation.

For example:

```notrust
{{#repeat 3}}
Index: {{@index}} (first: {{@first}}; last: {{@last}})
{{/repeat}}
```

Produces:

```notrust
Index: 0 (first: true; last: false)
Index: 1 (first: false; last: false)
Index: 2 (first: false; last: true)
```

### Inverse Block

Like the standard `each` helper function, `repeat` can specify an inverse
block which will be rendered when `count == 0`. For example:

```notrust
{{#repeat 0}}
foo
{{else}}
bar
{{/repeat}}
```

Produces:

```notrust
bar
```

[handlebars]: https://github.com/sunng87/handlebars-rust

License: MIT
