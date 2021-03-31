#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod double_pendulum;
use std::f64;

use double_pendulum::DoublePendulum;

// ------ ------
//     Init
// ------ ------

const WIDTH: u64 = 500;
const HEIGHT: u64 = 500;
const ROD_WIDTH: f64 = 10.0;
const ROD_LENGTH: f64 = 100.0;
const CIRCLE_SCALE: f64 = 0.8;

fn init_dp() -> DoublePendulum {
    DoublePendulum::new(f64::consts::FRAC_PI_4, 0.0, 0.0, 0.0, ROD_LENGTH, 5.0)
}

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        t: 0.0,
        playing: true,
        system: init_dp(),
    }
}

// ------ ------
//     Model
// ------ ------

#[derive(Copy, Clone, Debug)]
/// The model.
struct Model {
    /// The time.
    t: f64,
    /// Whether the simulation is playing.
    playing: bool,
    /// The state:
    system: DoublePendulum,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    TogglePause,
    Reset,
    Tick(f64),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::TogglePause => {
            model.playing = !model.playing;
        }
        Msg::Reset => {
            *model = Model {
                t: 0.0,
                playing: true,
                system: init_dp(),
            }
        }
        Msg::Tick(dt) => model.system.multi_step(dt, 5),
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let dp = model.system;
    let (ox, oy) = ((WIDTH / 2) as f64, (HEIGHT / 2) as f64);
    let (jx, jy) = dp.get_joint_svg();
    div![
        svg![
            attrs! {
                At::ViewBox => format!("{} {} {} {}", 0, 0, WIDTH, HEIGHT);
            },
            rect![attrs! {
                At::Width => ROD_LENGTH,
                At::Height => ROD_WIDTH,
                At::X => ox,
                At::Y => oy - ROD_WIDTH / 2.0,
                At::Fill => "blue",
                At::Transform => format!("rotate({}, {}, {})",
                                         (f64::consts::FRAC_PI_2 - dp.a1).to_degrees(),
                                         ox,
                                         oy),
            }],
            rect![attrs! {
                At::Width => ROD_LENGTH,
                At::Height => ROD_WIDTH,
                At::X => ox + jx,
                At::Y => ox + jy - ROD_WIDTH / 2.0,
                At::Fill => "red",
                At::Transform => format!("rotate({}, {}, {})",
                                         (f64::consts::FRAC_PI_2 - dp.a2).to_degrees(),
                                         ox + jx,
                                         oy + jy)
            }],
            circle![attrs! {
                At::Cx => ox,
                At::Cy => oy,
                At::R => ROD_WIDTH * CIRCLE_SCALE,
                At::Stroke => "black",
                At::Fill => "black"
            }],
            circle![attrs! {
                At::Cx => ox + jx,
                At::Cy => oy + jy,
                At::R => ROD_WIDTH * CIRCLE_SCALE,
                At::Stroke => "black",
                At::Fill => "black"
            }]
        ],
        button![
            "Tick",
            attrs! {At::Id => "ticker"},
            ev(Ev::Click, |_| Msg::Tick(0.1)),
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
