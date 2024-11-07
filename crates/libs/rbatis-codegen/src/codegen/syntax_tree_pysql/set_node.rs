use crate::codegen::syntax_tree_pysql::Name;
use crate::codegen::syntax_tree_pysql::NodeType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl Name for SetNode {
    fn name() -> &'static str {
        "set"
    }
}
