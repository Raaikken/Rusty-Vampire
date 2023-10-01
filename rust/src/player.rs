#![allow(unused)]

use godot::engine::{CharacterBody2D, CharacterBody2DVirtual, Sprite2D, Area2D, CollisionShape2D };
use godot::prelude::*;
use godot::prelude::Vector2;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f32,

    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Player {
    #[func]
    fn get_direction(&self) -> Vector2 {
        let mut direction = Vector2::new(0.0, 0.0);
        let input = Input::singleton();

        direction.x = input.get_action_strength("move_right".into()) - input.get_action_strength("move_left".into());
        direction.y = input.get_action_strength("move_down".into()) - input.get_action_strength("move_up".into());

        return direction;
    }
}

#[godot_api]
impl CharacterBody2DVirtual for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Player {
            speed: 500.0,
            base,
        }
    }
    
    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        if input.is_action_just_pressed("quit".into()) {
            self.base.get_tree().expect("Quit").quit();
        }

        let move_direction = self.get_direction();
        let move_speed = move_direction.normalized() * Vector2::new(self.speed, self.speed);
        self.base.set_velocity(move_speed);
        self.base.move_and_slide();

    }
}

