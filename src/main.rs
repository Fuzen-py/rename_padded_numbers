/*
MIT License

Copyright (c) 2021 Fuzen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
};

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
}

fn files_only<E>(entry: Result<DirEntry, E>) -> Option<PathBuf> {
    let entry = entry.ok()?;
    if is_file(&entry) {
        Some(entry.path())
    } else {
        None
    }
}

fn main() {
    let cwd = env::current_dir().expect("Failed to get cwd");
    println!("Finding files in {:?}", cwd);
    let paths = fs::read_dir(cwd)
        .map(|dir| dir.filter_map(files_only).collect::<Vec<PathBuf>>())
        .expect("Failed to read dir");
    print!("Finding highest digit number");
    let mut digits = 0;
    // First Pass to find digit length
    for path in &paths {
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            let num_length = name
                .chars()
                .skip_while(|c| !c.is_numeric())
                .take_while(|c| c.is_numeric())
                .count();
            if num_length > digits {
                digits = num_length;
            }
        }
    }
    println!("\r Found number with {} digits, padding..", digits);
    let mut was_change = false;
    let mut longest_msg = 0;
    // Second pass, where files are renamed
    for path in paths {
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            let num: String = name
                .chars()
                .skip_while(|c| !c.is_numeric())
                .take_while(|c| c.is_numeric())
                .collect();
            let padded_num = format!("{:0>1$}", num, digits);
            let new_name = name.replace(&num, &padded_num);
            // NOOP
            if name == new_name {
                continue;
            }
            let new_path = path.with_file_name(new_name);
            let msg = format!("{:?} -> {:?}\r", path, new_path);
            {
                let len = msg.len();
                if longest_msg < len {
                    longest_msg = len;
                }
            }
            print!("{}\r", msg);
            fs::rename(path, new_path).expect("Failed to rename file");
            was_change = true;
        }
    }
    if was_change {
        println!("{: >1$}", "Complete!", longest_msg)
    } else {
        println!("No changes were made");
    }
}
