use crate::*;
use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    rc::{Rc, Weak},
    slice::Iter,
};

enum DirEntry {
    Dir(Rc<Directory>),
    File((usize, String)),
}

struct Directory {
    ptr: Weak<Directory>,
    parent: Weak<Directory>,
    name: String,
    contents: Vec<DirEntry>,
}

impl Directory {
    fn new<S: Into<String>>(name: S) -> Rc<Self> {
        Rc::new_cyclic(|ptr| Directory {
            ptr: ptr.clone(),
            parent: Weak::new(),
            name: name.into(),
            contents: Vec::new(),
        })
    }

    fn size(&self) -> usize {
        self.contents
            .iter()
            .map(|entry| match entry {
                DirEntry::Dir(dir) => dir.size(),
                DirEntry::File((len, _)) => *len,
            })
            .sum()
    }

    fn iter_dirs(&self) -> DirIterator {
        DirIterator {
            iter: self.contents.iter(),
        }
    }

    fn push_file<S: Into<String>>(&mut self, name: S, size: usize) {
        self.contents.push(DirEntry::File((size, name.into())));
    }

    fn push_dir<S: Into<String>>(&mut self, name: S) {
        self.contents.push(DirEntry::Dir(Rc::new_cyclic(|ptr| Directory {
            ptr: ptr.clone(),
            parent: self.ptr.clone(),
            name: name.into(),
            contents: Vec::new(),
        })));
    }

    fn get_dir<S: Into<String>>(&self, name: S) -> Option<Rc<Self>> {
        let dir_name = name.into();
        for entry in self.contents.iter() {
            if let DirEntry::Dir(dir) = entry {
                if dir.name == dir_name {
                    return Some(dir.clone());
                }
            }
        }
        None
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "- {} (dir)", self.name)?;
        for (i, entry) in self.contents.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            match entry {
                DirEntry::File((size, name)) => write!(f, "  - {name} (file, size={size})")?,
                DirEntry::Dir(dir) => write!(f, "  {}", dir.to_string().replace('\n', "\n  "))?,
            }
        }
        Ok(())
    }
}

struct DirIterator<'a> {
    iter: Iter<'a, DirEntry>,
}

impl<'a> Iterator for DirIterator<'a> {
    type Item = Rc<Directory>;

    fn next(&mut self) -> Option<Self::Item> {
        for entry in self.iter.by_ref() {
            if let DirEntry::Dir(dir) = entry {
                return Some(dir.clone());
            }
        }
        None
    }
}

fn small_dir_sum(base: Rc<Directory>, treshold: usize) -> usize {
    let mut sum = 0;
    for dir in base.iter_dirs() {
        sum += small_dir_sum(dir.clone(), treshold);
        let size = dir.size();
        if size <= treshold {
            sum += size;
        }
    }
    sum
}

fn min_dir_size_to_del(base: Rc<Directory>, treshold: usize) -> usize {
    let mut min_size = usize::MAX;
    for dir in base.iter_dirs() {
        min_size = min_size.min(min_dir_size_to_del(dir.clone(), treshold));
        let size = dir.size();
        if size >= treshold {
            min_size = min_size.min(size);
        }
    }
    min_size
}

pub fn solve(input: File) -> Result<Solution, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let root = Directory::new("/");
    let mut current_dir = root.clone();
    for (i, line) in reader.lines().enumerate() {
        let term = line?;
        let parts = term.split(' ').collect::<Vec<_>>();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => match parts[2] {
                    "/" => current_dir = root.clone(),
                    ".." => {
                        current_dir = current_dir
                            .parent
                            .upgrade()
                            .ok_or_else(|| format!("directory {} does not have a parent", current_dir.name))?
                    }
                    dir_name => {
                        current_dir = current_dir.get_dir(dir_name).ok_or_else(|| {
                            format!(
                                "directory {} does not have a directory called {}",
                                current_dir.name, dir_name
                            )
                        })?
                    }
                },
                "ls" => {}
                _ => return Err(format!("malformed input at line {}", i + 1).into()),
            },
            "dir" => unsafe {
                Rc::get_mut_unchecked(&mut current_dir).push_dir(parts[1]);
            },
            size => unsafe {
                Rc::get_mut_unchecked(&mut current_dir).push_file(parts[1], size.parse::<usize>()?);
            },
        }
    }
    let free_space = 70000000 - root.size();
    let space_needed = 30000000 - free_space;
    Ok((
        Box::new(small_dir_sum(root.clone(), 100000)),
        Box::new(min_dir_size_to_del(root, space_needed)),
    ))
}
