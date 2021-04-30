use rust_server_model::parser;

fn main() {
    parse_smithy().unwrap()
}

fn parse_smithy() -> anyhow::Result<()>{
    println!("Current Dir: {}", std::env::current_dir()?.to_str().unwrap());
    let mut models_dir = std::env::current_dir()?;
    models_dir.push("model");

    let contents = parser::read_models(&models_dir)?;
    println!("Files found: {}", contents.len());
    for model in contents {
        println!("Some contents: \n{}", model);
    }
    Ok(())
}