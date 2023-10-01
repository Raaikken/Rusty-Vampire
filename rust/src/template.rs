#![allow(unused)]

use godot::engine::{ Class, ClassVirtual };
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Class)]
struct Template {

    #[base]
    base: Base<Class>,
}

#[godot_api]
impl Template {
    #[func]
    fn hello_godot() {
        godot_print("Hello, Godot!");
    }
}

#[godot_api]
impl ClassVirtual for Template {
    fn init(mut base: Base<Class>) -> Self {
        base.make_current();

        Template { 
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {

    }
}