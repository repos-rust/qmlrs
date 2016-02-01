#[macro_use]
extern crate qmlrs;
mod math;
mod math_objects;

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("factorial", math::Factorial);
    engine.load_local_file("examples/factorial_ui.qml");

    engine.exec();
}
