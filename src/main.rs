use mdedit;

fn main() {
    let md: Vec<String> = match mdedit::read("example.md".to_string()) {
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
