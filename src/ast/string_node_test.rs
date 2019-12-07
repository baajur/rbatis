use crate::ast::string_node::StringNode;
use std::rc::Rc;

use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use crate::engines::rbatis_engine::node::NodeType::NString;
use crate::ast::node::SqlNode;
use serde_json::json;
use test::Bencher;
use crate::ast::node_config_holder::NodeConfigHolder;
use crate::engines::rbatis_engine::runtime::RbatisEngine;

#[test]
pub fn test_string_node(){
    let mut john = json!({
        "arg": 2,
    });
    let mut holder=NodeConfigHolder::new();
    let mut sNode = StringNode::new("arg+1=#{arg+1}");

    let r=sNode.eval(&mut john,&mut holder).unwrap();
    println!("{}",r);
}


#[bench]
fn bench_string_node(b: &mut Bencher) {
    let mut john = json!({
        "arg": 2,
    });

    let engine= rbatis_engine::new();

    let mut sNode = StringNode::new("arg+1=#{arg}");
    let mut holder=NodeConfigHolder::new();
    b.iter(|| {
        sNode.eval(&mut john,&mut holder).unwrap();
    });
}