use std::{fmt::Display, vec};

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
    depth: usize,
}
#[derive(Debug)]
pub struct Folder {
    name: String,
    contents: Vec<Entity>,
    depth: usize,
}
impl Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_contents = String::new();
        for content in self.contents.iter() {
            match content {
                Entity::File(file) => formatted_contents.push_str(&format!(
                    "{:indent$}- {} (file, size = {})\n",
                    "",
                    file.name,
                    file.size,
                    indent = file.depth * 2
                )),
                Entity::Folder(folder) => formatted_contents.push_str(&format!(
                    "{:indent$}{}",
                    "",
                    folder,
                    indent = folder.depth * 2,
                )),
            }
        }
        write!(f, "- {} (dir)\n{}", self.name, formatted_contents)
    }
}
#[derive(Debug)]
enum Entity {
    File(File),
    Folder(Folder),
}

const CORRUPTED_FILE_MESSAGE: &str = "Corrupted file";

#[derive(Debug)]
struct ChildFolderDoesNotExist {
    folder_name: String,
}
impl Display for ChildFolderDoesNotExist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Child folder \"{}\" does not exists!", self.folder_name)
    }
}

fn find_folder_by_location_mut<'a>(
    location_slice: &'a [&str],
    tree: &'a mut Folder,
) -> Result<&'a mut Folder, ChildFolderDoesNotExist> {
    // base case
    if location_slice.len() == 1 {
        return Ok(tree);
    }

    let new_location_slice = &location_slice[1..];

    let next_folder_name = new_location_slice[0];

    for content in tree.contents.iter_mut() {
        if let Entity::Folder(folder) = content {
            if folder.name == next_folder_name {
                return find_folder_by_location_mut(new_location_slice, folder);
            }
        }
    }

    return Err(ChildFolderDoesNotExist {
        folder_name: next_folder_name.to_owned(),
    });
}

pub fn get_file_system(log: String) -> Result<Folder, &'static str> {
    let mut result = Folder {
        name: "/".to_owned(),
        contents: vec![],
        depth: 0,
    };

    let mut location = vec!["/"];

    let mut is_listing = false;
    for line in log.lines() {
        let statements: Vec<&str> = line.split(" ").collect();
        match *statements.get(0).expect(CORRUPTED_FILE_MESSAGE) {
            "$" => {
                // input
                match *statements.get(1).expect(CORRUPTED_FILE_MESSAGE) {
                    "ls" => {
                        is_listing = true;
                    }
                    "cd" => {
                        is_listing = false;
                        let folder_name = statements.get(2).expect(CORRUPTED_FILE_MESSAGE);

                        match *folder_name {
                            ".." => {
                                location.pop();
                            }
                            "/" => {
                                location = vec!["/"];
                            }
                            other => {
                                location.push(other);
                            }
                        }
                    }
                    _ => {
                        is_listing = false;
                    }
                }
            }
            statement => {
                // output
                if is_listing {
                    let name_of_new_entity = statements.get(1).expect(CORRUPTED_FILE_MESSAGE);

                    let current_folder = match find_folder_by_location_mut(&location, &mut result) {
                        Ok(value) => value,
                        Err(error) => {
                            dbg!(&result);
                            panic!("{error}");
                        }
                    };
                    if statement == "dir" {
                        current_folder.contents.push(Entity::Folder(Folder {
                            contents: vec![],
                            name: name_of_new_entity.to_string(),
                            depth: current_folder.depth + 1,
                        }))
                    } else {
                        current_folder.contents.push(Entity::File(File {
                            size: statement.parse().unwrap(),
                            name: name_of_new_entity.to_string(),
                            depth: current_folder.depth + 1,
                        }))
                    }
                }
            }
        }
    }

    return Ok(result);
}

pub fn first_part(root_folder: Folder) -> u32 {
    let folder_sizes = get_folder_sizes(&root_folder);
    dbg!(&folder_sizes);

    return folder_sizes.iter().sum();
}

fn get_folder_sizes(folder: &Folder) -> Vec<u32> {
    let mut current_folders_sizes: Vec<u32> = vec![];
    let mut total_size_of_files_this_folder: u32 = 0;

    for entity in folder.contents.iter() {
        match entity {
            Entity::File(file) => total_size_of_files_this_folder += file.size,
            Entity::Folder(folder) => {
                println!("Folder: {}", folder.name);
                let mut child_folder_sizes = get_folder_sizes(folder);
                println!("Sizes: {:?}", child_folder_sizes);
                current_folders_sizes.append(&mut child_folder_sizes);
                println!(
                    "Sizes of all {} child folders: {:?}",
                    folder.name, current_folders_sizes
                );
            }
        }
    }

    let total_size_child_folders: u32 = current_folders_sizes.iter().sum();
    let total_size_this_folder: u32 = total_size_child_folders + total_size_of_files_this_folder;

    if total_size_this_folder < 100000 {
        current_folders_sizes.push(total_size_this_folder);
    }

    return current_folders_sizes;
}
