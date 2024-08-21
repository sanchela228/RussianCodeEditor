// mod editor;
// mod file;
// mod work_space;
// mod line;

// mod drawer;
// mod rmxl_rider;
//
// use std::ops::Deref;
// use raylib::ffi::ConfigFlags;
// use raylib::prelude::*;
// use rfd::FileDialog;
// use std::fs;
// use std::io::{self, Read};

// fn read_file_content(file_path: &str) -> io::Result<String> {
//     let mut file = fs::File::open(file_path)?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;
//     Ok(content)
// }

use raylib::color::Color;
use raylib::drawing::RaylibDraw;

#[derive(Debug, Clone)]
enum ViewType {
    Percent,
    Pixel,
    ViewportHeight,
    ViewportWidth
}

#[derive(Debug, Clone)]
struct Block {
    name: Option<String>,
    id: Option<String>,
    width: Option<u32>,
    width_type: Option<ViewType>,
    height: Option<u32>,
    height_type: Option<ViewType>,
    parent: Option<*mut Block>,
    childrens: Option<Vec<*mut Block>>,
    background: Option<Color>,
}

impl Block {
   fn new () -> Block {
       Block {
           name: None,
           id: None,
           width: None,
           width_type: None,
           height: None,
           height_type: None,
           parent: None,
           childrens: None,
           background: None
       }
   }

    fn clone (&self) -> Block {
        Block {
            name: self.name.clone(),
            id: self.id.clone(),
            width: self.width.clone(),
            width_type: self.width_type.clone(),
            height: self.height.clone(),
            height_type: self.height_type.clone(),
            parent: self.parent.clone(),
            childrens: self.childrens.clone(),
            background: self.background.clone(),
        }
    }
}

struct Main {
    blocks: Vec<Block>,
}

impl Main {
    fn new(blocks: Vec<Block>) -> Main {
        Main {
            blocks: blocks,
        }
    }
}

fn parse_view_type(value: &str) -> ViewType {
    match value {
        "percent" => ViewType::Percent,
        "pixel" => ViewType::Pixel,
        "viewportHeight" => ViewType::ViewportHeight,
        "viewportWidth" => ViewType::ViewportWidth,
        "vh" => ViewType::ViewportHeight,
        "vw" => ViewType::ViewportWidth,
        _ => ViewType::Pixel,
    }
}
fn parse_attributes(attributes: &str) -> (Option<u32>, Option<ViewType>) {
    let parts: Vec<&str> = attributes.split(',').collect();
    if parts.len() == 2 {
        (
            parts[0].trim().parse().ok(),
            Some(parse_view_type(parts[1].trim())),
        )
    } else {
        (None, None)
    }
}

fn parse_markup(markup: &str) -> Main {
    let mut blocks: Vec<Block> = Vec::new();
    let mut current_block = Block::new();
    let mut in_block = false;

    for line in markup.lines() {
        let line = line.trim();
        if line.starts_with("<block") {
            in_block = true;
            current_block = Block::new();
            if let Some(id_start) = line.find("id{") {
                if let Some(id_end) = line[id_start..].find("}") {
                    current_block.id = Some(line[id_start + 3..id_start + id_end].to_string());
                }
            }

            if let Some(height_start) = line.find("height{") {
                if let Some(height_end) = line[height_start..].find("}") {
                    let (height, height_type) = parse_attributes(&line[height_start + 7..height_start + height_end]);
                    current_block.height = height;
                    current_block.height_type = height_type;
                }
            }

            if let Some(width_start) = line.find("width{") {
                if let Some(width_end) = line[width_start..].find("}") {
                    let (width, width_type) = parse_attributes(&line[width_start + 6..width_start + width_end]);
                    current_block.width = width;
                    current_block.width_type = width_type;
                }
            }
            blocks.push(current_block.clone());
        } else if line.starts_with("</") {
            in_block = false;
        }
    }

    for block in &blocks {
        println!("{:?}", block);
    }

    let main = Main::new(blocks);

    return main;
}

fn render_view (main: &Main)
{
    for block in &main.blocks
    {
        println!("{:?}", block);
    }

}

