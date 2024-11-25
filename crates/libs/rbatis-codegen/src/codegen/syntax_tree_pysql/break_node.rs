use crate::codegen::syntax_tree_pysql::AsHtml;
use crate::codegen::syntax_tree_pysql::Name;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BreakNode {}

impl AsHtml for BreakNode {
    fn as_html(&self) -> String {
        "<break/>".to_string()
    }
}

impl Name for BreakNode {
    fn name() -> &'static str {
        "break"
    }
}
