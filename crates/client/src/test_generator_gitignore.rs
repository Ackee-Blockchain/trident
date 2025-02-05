use crate::test_generator::Error;

use crate::constants::*;
use fehler::throws;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::___private::TestGenerator;
use crate::construct_path;

impl TestGenerator {
    #[throws]
    pub(crate) fn update_gitignore(&self, ignored_path: &str) {
        let gitignore_path = construct_path!(self.root, GIT_IGNORE);
        if gitignore_path.exists() {
            let file = std::fs::File::open(&gitignore_path)?;
            for line in std::io::BufReader::new(file).lines().map_while(Result::ok) {
                if line == ignored_path {
                    // INFO do not add the ignored path again if it is already in the .gitignore file
                    println!("{SKIP} [{GIT_IGNORE}], already contains [{ignored_path}]");

                    return;
                }
            }
            // Check if the file ends with a newline
            let mut file = std::fs::File::open(&gitignore_path)?;
            let mut buf = [0; 1];
            file.seek(std::io::SeekFrom::End(-1))?;
            file.read_exact(&mut buf)?;

            let file = OpenOptions::new().append(true).open(gitignore_path);

            if let Ok(mut file) = file {
                if buf[0] == b'\n' {
                    writeln!(file, "{}", ignored_path)?;
                } else {
                    writeln!(file, "\n{}", ignored_path)?;
                }
                println!("{FINISH} [{GIT_IGNORE}] update with [{ignored_path}]");
            }
        } else {
            println!("{SKIP} [{GIT_IGNORE}], not found");
        }
    }
}
