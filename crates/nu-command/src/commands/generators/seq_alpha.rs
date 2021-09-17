use crate::prelude::*;
use nu_engine::WholeStreamCommand;
use nu_errors::ShellError;
use nu_protocol::{Signature, SyntaxShape, UntaggedValue, value::StrExt, Value};
use nu_source::Tagged;
use std::cmp;

pub struct SeqAlpha;

impl WholeStreamCommand for SeqAlpha {
    fn name(&self) -> &str {
        "seq alpha"
    }

    fn signature(&self) -> Signature {
        Signature::build("seq alpha")
            .rest("rest", SyntaxShape::String, "sequence values")
    }

    fn usage(&self) -> &str {
        "Print sequences of strings in lexicographic order"
    }

    fn run(&self, args: CommandArgs) -> Result<OutputStream, ShellError> {
        seq_alpha(args)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "sequence a to e",
                example: "seq a e",
                result: Some(vec![
                             UntaggedValue::string("a").into(),
                             UntaggedValue::string("b").into(),
                             UntaggedValue::string("c").into(),
                             UntaggedValue::string("d").into(),
                             UntaggedValue::string("e").into(),
                ])
            },
        ]
    }   
}

fn seq_alpha(args: CommandArgs) -> Result<OutputStream, ShellError> {

    let name = args.call_info.name_tag.clone();

    let rest_strs: Vec<Tagged<String>> = args.rest(0)?;

    if rest_strs.is_empty() {
        return Err(ShellError::labeled_error(
            "seq requires some parameters",
            "needs parameter",
            name,
        ));
    }

    let sep = "\n";

    let first = rest_strs[0].chars().nth(0).unwrap();
    let last = rest_strs[rest_strs.len() - 1].chars().nth(0).unwrap();

    let mut ret_str = String::new();

    let mut is_first_elem = true;

    let itr = (first..=last).into_iter();

    for elem in itr {
        if !is_first_elem {
            ret_str.push_str(sep);
        }
        is_first_elem = false;
        ret_str.push(elem);
    }

    let rows: Vec<Value> = ret_str
        .lines()
        .map(|v| v.to_str_value_create_tag())
        .collect();

    Ok(rows.into_iter().into_output_stream())
}
    

    

    
