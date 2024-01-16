// SPDX-License-Identifier: Apache-2.0

//! This crate provides a [handlebars] helper function which repeats a block
//! a given number of times (the `count`). For example:
//!
//! ```notrust
//! {{#repeat 3}}
//! hi
//! {{/repeat}}
//! ```
//!
//! Produces:
//!
//! ```notrust
//! hi
//! hi
//! hi
//! ```
//!
//! ## Local Variables
//!
//! Within the repeated block, there are three local variables in addition to
//! the standard context:
//!
//! 1. `@index` is an integer indicating the index of the current repetition.
//! 2. `@first` is a boolean indicating whether this is the first repetation.
//! 3. `@last` is a boolean indicating whether this is the last repetation.
//!
//! For example:
//!
//! ```notrust
//! {{#repeat 3}}
//! Index: {{@index}} (first: {{@first}}; last: {{@last}})
//! {{/repeat}}
//! ```
//!
//! Produces:
//!
//! ```notrust
//! Index: 0 (first: true; last: false)
//! Index: 1 (first: false; last: false)
//! Index: 2 (first: false; last: true)
//! ```
//!
//! ## Inverse Block
//!
//! Like the standard `each` helper function, `repeat` can specify an inverse
//! block which will be rendered when `count == 0`. For example:
//!
//! ```notrust
//! {{#repeat 0}}
//! foo
//! {{else}}
//! bar
//! {{/repeat}}
//! ```
//!
//! Produces:
//!
//! ```notrust
//! bar
//! ```
//!
//! [handlebars]: https://github.com/sunng87/handlebars-rust

#![deny(clippy::all)]
#![deny(missing_docs)]

use handlebars::*;

/// The `repeat` handler object
///
/// To use, register it in your handlebars registry:
///
/// ```rust
/// let mut reg = handlebars::Handlebars::new();
/// reg.register_helper("repeat", Box::new(handlebars_repeat::RepeatHelper));
/// ```
#[derive(Clone, Copy)]
pub struct RepeatHelper;

impl HelperDef for RepeatHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("repeat", 0))?
            .value();

        let count = value.as_u64().ok_or_else(|| {
            RenderErrorReason::ParamTypeMismatchForName(
                "repeat",
                "0".to_string(),
                "u64".to_string(),
            )
        })?;

        let template = h
            .template()
            .ok_or_else(|| RenderErrorReason::BlockContentRequired)?;

        for i in 0..count {
            let mut block = rc.block().cloned().unwrap_or_default();
            block.set_local_var("index", i.into());
            block.set_local_var("first", (i == 0).into());
            block.set_local_var("last", (i == count - 1).into());
            rc.push_block(block);

            template.render(r, ctx, rc, out)?;

            rc.pop_block();
        }

        if count == 0 {
            if let Some(template) = h.inverse() {
                template.render(r, ctx, rc, out)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use serde_json::json;

    const T: &str =
        "{{#repeat count}}{{name}}:{{@index}}:{{@first}}:{{@last}} {{else}}bar{{/repeat}}";

    #[inline]
    fn render(template: &str, count: u64) -> Result<String, RenderError> {
        let data = json!({"name": "foo", "count": count});

        let mut reg = Handlebars::new();
        reg.register_helper("repeat", Box::new(RepeatHelper));
        reg.render_template(template, &data)
    }

    #[rstest]
    #[case(0, "bar")]
    #[case(1, "foo:0:true:true ")]
    #[case(2, "foo:0:true:false foo:1:false:true ")]
    #[case(3, "foo:0:true:false foo:1:false:false foo:2:false:true ")]
    fn success(#[case] count: u64, #[case] output: &str) {
        assert_eq!(render(T, count).unwrap(), output);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    fn missing_arg(#[case] count: u64) {
        let template = "{{#repeat}}{{name}}{{/repeat}}";
        let err = render(template, count).unwrap_err();
        assert!(matches!(
            err.reason(),
            RenderErrorReason::ParamNotFoundForIndex("repeat", 0)
        ))
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    fn wrong_arg_type(#[case] count: u64) {
        let template = "{{#repeat \"foo\"}}{{name}}{{/repeat}}";
        let err = render(template, count).unwrap_err();
        assert!(
            matches!(err.reason(), RenderErrorReason::ParamTypeMismatchForName(
            "repeat",
            a,
            b,
        ) if (a == &"0".to_string()) && b == &"u64".to_string())
        )
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    fn missing_block(#[case] count: u64) {
        let template = "{{repeat count}}";
        let err = render(template, count).unwrap_err();
        assert!(matches!(
            err.reason(),
            RenderErrorReason::BlockContentRequired
        ))
    }
}
