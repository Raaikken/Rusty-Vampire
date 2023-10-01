#![allow(unused)]

use godot::engine::{ CharacterBody2D, CharacterBody2DVirtual, PlaceholderTexture2DArray };
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Enemy {
    speed_vector: Vector2,

    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Enemy {
    #[func]
    fn get_direction_to_player(&self) -> Vector2 {
        let player_nodes = self.base.get_tree().expect("msg").get_nodes_in_group("player".into());
        if player_nodes.len() > 0 {
            let player = player_nodes.first().expect("msg").get_node_as::<Node2D>(".");
            return (player.get_global_position() - self.base.get_global_position()).normalized();
        }
        else {
            godot_print!("Player not found!");
            return Vector2::ZERO;
        }
    }
}

#[godot_api]
impl CharacterBody2DVirtual for Enemy {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Enemy {
            speed_vector: Vector2::new(100.0, 100.0),
            base,
        }
    }
    
    fn physics_process(&mut self, delta: f64) {
        let global_position = self.get_direction_to_player();
        self.base.set_velocity(global_position * self.speed_vector);
        self.base.move_and_slide();
    }
}