fn main()
{
    let markup = r#"
        <main>
            <block id{element-test-id} height{50, percent}>
                <block width{50, percent} height{50, percent}/>
            </>
        </main>
        "#;

    let main = parse_markup(markup);
    println!("-----------------------------------------");
    // render_view(&main);


    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("")
        .resizable()
        .transparent()
        // .undecorated()
        .msaa_4x()
        .vsync()
        .build();

    while !rl.window_should_close()
    {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(255, 255, 255, 255));

        let mut prevent_block: Option<Block> = None;
        let mut prevent_block_clone = None;

        for block in &main.blocks
        {
            if prevent_block.is_some()
            {
                prevent_block_clone = Option::from(prevent_block.unwrap().clone());
                let mut height: f32 = prevent_block_clone.unwrap().height as f32;

                println!("---height: {:?}", height);

                if block.height.is_some()
                {
                    let height_block = block.height.unwrap();
                    let height_type = block.height_type.clone();

                    println!("height_type: {:?}", height_type);

                    match height_type
                    {
                        None => {},
                        Some(ViewType::Percent) => {
                            height = (height / 100.0) * height_block as f32;
                        },
                        Some(ViewType::Pixel) => {
                            height = height_block as f32;
                        },
                        Some(_) => {}
                    }
                }

                let width = block.width;

                d.draw_rectangle(
                    0.0 as i32,
                    0.0 as i32,
                    d.get_screen_width(),
                    height as i32,
                    Color::new(220, 0, 120, 255)
                );
            }
            else
            {
                let mut height: f32 = d.get_screen_height() as f32;
                if block.height.is_some()
                {
                    let height_block = block.height.unwrap();
                    let height_type = block.height_type.clone();

                    println!("height_type: {:?}", height_type);

                    match height_type
                    {
                        None => {},
                        Some(ViewType::Percent) => {
                            height = (d.get_screen_height() as f32  / 100.0) * height_block as f32;
                        },
                        Some(ViewType::Pixel) => {
                            height = height_block as f32;
                        },
                        Some(_) => {}
                    }

                    prevent_block_clone = Option::from(block.clone());
                    prevent_block_clone.unwrap().height = Some(height as u32);
                }

                let width = block.width;

                println!("get_screen_height: {:?}", d.get_screen_height());
                println!("height: {:?}, width: {:?}", height, width);

                d.draw_rectangle(
                    0.0 as i32,
                    0.0 as i32,
                    d.get_screen_width(),
                    height as i32,
                    Color::new(0, 0, 0, 255)
                );

                prevent_block = Option::from(&prevent_block_clone.clone());
            }
        }


    }
}

  /*
    // let mut content: String = String::new();

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("")
        .resizable()
        .transparent()
        // .undecorated()
        .msaa_4x()
        .vsync()
        .build();

    let mut  content: String = String::from("ewqeqw  wqe qwe q\nqweqweqwe\nqweqweqwewqqw");
    //ConfigFlags::FLAG_WINDOW_UNDECORATED
    // rl.gui_lock();
    // rl.gui_disable();
    // rl.clear_window_state( rl.get_window_state() );
    // rl.set_window_flags(&thread );

    // rl.set_window_

    // rl.set_target_fps(10);
    // rl.set_target_fps(1);

    // Начальная позиция скролла
    let mut scroll_y: f32 = 0.0;

    let font = rl.load_font(&thread, "Harm.ttf").expect("Could not load font");
    let text = "Open File Dialog";
    let text_position = Vector2 { x: 20.0, y: 100.0 };
    let text_size = 20;


    let mut is_dragging = false;
    let mut last_mouse_pos = Vector2::default();

    let mut last_mouse_pos: Option<Vector2> = None;
    while !rl.window_should_close()
    {
        // let mut d = rl.begin_drawing(&thread);

        let fps = rl.get_fps();

        let current_mouse_pos = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            // d.draw_text("test", 400, 50, 20, Color::BLACK);
            let mouse_position = rl.get_mouse_position();
            let text_bounds = Rectangle {
                x: text_position.x,
                y: text_position.y,
                width: (text.len() as f32) * text_size as f32,
                height: text_size as f32,
            };

            if mouse_position.x >= text_bounds.x
                && mouse_position.x <= (text_bounds.x + text_bounds.width)
                && mouse_position.y >= text_bounds.y
                && mouse_position.y <= (text_bounds.y + text_bounds.height)
            {
                if let Some(path) = FileDialog::new().pick_file() {
                    content = read_file_content(&path.to_string_lossy()).unwrap();
                    // d.draw_text("test asd asd asd asdasd asd", 400, 50, 20, Color::BLACK);
                }
            }
        }


        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {


            // println!("{:?} current_mouse_pos", current_mouse_pos);
            last_mouse_pos = Some(current_mouse_pos);
        } else if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let Some(last_pos) = last_mouse_pos else { todo!() };

            let delta = current_mouse_pos - last_pos;


            // println!("{:?} last_pos", last_pos);
            // // if let Some(last_pos) = last_mouse_pos
            // // {
            // println!("{:?} current_mouse_pos", current_mouse_pos);
            // println!("{:?} delta", delta);
            let window_pos = rl.get_window_position();
            rl.set_window_position(window_pos.x as i32 + delta.x as i32, window_pos.y as i32 + delta.y as i32);
            //     last_mouse_pos = Some(current_mouse_pos);
            // }
        } else {
            last_mouse_pos = None;
        }

        let mut d = rl.begin_drawing(&thread);


        // d.clear_background(Color::YELLOW);
        d.clear_background(Color::new(0, 0, 0, 0));
        // d.draw_rectangle(0, 0, d.get_screen_width(), d.get_screen_height() / 2, Color::BLUE);
        // d.draw_rectangle(0, d.get_screen_height() / 2, d.get_screen_width(), d.get_screen_height() / 2, Color::YELLOW);


        let width = d.get_screen_width() as f32;
        let height = d.get_screen_height() as f32;


        // d.draw_rectangle(3, 3, (width - 6.0) as i32, (height - 6.0) as i32, Color::WHITE);
        // let corner_radius = 10.0;

        // Draw circles in the corners to simulate rounded corners
        // d.draw_circle_v(Vector2::new(corner_radius, corner_radius), corner_radius, Color::WHITE);
        // d.draw_circle_v(Vector2::new(width - corner_radius, corner_radius), corner_radius, Color::WHITE);
        // d.draw_circle_v(Vector2::new(corner_radius, height - corner_radius), corner_radius, Color::WHITE);
        // d.draw_circle_v(Vector2::new(width - corner_radius, height - corner_radius), corner_radius, Color::WHITE);
        // Draw the text

        // Check if the mouse is over the text and if the mouse button is pressed
        draw_rounded_rectangle(&mut d, 3, 3, (width - 6.0) as i32,
                               (height - 6.0) as i32, 10, Color::new(43, 45, 48, 255));
        // d.draw_rectangle(3, 3, d.get_screen_width(), 30, Color::BLUE);

        // Color::new(60, 63, 65, 255) color head of the editor
        // Color::new(43, 45, 48, 255) main color of the editor


        d.draw_text(&format!("FPS: {}", fps), 380, 10, 20, Color::BLACK);
        d.draw_text_ex(&font, text, text_position, text_size as f32, 2.0, Color::BLACK);
        d.draw_text_ex(&font, "Hello, world!", Vector2 { x: 52.0, y: 12.0 }, font.base_size() as f32, 0.0, Color::BLACK);


        let wheel_move = d.get_mouse_wheel_move() as f32;
        scroll_y += wheel_move * 20.0;


        //d.draw_text(content.as_str(), 400, 50, 20, Color::BLACK);

        let mouse_position = d.get_mouse_position();

        // Переменные, которые определяют положение и размер текста
        let text_x = 100;
        let text_width = 800;
        let window_height = d.get_screen_height(); // Высота окна
        let line_height = 20;

        let first_line = (scroll_y / line_height as f32).floor() as usize;
        let last_line = ((scroll_y + window_height as f32) / line_height as f32).ceil() as usize;

        for (i, line) in content.lines().enumerate() {
            let y_position = 50 + (i as i32 * line_height) - scroll_y as i32;

            if i >= first_line && i <= last_line
            {
                if mouse_position.y >= y_position as f32 && mouse_position.y < (y_position + line_height) as f32 {
                    if mouse_position.x >= text_x as f32 && mouse_position.x <= (text_x + text_width) as f32 {
                        let rectangle = Rectangle::new(text_x as f32, y_position as f32, text_width as f32, line_height as f32);
                        d.draw_rectangle_rec(rectangle, Color::LIGHTGRAY);
                        d.draw_text(line, text_x, y_position, line_height, Color::BLACK);
                    } else {
                        d.draw_text(line, text_x, y_position, line_height, Color::BLACK);
                    }
                } else {
                    d.draw_text(line, text_x, y_position, line_height, Color::BLACK);
                }
            }
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
}


   */