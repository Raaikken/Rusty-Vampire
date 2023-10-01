use godot::engine::{ Camera2D, Camera2DVirtual };
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Camera2D)]
struct GameCamera {
    target_position: Vector2,

    #[base]
    base: Base<Camera2D>,
}

#[godot_api]
impl GameCamera {
    #[func]
    fn get_target(&self) -> Vector2 {
        let player_nodes = self.base.get_tree().expect("msg").get_nodes_in_group("player".into());
        if player_nodes.len() > 0 {
            let player = player_nodes.first().expect("msg").get_node_as::<Node2D>(".");
            return player.get_global_position()
        }
        else {
            godot_print!("Player not found!");
            return Vector2::ZERO;
        }
    }
}

#[godot_api]
impl Camera2DVirtual for GameCamera {
    fn init(mut base: Base<Camera2D>) -> Self {
        base.make_current();

        GameCamera { 
            target_position: Vector2 { x: 0.0, y: 0.0 },
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.target_position = self.get_target();
        let global_position = self.base.get_global_position();
        self.base.set_global_position(global_position.lerp(self.target_position, 1.0_f32.powf(-delta as f32 * 10.0)));
    }
}