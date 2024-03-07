use std::process;

use aes_cmd::file_copy::FileCopy;
use aes_cmd::buf_aes_cipher::BuffAesCipher;
use aes_cmd::dir_file_iter::DirFileIter;
use argh::FromArgs;

/// aes encrypt command
#[derive(FromArgs)]
struct AesParam {
    /// whether decrypt
    #[argh(switch, short = 'd')]
    decrypt: bool,

    /// whether encrypt
    #[argh(switch, short = 'e')]
    encrypt: bool,

    /// input file
    #[argh(option, short = 'i')]
    ifile: Option<String>,

    /// output file
    #[argh(option, short = 'o')]
    ofile: Option<String>,

    /// input direction
    #[argh(option)]
    idir: Option<String>,

    /// output direction
    #[argh(option)]
    odir: Option<String>,

    /// aes key
    #[argh(short = 'k', option)]
    key: Option<String>,

    /// cache size (kb)
    #[argh(option)]
    cache: Option<usize>
}

fn main() {
    let params: AesParam = argh::from_env();

    if !params.decrypt && !params.encrypt {
        println!("encrypt or decrypt ? -e or -d");
        process::exit(1);
    } else if params.decrypt && params.encrypt {
        println!("encrypt or decrypt ? -e or -d, specify one");
        process::exit(1);
    }

    if params.key == None {
        println!("need a key");
        process::exit(1);
    }

    let in_file_name = params.ifile.unwrap_or("".into());
    let in_dir = params.idir.unwrap_or("".into());

    if in_file_name.len() == 0 && in_dir.len() == 0 {
        println!("must specify input file or input direction");
        process::exit(1)
    }

    let out_file_name = params.ofile.unwrap_or("".into());
    let out_dir = params.odir.unwrap_or("".into());

    if in_dir != "" {
        if out_dir.starts_with(&in_dir) {
            println!("input dir {} include\nout dir {}", in_dir, out_dir);
            process::exit(1);
        }
    }

    let mut cipher = BuffAesCipher::new(params.key.unwrap().as_str(), params.cache.unwrap_or(0) * 1024);

    if in_file_name.len() != 0 {
        copy_file(
            in_file_name.as_str(),
            out_file_name.as_str(),
            out_dir.as_str(),
            &mut cipher,
            params.decrypt
        )
    } else if in_dir.len() != 0 {
        let dir = DirFileIter::new(in_dir.as_str()).expect("open input dir failed.");
        for in_file_name in dir {
            copy_file(
                in_file_name.as_str(),
                out_file_name.as_str(),
                out_dir.as_str(),
                &mut cipher,
                params.decrypt
            )
        }
    }
}

fn file_copy_cbk(wrote_bytes: usize, total: u64, name: &str) {
    print!("\r{} : {}({})", name, total, wrote_bytes);
}

fn copy_file(in_file_name: &str, out_file_name: &str, out_dir: &str, cipher: &mut BuffAesCipher, b_decrypter: bool) {
    let mut f_copy = FileCopy::new(in_file_name, out_file_name, out_dir, cipher).expect("create file copy error!");
    let now = std::time::Instant::now();
    if b_decrypter {
        f_copy.decrypt(file_copy_cbk);
    } else {
        f_copy.encrypt(file_copy_cbk);
    }
    println!("\ndecrypt({}), duration: {:?}", b_decrypter, now.elapsed());
}
