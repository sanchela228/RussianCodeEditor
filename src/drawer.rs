use std::{fs, io};
use std::io::Read;
use raylib::prelude::Font;
use raylib::prelude::*;
use rfd::FileDialog;

pub (crate) mod drawer
{
    use raylib::color::Color;
    use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
    use crate::::Event;

    pub fn draw_main(mut d: RaylibDrawHandle)
    {
        d.clear_background(Color::new(0, 0,0,0));

        let width = d.get_screen_width() as f32;
        let height = d.get_screen_height() as f32;

        draw_rounded_rectangle(&mut d, 3, 3, (width - 6.0) as i32,
                               (height - 6.0) as i32, 10, Color::new(43, 45, 48, 255));


        // mouse events generate preview event
        draw_great_flag(d);
        draw_window_menu(d);

        // draw_workflow(d); soon..

    }

    fn draw_workflow (mut d: RaylibDrawHandle)
    {

    }

    fn draw_window_menu(mut d: RaylibDrawHandle)
    {
        // close window icon

        Event.new();

        d.window_should_close();
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