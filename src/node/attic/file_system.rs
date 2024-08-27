use std::cell::RefCell;
use std::fmt;
use std::num::NonZeroUsize;

use std::rc::Rc;

use h10::http::result::{H10LibError, H10LibResult};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FileSystem {
    pub(crate) root: Rc<RefCell<FsNode>>,
}

impl FileSystem {
    pub(crate) fn new() -> Self {
        FileSystem {
            root: Rc::new(RefCell::new(FsNode::Directory(Directory::new("/")))),
        }
    }
    pub(crate) fn print_structure(&self) {
        self.print_node(&self.root, 0);
    }

    fn print_node(&self, node: &Rc<RefCell<FsNode>>, depth: usize) {
        let indent = "  ".repeat(depth);
        match &*node.borrow() {
            FsNode::File(file) => println!("{}`-- {}", indent, file.metadata.name.0),
            FsNode::Directory(dir) => {
                println!("{}|-+- {}", indent, dir.metadata.name.0);
                for child in &dir.contents {
                    self.print_node(child, depth + 1);
                }
            }
        }
    }
}

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "File System Structure:")?;
        self.print_node(&self.root, 0);
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum FsNode {
    File(File),
    Directory(Directory),
}

impl FsNode {
    pub(crate) fn add_directory(&mut self, dir: Directory) -> H10LibResult<()> {
        if let FsNode::Directory(directory) = self {
            directory.add_directory(dir);
            Ok(())
        } else {
            Err(H10LibError::Custom(
                "Trying to add a directory to a file node".into(),
            ))
        }
    }

    pub(crate) fn add_file(&mut self, file: File) -> H10LibResult<()> {
        if let FsNode::Directory(directory) = self {
            directory.add_file(file);
            Ok(())
        } else {
            Err(H10LibError::Custom(
                "Trying to add a file to a file node".into(),
            ))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct FsNodeMetadata {
    name: FsNodeName,
    created_at: UnixTimeStamp,
    updated_at: UnixTimeStamp,
    permission: UnixPermission,
    owner: UnixOwnership,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct File {
    metadata: FsNodeMetadata,
    content: FileContent,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct FileContent {
    size: usize,
    inner: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Directory {
    metadata: FsNodeMetadata,
    contents: Vec<Rc<RefCell<FsNode>>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct FsNodeName(Rc<str>);

impl FsNodeName {
    pub(crate) fn from_str(s: &str) -> H10LibResult<Self> {
        Ok(Self(s.into()))
    }
}

impl Directory {
    pub(crate) fn new(name: &str) -> Self {
        Directory {
            metadata: FsNodeMetadata {
                name: FsNodeName::from_str(name).unwrap(),
                created_at: UnixTimeStamp::now().unwrap(),
                updated_at: UnixTimeStamp::now().unwrap(),
                permission: UnixPermission::new(7, 7, 7),
                owner: UnixOwnership::new(1, 1).unwrap(),
            },
            contents: Vec::new(),
        }
    }

    pub(crate) fn add_file(&mut self, file: File) {
        self.contents
            .push(Rc::new(RefCell::new(FsNode::File(file))));
    }

    pub(crate) fn add_directory(&mut self, dir: Directory) {
        self.contents
            .push(Rc::new(RefCell::new(FsNode::Directory(dir))));
    }
}

impl File {
    pub(crate) fn new(name: &str, content: &str) -> Self {
        File {
            metadata: FsNodeMetadata {
                name: FsNodeName::from_str(name).unwrap(),
                created_at: UnixTimeStamp::now().unwrap(),
                updated_at: UnixTimeStamp::now().unwrap(),
                permission: UnixPermission::new(6, 4, 4),
                owner: UnixOwnership::new(1, 1).unwrap(),
            },
            content: FileContent {
                size: content.len(),
                inner: content.as_bytes().to_vec(),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnixTimeStamp(i64);

impl UnixTimeStamp {
    pub(crate) fn now() -> H10LibResult<Self> {
        use std::time::SystemTime;
        let now = SystemTime::now();
        let now_unix_epoch = now.duration_since(SystemTime::UNIX_EPOCH)?;

        Ok(Self(now_unix_epoch.as_secs() as i64))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnixPermission(u16);

impl UnixPermission {
    pub(crate) fn new(owner: u8, group: u8, others: u8) -> Self {
        let value = ((owner as u16) << 6) | ((group as u16) << 3) | (others as u16);
        Self(value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnixOwnership {
    uid: UnixUid,
    gid: UnixGid,
}

impl UnixOwnership {
    pub(crate) fn new(uid: usize, gid: usize) -> H10LibResult<Self> {
        Ok(Self {
            uid: UnixUid::new(uid)?,
            gid: UnixGid::new(gid)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnixUid(NonZeroUsize);
impl UnixUid {
    pub(crate) fn new(id: usize) -> H10LibResult<Self> {
        todo!();
        Ok(Self(NonZeroUsize::try_from(id).unwrap()))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnixGid(NonZeroUsize);
impl UnixGid {
    pub(crate) fn new(id: usize) -> H10LibResult<Self> {
        todo!();
        Ok(Self(NonZeroUsize::try_from(id).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_system_structure() {
        let fs = FileSystem::new();

        {
            let mut root = fs.root.borrow_mut();

            let mut documents = Directory::new("documents");
            documents.add_file(File::new("resume.txt", "My resume content"));
            documents.add_file(File::new("notes.txt", "Some important notes"));

            let mut pictures = Directory::new("pictures");
            pictures.add_file(File::new("vacation.jpg", "Binary content..."));

            root.add_directory(documents);
            root.add_directory(pictures);
            root.add_file(File::new("hello.txt", "Hello, world!"));
        } // root is dropped here, releasing the mutable borrow

        // This test just ensures that the structure can be created without panicking
        // You might want to add more specific assertions based on your requirements
        fs.print_structure();
    }
}
