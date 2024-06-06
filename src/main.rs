fn find<'a>(paths: &'a [&str], extension: &'a str) -> impl Iterator<Item = String> + 'a {
    generator::Gn::new_scoped(move |mut scope| {
        for directory in paths {
            if let Ok(dir) = std::fs::read_dir(directory) {
                for entry in dir.filter_map(Result::ok) {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(extension) {
                            scope.yield_([directory, "/", file_name].concat());
                        }
                    }
                }
            }
        }
        generator::done!()
    })
}

fn main() {
    let paths = ["./src"];
    let extension = ".rs";
    let f: Vec<_> = find(&paths, &extension).collect();
    println!("{:?}", f);
}
