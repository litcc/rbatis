use crate::codegen::syntax_tree_pysql::Name;
use crate::codegen::syntax_tree_pysql::NodeType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhereNode {
    pub childs: Vec<NodeType>,
}

impl Name for WhereNode {
    fn name() -> &'static str {
        "where"
    }
}
