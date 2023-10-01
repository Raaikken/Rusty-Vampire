#![allow(unused)]

use godot::engine::{ Node2D, Node2DVirtual };
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct SwordAbility {

    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl SwordAbility {
    #[func]
    fn hello_godot() {
        godot_print!("Hello, Godot!");
    }
}

#[godot_api]
impl Node2DVirtual for SwordAbility {
    fn init(mut base: Base<Node2D>) -> Self {
        SwordAbility {
            base,
        }
    }

    fn ready(&mut self) {
        
    }

    fn physics_process(&mut self, delta: f64) {

    }
}