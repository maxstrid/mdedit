mod markdown;

fn main() {
    let md: Vec<String> = match markdown::read("example.md".to_string()) {
        Ok(data) => data,
        Err(err) => {
            println!("{err:?}");
            return;
        }
    };

    for line in md {
        println!("{line}");
    }

    match markdown::write("test.md".to_string(), "#Hello".to_string()) {
        Err(err) => println!("{err:?}"),
        _ => (),
    }
}
