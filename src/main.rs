mod markdown;

fn main() {
    let md: Vec<String> = match markdown::read("example.md".to_string()) {
        Ok(data) => data,
        Err(err) => {
            println!("{:?}", err);
            return
        },
    };

    for line in md {
        println!("{line}");
    }
}
