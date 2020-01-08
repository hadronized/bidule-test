use bidule::Stream;
use luminance_glfw::surface::{Action, GlfwSurface, Key, MouseButton, Surface, WindowDim, WindowEvent, WindowOpt};

enum Button {
  Increment,
  Decrement,
}

fn enstate_button(counter: isize, button: &Button) -> isize {
  match button {
    Button::Increment => counter + 1,
    Button::Decrement => counter - 1
  }
}

fn main() {
  let mut surface = GlfwSurface::new(WindowDim::Windowed(200, 200), "bidule test", WindowOpt::default()).unwrap();

  let plus_button = Stream::<Button>::new();
  let minus_button = Stream::<Button>::new();
  let counter = plus_button.merge(&minus_button).fold(0, enstate_button);
  let mut last_pos = None;

  counter.subscribe(|counter| println!("value: {}", counter));

  'outer: loop {
    let width = surface.width();

    for event in surface.poll_events() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'outer,

        WindowEvent::CursorPos(x, _) => {
          last_pos = Some(x);
        }

        WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => {
          if let Some(pos) = last_pos {
            if pos <= (width / 2) as f64 {
              plus_button.send(&Button::Decrement);
            } else {
              minus_button.send(&Button::Increment);
            }
          }
        }

        _ => ()
      }
    }
  }
}
