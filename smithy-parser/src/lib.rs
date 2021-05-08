use abnf::rulelist;
use std::fs;

mod matchers;
mod functors;

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

// const SMITHY_IDL_PATH: &str = "configuration/smithy-idl.txt";
//
// fn parse_smithy(models: Vec<String>) -> anyhow::Result<()> {
//     let mut smithy_idl = std::env::current_dir()?;
//     smithy_idl.push(SMITHY_IDL_PATH);
//
//     let smithy_idl = fs::read_to_string(smithy_idl)?;
//     println!("IDL: {}", smithy_idl);
//     let rules = match rulelist(smithy_idl.trim()) {
//         Ok(rules) => rules,
//         Err(err) => { eprintln!("{:?}", err); return Err(anyhow::Error::new(err))}
//     };
//     println!("{:#?}", rules);
//     Ok(())
// }
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> anyhow::Result<()>{
//         crate::parse_smithy(vec![])
//
//     }
// }
