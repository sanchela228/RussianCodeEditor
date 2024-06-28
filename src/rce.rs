use std::{fs, io};
use std::io::Read;
// use raylib::prelude::Font;
use raylib::prelude::*;
use rfd::FileDialog;


pub struct File
{
    content: String,
    name: String,
    path: String,
    have_changes: bool
}

impl File {
    pub fn save(&mut self)
    {
        fs::write(&self.path, &self.content).expect("Unable to write file");
        self.have_changes = false;
    }
    pub fn new(name: String, path: String, content: String) -> Self
    {
        File {
            content,
            name,
            path,
            have_changes: false
        }
    }
}

pub struct Line{
    number: i32,
    content: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    scroll_x: f32,
    scroll_y: f32
}
pub struct WorkSpace
{
    lines: Vec<Line>,
    selected: i32,
    scroll_y: f32,
    scroll_x: f32,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}
pub struct Editor
{
    work_space: WorkSpace,
    files: Vec<File>,
    font: Font,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Editor
{
    pub fn new(width: i32, height: i32, x: i32, y: i32) -> Self
    {
        Editor {
            work_space: WorkSpace::new(),
            files: Vec::new(),
            font: Font::load_font("Harm.ttf").unwrap(),
            x, y, width, height
        }
    }
    pub fn printtest(&mut self)
    {

        println!("test");
    }
    pub fn read_file_content(&mut self, file_path: &str) -> io::Result<String>
    {
        let mut file = fs::File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(content)
    }
    pub fn open_file_dialog(&mut self)
    {
        if let Some(path) = FileDialog::new().pick_file()
        {
            self.add_file(path.to_string_lossy().to_string());
        }
    }
    pub fn add_file(&mut self, path: String)
    {
        let content = self.read_file_content(&path.to_string()).unwrap();

        let file = File::new(
            path.to_string(),
            path.to_string(),
            content
        );

        self.files.push(file);
    }
}

