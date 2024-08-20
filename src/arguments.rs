use clap::Parser;

#[derive(Parser)]
#[command(about, version)]
pub struct Arguments {
    /// PID of the process
    #[arg(long, short, value_parser = validate_pid)]
    pub pid: u32,

    /// Output file name
    #[arg(long, short)]
    pub output: Option<String>
}

fn validate_pid(s: &str) -> Result<u32, String> {
    s
        .parse::<u32>()
        .map_err(|_| {
            String::from("Not a valid unisigned 32 bit integer!")
        })
}
