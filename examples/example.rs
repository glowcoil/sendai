use sendai::backends::glfw::run;
use sendai::Button;
use sendai::gouache::PathBuilder;

fn main() {
    let play_icon = PathBuilder::new()
        .move_to(4.0, 3.0)
        .line_to(4.0, 13.0)
        .line_to(12.0, 8.0)
        .build();
    let mut button = Button::new(play_icon);

    run(&mut button);
}
