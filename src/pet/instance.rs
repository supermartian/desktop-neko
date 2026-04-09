use std::sync::Arc;

use crate::package::loader::LoadedPackage;
use super::state_machine::{StateMachine, SmContext};
use super::animation::Animator;
use super::movement::MovementState;
use slint::{ComponentHandle, Weak};

pub struct PetInstance {
    pub state_machine: StateMachine,
    pub animator: Animator,
    pub movement: MovementState,
    pub package: Arc<LoadedPackage>,
    pub window_handle: Weak<crate::PetWindow>,
    pub pet_id: usize,
}

impl PetInstance {
    pub fn new(package: Arc<LoadedPackage>, window_handle: Weak<crate::PetWindow>, screen_w: f32, screen_h: f32, pet_id: usize) -> Self {
        let state_machine = StateMachine::new(Arc::new(package.behavior.clone()));
        let animator = Animator::new(vec![], package.manifest.sprite.fps);
        let movement = MovementState::new(
            package.manifest.sprite.width as f32,
            package.manifest.sprite.height as f32,
            screen_w,
            screen_h
        );
        let mut instance = Self {
            state_machine,
            animator,
            movement,
            package,
            window_handle,
            pet_id,
        };
        
        instance.apply_current_state();
        instance
    }

    pub fn apply_current_state(&mut self) {
        let state_name = &self.state_machine.current_state;
        if let Some(state_def) = self.package.behavior.states.iter().find(|s| s.name == *state_name) {
            let anim_name = &state_def.animation;
            if let Some(frames) = self.package.animations.get(anim_name) {
                let override_fps = self.package.manifest.animations.iter()
                    .find(|a| a.name == *anim_name)
                    .and_then(|a| a.fps)
                    .unwrap_or(self.package.manifest.sprite.fps);
                self.animator.reset(frames.clone(), override_fps);
            }
        }
    }

    pub fn update(&mut self, dt_ms: u64, cursor_pos: (f32, f32), screen_w: f32, screen_h: f32) {
        self.movement.resize_screen(screen_w, screen_h);
        
        let dist = ((self.movement.x + self.movement.width/2.0 - cursor_pos.0).powi(2) + 
                    (self.movement.y + self.movement.height/2.0 - cursor_pos.1).powi(2)).sqrt();
        
        let is_cursor_following = false; // Vector approach detection omitted for simplicity
        
        let ctx = SmContext {
            cursor_distance: dist,
            is_cursor_following,
            at_left_edge: self.movement.is_at_edge("left"),
            at_right_edge: self.movement.is_at_edge("right"),
            animation_done: self.animator.is_done(),
            is_falling: self.movement.is_falling,
        };

        if self.state_machine.tick(dt_ms, ctx) {
            self.apply_current_state();
        }

        self.animator.tick(dt_ms);

        let movement_def = self.package.behavior.states.iter()
            .find(|s| s.name == self.state_machine.current_state)
            .and_then(|s| s.movement.clone());

        self.movement.tick(&movement_def, dt_ms, cursor_pos);

        if let Some(window) = self.window_handle.upgrade() {
            if let Some(frame) = self.animator.current_frame() {
                window.set_current_frame(slint::Image::from_rgba8(frame));
            }
            let pos = slint::PhysicalPosition::new(self.movement.x as i32, self.movement.y as i32);
            window.window().set_position(pos);
        }
    }
}
