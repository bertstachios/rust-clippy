use crate::syntax::ast::{Expr, ExprKind, UnOp};
use crate::rustc::lint::{EarlyContext, EarlyLintPass, LintArray, LintPass};
use crate::rustc::{declare_tool_lint, lint_array};
use if_chain::if_chain;
use crate::utils::{snippet, span_lint_and_sugg};

/// **What it does:** Checks for usage of `*&` and `*&mut` in expressions.
///
/// **Why is this bad?** Immediately dereferencing a reference is no-op and
/// makes the code less clear.
///
/// **Known problems:** Multiple dereference/addrof pairs are not handled so
/// the suggested fix for `x = **&&y` is `x = *&y`, which is still incorrect.
///
/// **Example:**
/// ```rust
/// let a = f(*&mut b);
/// let c = *&d;
/// ```
declare_clippy_lint! {
    pub DEREF_ADDROF,
    complexity,
    "use of `*&` or `*&mut` in an expression"
}

pub struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(DEREF_ADDROF)
    }
}

fn without_parens(mut e: &Expr) -> &Expr {
    while let ExprKind::Paren(ref child_e) = e.node {
        e = child_e;
    }
    e
}

impl EarlyLintPass for Pass {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, e: &Expr) {
        if_chain! {
            if let ExprKind::Unary(UnOp::Deref, ref deref_target) = e.node;
            if let ExprKind::AddrOf(_, ref addrof_target) = without_parens(deref_target).node;
            then {
                span_lint_and_sugg(
                    cx,
                    DEREF_ADDROF,
                    e.span,
                    "immediately dereferencing a reference",
                    "try this",
                    format!("{}", snippet(cx, addrof_target.span, "_")),
                );
            }
        }
    }
}

/// **What it does:** Checks for references in expressions that use
/// auto dereference.
///
/// **Why is this bad?** The reference is a no-op and is automatically
/// dereferenced by the compiler and makes the code less clear.
///
/// **Example:**
/// ```rust
/// struct Point(u32, u32);
/// let point = Foo(30, 20);
/// let x = (&point).x;
/// ```
declare_clippy_lint! {
    pub REF_IN_DEREF,
    complexity,
    "Use of reference in auto dereference expression."
}

pub struct DerefPass;

impl LintPass for DerefPass {
    fn get_lints(&self) -> LintArray {
        lint_array!(REF_IN_DEREF)
    }
}

impl EarlyLintPass for DerefPass {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, e: &Expr) {
        if_chain! {
            if let ExprKind::Field(ref object, ref field_name) = e.node;
            if let ExprKind::Paren(ref parened) = object.node;
            if let ExprKind::AddrOf(_, ref inner) = parened.node;
            then {
                span_lint_and_sugg(
                    cx,
                    REF_IN_DEREF,
                    object.span,
                    "Creating a reference that is immediately dereferenced.",
                    "try this",
                    format!(
                        "{}.{}",
                        snippet(cx, inner.span, "_"),
                        snippet(cx, field_name.span, "_")
                    )
                );
            }
        }
    }
}
