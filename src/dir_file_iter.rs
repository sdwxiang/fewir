use std::{error::Error, fs, os::windows::fs::MetadataExt};

pub struct DirFileIter {
    dirs: Vec<fs::ReadDir>,
}

impl DirFileIter {
    pub fn new(in_dir: &str) -> Result<Self, Box<dyn Error>> {
        let mut dirs = Vec::<fs::ReadDir>::new();

        dirs.push(fs::read_dir(in_dir)?);

        Ok(Self {
            dirs
        })
    }
}

impl Iterator for DirFileIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut file_name: Option<String> = None;

        loop {
            let mut new_dirs = Vec::<fs::ReadDir>::new();

            if let Some(entry_iter) = self.dirs.first_mut() {
                while let Some(entry_result) = entry_iter.next() {
                    if let Ok(entry) = entry_result {
                        let path = entry.path();
                        // // FILE_ATTRIBUTE_HIDDEN 2 0x00000002
                        if path.metadata().unwrap().file_attributes() & 2 != 0 {
                            continue;
                        }
                        if path.is_dir() {
                            new_dirs.push(fs::read_dir(path).unwrap());
                        } else if path.is_file() {
                            file_name = Some(path.to_str().unwrap().to_owned());
                            break;
                        }
 
                    } else {
                        continue;
                    }
                }

            } else {
                break; // self.dirs empty. finished
            }

            let new_dirs_count = new_dirs.len();

            if new_dirs_count == 0 && file_name == None {
                // self.dirs.first has no dirs and no files
                self.dirs.remove(0); // self.dirs.first fished then remove

            } else {
                if new_dirs_count != 0 {
                    // self.dirs.first has dirs, then append to self.dirs
                    self.dirs.append(&mut new_dirs);
                }

                if file_name != None {
                    // find file, return the name
                    break;
                }
            }

            if self.dirs.len() == 0 {
                break; // self.dirs empty. finished
            }
        }

        file_name
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dir_file_iter_test() {
        let dir = super::DirFileIter::new("c:\\Users\\sdwxi\\Videos")
            .expect("create dir filter iter error");
        for file in dir {
            println!("{file}");
        }
    }
}