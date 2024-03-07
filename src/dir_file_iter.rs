use std::{fs, os::windows::fs::MetadataExt};

pub struct DirFileIter {
    dirs: Vec<fs::ReadDir>
}

impl DirFileIter {
    pub fn new(in_dir: &str) -> Self{
        let mut dirs = Vec::<fs::ReadDir>::new();

        dirs.push(fs::read_dir(in_dir).unwrap());

        Self {
            dirs
        }
    }
}

impl Iterator for DirFileIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut file_name: Option<String> = None;

        loop {
            let mut new_dirs = Vec::<fs::ReadDir>::new();
            match self.dirs.first_mut() {
                Some(entry_iter) => {
                    loop {
                        match entry_iter.next() {
                            Some(entry_result) => {
                                match entry_result {
                                    Ok(entry) => {
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
                                    },
                                    Err(_) => continue,
                                }
                            },
                            None => break,
                        }
                    }
                },
                None => break,
            }

            let new_dirs_count = new_dirs.len();

            if new_dirs_count == 0 && file_name == None {
                self.dirs.remove(0);

            } else {
                if new_dirs_count != 0 {
                    self.dirs.append(&mut new_dirs);
                }

                if file_name != None {
                    break;
                }
            }

            if self.dirs.len() == 0 {
                break;
            }
        }

        file_name
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dir_file_iter_test() {
        for file in super::DirFileIter::new("c:\\Users\\sdwxi\\Videos") {
            println!("{file}");
        }
    }
}