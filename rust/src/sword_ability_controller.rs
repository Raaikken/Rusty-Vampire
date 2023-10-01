#![allow(unused)]

use godot::engine::{ Node2D, Node2DVirtual, Timer };
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct SwordAbilityController {
    ability_scene: Gd<PackedScene>,

    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl SwordAbilityController {
    #[func]
    fn hello_godot() {
        godot_print!("Hello, Godot!");
    }

    #[func]
    fn on_timeout(&mut self) {
        let mut enemies = self.base.get_tree().expect("msg").get_nodes_in_group("enemy".into());
        if enemies.len() == 0 {
            return;
        }

        let player_nodes = self.base.get_tree().expect("msg").get_nodes_in_group("player".into());
        let player = player_nodes.first().expect("msg").get_node_as::<Node2D>(".");

        while enemies.len() > 0 {
            let mut enemy = enemies.pop().expect("msg").get_node_as::<Node2D>(".");
            if enemy.get_global_position().distance_squared_to(player.get_global_position()) < 250.0 {
                
            }
        }

        let mut ability_instance = self.ability_scene.instantiate_as::<Node2D>();
        player.get_parent().expect("msg").add_child(ability_instance.clone().upcast());
        ability_instance.set_global_position(player.get_global_position());
    }
}

#[godot_api]
impl Node2DVirtual for SwordAbilityController {
    fn init(mut base: Base<Node2D>) -> Self {
        SwordAbilityController {
            ability_scene: PackedScene::new(),
            base,
        }
    }

    fn ready(&mut self) {
        self.ability_scene = load("res://abilities/sword_ability.tscn");
        let mut timer = self.base.get_node_as::<Timer>("Timer");
        timer.start();
    }

    fn physics_process(&mut self, delta: f64) {

    }
}