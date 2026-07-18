//! Filesystem.
//!
//! Objetivo de aprendizaje: entender inodos, directorios, permisos,
//! resolución de rutas y validación de árbol.

use std::collections::BTreeMap;

/// Identificador educativo de inodo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InodeId(u64);

impl InodeId {
    /// Crea un identificador de inodo.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Devuelve el valor numérico.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Tipo de archivo representado por un inodo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
}

/// Permisos educativos de lectura, escritura y ejecución.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permissions {
    read: bool,
    write: bool,
    execute: bool,
}

impl Permissions {
    /// Crea permisos explícitos.
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        Self {
            read,
            write,
            execute,
        }
    }

    /// Indica si permite lectura.
    pub fn can_read(&self) -> bool {
        self.read
    }

    /// Indica si permite escritura.
    pub fn can_write(&self) -> bool {
        self.write
    }

    /// Indica si permite ejecución.
    pub fn can_execute(&self) -> bool {
        self.execute
    }
}

/// Entrada de directorio: nombre visible e inodo destino.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    name: String,
    inode: InodeId,
}

impl DirectoryEntry {
    /// Crea una entrada de directorio validada.
    pub fn new(name: impl Into<String>, inode: InodeId) -> Result<Self, FileSystemError> {
        let name = name.into();
        validate_name(&name)?;
        Ok(Self { name, inode })
    }

    /// Nombre de la entrada.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Inodo al que apunta.
    pub fn inode(&self) -> InodeId {
        self.inode
    }
}

/// Inodo educativo con metadata mínima.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inode {
    id: InodeId,
    file_type: FileType,
    permissions: Permissions,
    size_bytes: usize,
    entries: BTreeMap<String, DirectoryEntry>,
}

impl Inode {
    /// Crea un inodo.
    pub fn new(id: InodeId, file_type: FileType, permissions: Permissions) -> Self {
        Self {
            id,
            file_type,
            permissions,
            size_bytes: 0,
            entries: BTreeMap::new(),
        }
    }

    /// Identificador del inodo.
    pub fn id(&self) -> InodeId {
        self.id
    }

    /// Tipo del inodo.
    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    /// Permisos del inodo.
    pub fn permissions(&self) -> Permissions {
        self.permissions
    }

    /// Tamaño educativo en bytes.
    pub fn size_bytes(&self) -> usize {
        self.size_bytes
    }

    /// Busca una entrada hija por nombre.
    pub fn entry(&self, name: &str) -> Option<&DirectoryEntry> {
        self.entries.get(name)
    }

    /// Cantidad de entradas si el inodo es directorio.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    fn insert_entry(&mut self, entry: DirectoryEntry) -> Result<(), FileSystemError> {
        if self.file_type != FileType::Directory {
            return Err(FileSystemError::NotDirectory(self.id));
        }

        if self.entries.contains_key(entry.name()) {
            return Err(FileSystemError::AlreadyExists(entry.name().to_string()));
        }

        self.entries.insert(entry.name.clone(), entry);
        Ok(())
    }
}

/// Modelo educativo de filesystem como árbol de directorios e inodos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileSystemModel {
    root: InodeId,
    next_inode: u64,
    inodes: BTreeMap<InodeId, Inode>,
}

impl FileSystemModel {
    /// Crea un filesystem con directorio raíz.
    pub fn new() -> Self {
        let root = InodeId::new(1);
        let mut inodes = BTreeMap::new();
        inodes.insert(
            root,
            Inode::new(
                root,
                FileType::Directory,
                Permissions::new(true, true, true),
            ),
        );

        Self {
            root,
            next_inode: 2,
            inodes,
        }
    }

    /// Inodo raíz.
    pub fn root_id(&self) -> InodeId {
        self.root
    }

    /// Busca un inodo por identificador.
    pub fn inode(&self, id: InodeId) -> Option<&Inode> {
        self.inodes.get(&id)
    }

    /// Crea un archivo en una ruta absoluta.
    pub fn create_file(
        &mut self,
        path: &str,
        permissions: Permissions,
    ) -> Result<InodeId, FileSystemError> {
        self.create_node(path, FileType::File, permissions)
    }

    /// Crea un directorio en una ruta absoluta.
    pub fn create_directory(
        &mut self,
        path: &str,
        permissions: Permissions,
    ) -> Result<InodeId, FileSystemError> {
        self.create_node(path, FileType::Directory, permissions)
    }

