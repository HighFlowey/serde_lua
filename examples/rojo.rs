use serde_lua::ConfigPair;

const SRC: &str = include_str!(".project.luau");

fn main() {
    let result = ConfigPair::from_str(SRC);

    match result {
        Ok(config) => {
            // config.print_pretty();

            println!("{}", serde_json::to_string_pretty(&config).unwrap());
        }
        Err(err) => println!("{err}"),
    }
}
