use std::{fs, io};
use std::io::Read;
use raylib::prelude::Font;
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
    selected: Option<i32>,
    scroll_y: f32,
    scroll_x: f32,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

impl WorkSpace
{
    pub fn new() -> Self
    {
        WorkSpace {
            lines: Vec::new(),
            selected: None,
            scroll_y: 0.0,
            scroll_x: 0.0,
            width: 0,
            height: 0,
            x: 0,
            y: 0
        }
    }
}

pub struct Editor
{
    pub title: String,
    pub work_space: Option<WorkSpace>,
    pub files: Vec<File>,
    pub font: Option<Font>,
    pub width: i32,
    pub height: i32,
}

pub struct EventsCollector
{
    pub Events: Option<Event>
}

pub struct Event
{
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

impl Event
{
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self
    {
        let bounds = Event {
            x,
            y,
            width,
            height
        };

        bounds
    }
}
impl Editor
{
    pub fn new(width: i32, height: i32) -> Self
    {
        Editor {
            title: String::from("Russian Code Editor"),
            work_space: None,
            files: Vec::new(),
            font: None,
            width, height
        }
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

    pub fn mouse_click(&mut self, x: i32, y: i32)
    {
        // check if is mouse boundle click
        if ( x > 0 && x < self.width ) && ( y > 0 && y < self.height )
        {
          // event

           Event.new();
        }



        println!("x: {}, y: {}", x, y);
    }
}

