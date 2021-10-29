use std::process::exit;

mod args;
mod az_metadata;
mod internals;

fn main() {
    let opt = args::parse_args();
    // generate config file if flag is true.
    if opt.init {
        match args::Opt::generate_config_file(&opt) {
            Ok(()) => {
                println!("{} file written in the current directory.", "config.toml");
                println!("Edit it and run this app again");
                exit(0);
            }
            Err(err) => {
                println!("An error occurred: {:?}", err);
                exit(1);
            }
        }
    }
    println!("Parse Azure Indexer definition");
    println!("Will fecth indexer defined in {:?}", opt.config);
    println!("Either customixe that file or pass a different one from the command line");
    println!("For more example try --help");

    let azure = az_metadata::read_toml_file(opt.config).expect("Error reading toml");
    dbg!(azure);

    // let file = std::fs::File::open("source.json").expect("Error opening the file");
    // let reader = std::io::BufReader::new(file);
    // let def = internals::parse_json(reader).expect("Error parsing json");

    // let mut buffer = std::fs::File::create("out.ts").expect("Error creating the output file");

    // buffer = def.write_to_file(buffer);

    // let metadata = buffer.metadata().expect("Error reading metadata");
    // println!("out.ts - {} bytes written", metadata.len());

    exit(0)
}
