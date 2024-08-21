use raylib::color::Color;
use raylib::drawing::RaylibDraw;

mod rce;

fn main()
{
    let mut editor = rce::Editor::new( 800, 600 );

    let (mut rl, thread) = raylib::init()
        .size(editor.width, editor.height)
        .title(&*editor.title)
        .resizable()
        .transparent()
        .undecorated()
        .msaa_4x()
        .vsync()
        .build();

    // main loop
    while !rl.window_should_close()
    {
        let WUNDOW_WUDTH = rl.get_screen_width();
        let WUNDOW_HEIGHT = rl.get_screen_height();

        if editor.files.len() == 0
        {
            editor.open_file_dialog();
        }
        else
        {
            let mut d = rl.begin_drawing(&thread);

            rce::drawer::draw_main(d);

            while !rl.window_should_close()
            {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    editor.mouse_click(&mut d);
                }

            }




            // d.clear_background(Color::new(0, 0,0,0));
        }
    }
}