    /// Resuelve una ruta absoluta a un inodo.
    pub fn resolve_path(&self, path: &str) -> Result<InodeId, FileSystemError> {
        if path == "/" {
            return Ok(self.root);
        }

        let parts = split_absolute_path(path)?;
        let mut current = self.root;

        for part in parts {
            let inode = self
                .inodes
                .get(&current)
                .ok_or(FileSystemError::MissingInode(current))?;
            if inode.file_type != FileType::Directory {
                return Err(FileSystemError::NotDirectory(current));
            }

            current = inode
                .entry(part)
                .ok_or_else(|| FileSystemError::NotFound(path.to_string()))?
                .inode();
        }

        Ok(current)
    }

    /// Enlaza un directorio existente bajo otro directorio.
    ///
    /// Esta operación existe para enseñar por qué los filesystems evitan ciclos
    /// de directorios.
    pub fn link_directory(
        &mut self,
        parent_path: &str,
        name: &str,
        target: InodeId,
    ) -> Result<(), FileSystemError> {
        validate_name(name)?;
        let parent = self.resolve_path(parent_path)?;
        let target_inode = self
            .inodes
            .get(&target)
            .ok_or(FileSystemError::MissingInode(target))?;
        if target_inode.file_type != FileType::Directory {
            return Err(FileSystemError::NotDirectory(target));
        }

        if self.would_create_cycle(parent, target)? {
            return Err(FileSystemError::DirectoryCycle { parent, target });
        }

        let entry = DirectoryEntry::new(name, target)?;
        self.insert_entry(parent, entry)
    }

    fn create_node(
        &mut self,
        path: &str,
        file_type: FileType,
        permissions: Permissions,
    ) -> Result<InodeId, FileSystemError> {
        let (parent_path, name) = split_parent_and_name(path)?;
        let parent = self.resolve_path(&parent_path)?;
        let id = self.allocate_inode();
        let entry = DirectoryEntry::new(name, id)?;

        self.insert_entry(parent, entry)?;
        self.inodes
            .insert(id, Inode::new(id, file_type, permissions));
        Ok(id)
    }

    fn allocate_inode(&mut self) -> InodeId {
        let id = InodeId::new(self.next_inode);
        self.next_inode += 1;
        id
    }

    fn insert_entry(
        &mut self,
        parent: InodeId,
        entry: DirectoryEntry,
    ) -> Result<(), FileSystemError> {
        self.inodes
            .get_mut(&parent)
            .ok_or(FileSystemError::MissingInode(parent))?
            .insert_entry(entry)
    }

    fn would_create_cycle(
        &self,
        parent: InodeId,
        target: InodeId,
    ) -> Result<bool, FileSystemError> {
        if parent == target {
            return Ok(true);
        }

        self.contains_descendant(target, parent)
    }

    fn contains_descendant(&self, root: InodeId, needle: InodeId) -> Result<bool, FileSystemError> {
        let inode = self
            .inodes
            .get(&root)
            .ok_or(FileSystemError::MissingInode(root))?;
        if inode.file_type != FileType::Directory {
            return Ok(false);
        }

        for entry in inode.entries.values() {
            if entry.inode() == needle || self.contains_descendant(entry.inode(), needle)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Default for FileSystemModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Error educativo de filesystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileSystemError {
    InvalidPath(String),
    InvalidName(String),
    NotFound(String),
    AlreadyExists(String),
    NotDirectory(InodeId),
    MissingInode(InodeId),
    DirectoryCycle { parent: InodeId, target: InodeId },
}

fn split_parent_and_name(path: &str) -> Result<(String, &str), FileSystemError> {
    let parts = split_absolute_path(path)?;
    let (name, parent_parts) = parts
        .split_last()
        .ok_or_else(|| FileSystemError::InvalidPath(path.to_string()))?;
    let parent_path = if parent_parts.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", parent_parts.join("/"))
    };

    Ok((parent_path, name))
}

fn split_absolute_path(path: &str) -> Result<Vec<&str>, FileSystemError> {
    if !path.starts_with('/') {
        return Err(FileSystemError::InvalidPath(path.to_string()));
    }

    let parts = path
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    for part in &parts {
        validate_name(part)?;
    }

    Ok(parts)
}

fn validate_name(name: &str) -> Result<(), FileSystemError> {
    if name.is_empty() || name == "." || name == ".." || name.contains('/') || name.contains('\0') {
        return Err(FileSystemError::InvalidName(name.to_string()));
    }

    Ok(())
}
