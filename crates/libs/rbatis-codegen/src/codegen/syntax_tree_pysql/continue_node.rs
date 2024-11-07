use crate::codegen::syntax_tree_pysql::AsHtml;
use crate::codegen::syntax_tree_pysql::Name;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinueNode {}

impl AsHtml for ContinueNode {
    fn as_html(&self) -> String {
        format!("<continue />")
    }
}

impl Name for ContinueNode {
    fn name() -> &'static str {
        "continue"
    }
}
