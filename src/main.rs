use std::ops::Deref;
use raylib::ffi::ConfigFlags;
use raylib::prelude::*;
use rfd::FileDialog;


struct Editor {

}

impl Editor{
    fn  open_file_dialog(&self) {

    }
}

fn main()
{
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("")
        .resizable()
        .transparent()
        .undecorated()
        .msaa_4x()
        // .deref()
        .build();


    //ConfigFlags::FLAG_WINDOW_UNDECORATED
    // rl.gui_lock();
    // rl.gui_disable();
    // rl.clear_window_state( rl.get_window_state() );
    // rl.set_window_flags(&thread );

    // rl.set_window_

    rl.set_target_fps(60);
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

        let fps = rl.get_fps();

        let current_mouse_pos = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            // println!("{:?} current_mouse_pos", current_mouse_pos);
            last_mouse_pos = Some(current_mouse_pos);
        }
        else if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let Some(last_pos) = last_mouse_pos else { todo!() };

            let delta = current_mouse_pos - last_pos;


            println!("{:?} last_pos", last_pos);
            // if let Some(last_pos) = last_mouse_pos
            // {
            println!("{:?} current_mouse_pos", current_mouse_pos);
            println!("{:?} delta", delta);
                let window_pos = rl.get_window_position();
                rl.set_window_position(window_pos.x as i32 + delta.x as i32, window_pos.y as i32 + delta.y as i32);
            //     last_mouse_pos = Some(current_mouse_pos);
            // }
        }
        else
        {
            last_mouse_pos = None;
        }





        let mut d = rl.begin_drawing(&thread);



        // d.clear_background(Color::YELLOW);
        d.clear_background(Color::new(0, 0,0,0));
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


        // d.draw_text(&format!("FPS: {}", fps), 380, 10, 20, Color::BLACK);
        // d.draw_text_ex(&font, text, text_position, text_size as f32, 2.0, Color::BLACK);
        // d.draw_text_ex(&font, "Hello, world!", Vector2 { x: 52.0, y: 12.0 }, font.base_size() as f32, 0.0, Color::BLACK);
        // Check if the mouse is over the text and if the mouse button is pressed
        draw_rounded_rectangle(&mut d, 3, 3, (width - 6.0) as i32,
                               (height - 6.0) as i32, 10, Color::new(43, 45, 48, 255));
        // d.draw_rectangle(3, 3, d.get_screen_width(), 30, Color::BLUE);

        // Color::new(60, 63, 65, 255) color head of the editor
        // Color::new(43, 45, 48, 255) main color of the editor

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



        // Обновление позиции скролла
        let wheel_move = d.get_mouse_wheel_move() as f32;
        scroll_y += wheel_move * 20.0; // Множитель для скорости скролла

        // Используйте scroll_y для изменения координат отрисовки контента
        // Например, отрисовка текста с учетом позиции скролла
        d.draw_text("Scrollable content", 100, 50 - scroll_y as i32, 20, Color::BLACK);

        // Дополнительный контент, который также должен скроллиться
        draw_rounded_rectangle(&mut d, 113, 113, 200,
                               200, 10, Color::BLUE);
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