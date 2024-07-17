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
}

pub (crate) mod drawer
{
    use raylib::color::Color;
    use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

    pub fn draw_main(mut d: RaylibDrawHandle)
    {
        d.clear_background(Color::new(0, 0,0,0));

        let width = d.get_screen_width() as f32;
        let height = d.get_screen_height() as f32;

        draw_rounded_rectangle(&mut d, 3, 3, (width - 6.0) as i32,
                               (height - 6.0) as i32, 10, Color::new(43, 45, 48, 255));

        draw_great_flag(d);

    }

    fn draw_great_flag(mut d: RaylibDrawHandle)
    {
        draw_horizontal_gradient(&mut d, 70, 3, 110, 40,
                                 Color::new(60, 63, 65, 255),
                                 Color::new(200,200,200, 25));

        draw_horizontal_gradient(&mut d, 180, 3, 100, 40,
                                 Color::new(200,200,200, 25),
                                 Color::new(65,105,225, 25));

        draw_horizontal_gradient(&mut d, 280, 3, 150, 40,
                                 Color::new(65,105,225, 25),
                                 Color::new(178,34,34, 25));
        draw_horizontal_gradient(&mut d, 430, 3, 170, 40,
                                 Color::new(178,34,34, 25),
                                 Color::new(60, 63, 65, 255));
    }

    fn draw_rounded_rectangle(d: &mut RaylibDrawHandle, x: i32, y: i32, width: i32, height: i32, radius: i32, color: Color)
    {
        let color_head = Color::new(60, 63, 65, 255);

        d.draw_rectangle(x + radius, y, width - 2 * radius, height, color);
        d.draw_rectangle(x, y + radius, width, height - 2 * radius, color);

        d.draw_circle(x + radius, y + radius, radius as f32, color_head);
        d.draw_circle(x + width - radius, y + radius, radius as f32, color_head);
        d.draw_circle(x + radius, y + height - radius, radius as f32, color);
        d.draw_circle(x + width - radius, y + height - radius, radius as f32, color);

        d.draw_rectangle(x + radius, y, width - 2 * radius, radius, color_head);

        d.draw_rectangle(x + radius, y + height - radius, width - 2 * radius, radius, color);
        d.draw_rectangle(x, y + radius, radius, height - 2 * radius, color);
        d.draw_rectangle(x + width - radius, y + radius, radius, height - 2 * radius, color);

        d.draw_rectangle(x, y + radius, width, 30, color_head);
    }

    fn draw_vertical_gradient(
        d: &mut RaylibDrawHandle,
        start_x: i32,
        start_y: i32,
        width: i32,
        height: i32,
        top_color: Color,
        bottom_color: Color,
    ) {
        for i in 0..height {
            let ratio = i as f32 / height as f32;
            let color = Color {
                r: (top_color.r as f32 * (1.0 - ratio) + bottom_color.r as f32 * ratio) as u8,
                g: (top_color.g as f32 * (1.0 - ratio) + bottom_color.g as f32 * ratio) as u8,
                b: (top_color.b as f32 * (1.0 - ratio) + bottom_color.b as f32 * ratio) as u8,
                a: 255,
            };
            d.draw_rectangle(start_x, start_y + i, width, 1, color);
        }
    }

    fn draw_horizontal_gradient(
        d: &mut RaylibDrawHandle,
        start_x: i32,
        start_y: i32,
        width: i32,
        height: i32,
        left_color: Color,
        right_color: Color
    ) {
        for i in 0..width {
            let ratio = i as f32 / width as f32;
            let color = Color {
                r: (left_color.r as f32 * (1.0 - ratio) + right_color.r as f32 * ratio) as u8,
                g: (left_color.g as f32 * (1.0 - ratio) + right_color.g as f32 * ratio) as u8,
                b: (left_color.b as f32 * (1.0 - ratio) + right_color.b as f32 * ratio) as u8,
                a: 255,
            };
            d.draw_rectangle(start_x + i, start_y, 1, height, color);
        }
    }
}