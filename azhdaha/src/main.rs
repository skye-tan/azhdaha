use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::read();
    println!("{:#?}", cli);
    Ok(())
}
