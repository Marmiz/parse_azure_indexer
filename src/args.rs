use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    /// If true generate a base config in the current directory
    #[structopt(short, long)]
    pub init: bool,

    /// Configuration file. Will use config.toml in the current directory if not present
    #[structopt(parse(from_os_str), default_value = "config.toml")]
    pub config: PathBuf,
}

impl Opt {
    pub fn generate_config_file(&self) -> std::io::Result<()> {
        let content = br#"service_name = "<azure service name>"
index_name = "<index name>"
api_version = "2020-06-30""#;
        std::fs::write("config.toml", content)?;

        Ok(())
    }
}

pub fn parse_args() -> Opt {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    opt
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_default_opts() {
        let opt = parse_args();
        let expected = Opt {
            init: false,
            config: PathBuf::from("config.toml"),
        };
        assert_eq!(opt, expected);
    }
}
