use std::fs::File;
use std::io::{ BufRead, BufReader, BufWriter, Write };
use std::error::Error;
use std::path::{Path, PathBuf};
use crate::buf_aes_cipher::BuffAesCipher;
use chrono;

pub struct FileCopy<'a> {
    reader_f: BufReader<File>,
    writer_f: BufWriter<File>,
    cipher: &'a mut BuffAesCipher,
    file_size: u64,
    file_name: String,
    wrote_byties: usize
}

macro_rules! def_enc_dec_fun {
    (
        $name:ident,
        $enc_dec_fn:ident
    ) => {
        pub fn $name(&mut self, cbk: fn(wrote: usize, total: u64, name: &str) -> ()) {
            loop {
                match self.reader_f.fill_buf() {
                    Ok(buf) => {
                        let buf_len = buf.len();
                        if buf_len == 0 {
                            break;
                        }
    
                        let out_buf = self.cipher.$enc_dec_fn(buf);
                        
                        self.writer_f.write(out_buf).expect("write file error");
                        self.reader_f.consume(buf_len);

                        self.wrote_byties += buf_len;
                        cbk(self.wrote_byties, self.file_size, self.file_name.as_str());
                    },
                    Err(e) => {
                        panic!("read file error {:?}", e);
                    },
                }
            }
            self.writer_f.flush().expect("flush writer error");
        }
    };
}

fn get_out_file_name(out_file_name: &str, out_dir: &str, in_file_name: &str) -> String {
    if out_file_name.len() > 0 {
        return out_file_name.to_string();
    }

    if out_dir.len() != 0 {
        if !Path::new(out_dir).exists() {
            panic!("out direction dosn't exist. {}", out_dir);
        }
    }

    let in_path = Path::new(in_file_name);
    let mut dir = out_dir;
    if dir.len() == 0 {
        dir = in_path.parent().unwrap().to_str().unwrap();
    }
    let name_stem = in_path.file_stem().unwrap().to_str().unwrap().to_owned();
    let ext = in_path.extension().unwrap_or_default();

    let mut out_path: PathBuf = [dir, name_stem.as_str()].iter().collect();
    out_path.set_extension(ext);

    if out_path.exists() {
        let t_str = chrono::Local::now().format("%Y%m%d%H%M%S%3f").to_string();
        let name_stem_with_time = format!("{name_stem}-{}", t_str.as_str());

        out_path = [dir, name_stem_with_time.as_str()].iter().collect();
        out_path.set_extension(ext);
    }
    
    out_path.to_str().unwrap().to_string()
}

impl<'a> FileCopy<'a> {
    pub fn new(
        in_file_name: &str, 
        out_file_name: &str, 
        out_dir: &str,
        cipher: &'a mut BuffAesCipher) -> Result<Self, Box<dyn Error>> 
    {
        let input_file = File::open(in_file_name)?;
        let file_size = input_file.metadata()?.len();
        let out_file_name = get_out_file_name(out_file_name, out_dir, in_file_name);

        if Path::new(out_file_name.as_str()).exists() {
            panic!("{} file exists", out_file_name);
        }
        
        let reader_f = BufReader::with_capacity(
            cipher.cache_size(), 
            input_file
        );

        let writer_f = BufWriter::with_capacity(
            cipher.cache_size(), 
            File::create(out_file_name)?
        );

        Ok(Self{
            reader_f,
            writer_f,
            cipher,
            file_size,
            file_name: in_file_name.into(),
            wrote_byties: 0
        })
    }

    def_enc_dec_fun!(encrypt, encrypt);
    def_enc_dec_fun!(decrypt, decrypt);
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_out_file_name_test() {
        let result = crate::file_copy::get_out_file_name(
            "",
            "",
            "c:/test/sww/fff"
        );
        assert_eq!(result, "c:/test/sww\\fff.txt");

        let result = crate::file_copy::get_out_file_name(
            "",
            "d:/abc\\",
            "c:/test/sww/fff.txt"
        );
        assert_eq!(result, "d:/abc\\fff.txt");
    }
}