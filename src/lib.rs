pub mod file_controller {
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::path::Path;
    use std::option::Option;

    pub fn overwrite_to_file(path_str: &str, content: &str, silent: bool) -> () {
        // prompt user before rewriting, if file found
        let path = Path::new(path_str);

        // I like organizing dot notation like this, especially when it gets lengthy
        // TODO: Figure out how to add some extra formatting configurations to the prettify formatter, for laziness :p
        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path) {
                Ok(file) => file,
                Err(err) => panic!("{} {}: {}\n", "Couldn't open file at", path.display(), err),
            };

        match file.write(&content.as_bytes()) {
            Ok(size) => if silent {
                println!(
                "\nSuccessfully wrote to {}.\nSize of: {}\n",
                path.display(),
                size
            )}
            else {()},
            Err(err) => panic!(
                "Couldn't write string to file, at {}: {}\n",
                path.display(),
                err
            ),
        }
    }

    pub fn write_to_file(path_str: &str, content: &str, silent: bool) -> () {
        let path = Path::new(path_str);
        let mut file = match OpenOptions::new().write(true).append(true).open(&path) {
            Ok(file) => file,
            Err(err) => panic!("Couldn't open file at {}: {}\n", path.display(), err),
        };
        match file.write(&content.as_bytes()) {
            Ok(size) => if !silent { 
                println!(
                "\nSuccessfully wrote to {}.\nSize of: {}\n",
                path.display(),
                size
            )}
            else {()},
            Err(err) => panic!(
                "Couldn't write string to file, at {}: {}\n",
                path.display(),
                err
            ),
        }
    }

    pub fn read_from_file(_path: &str) -> () {
        let path = Path::new(_path);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => panic!("Couldn't open file at {}: {}\n", path.display(), err),
        };
        let mut result = String::new();
        match file.read_to_string(&mut result) {
            Ok(size) => print!(
                "{} contains:\n{}\n📬 Size of: {}\n",
                path.display(),
                result,
                size
            ),
            Err(err) => panic!(
                "Couldn't read from file to string at {}: {}\n",
                path.display(),
                err
            ),
        }
    }
}
