use std::sync::Arc;
use rand::Rng;
use crate::package::behavior::{BehaviorConfig, ConditionDef};

pub struct StateMachine {
    pub current_state: String,
    config: Arc<BehaviorConfig>,
    pub state_elapsed_ms: u64,
    pub click_count: u8,
    pub last_click_ms: u64,
}

pub struct SmContext {
    pub cursor_distance: f32,
    pub is_cursor_following: bool,
    pub at_left_edge: bool,
    pub at_right_edge: bool,
    pub animation_done: bool,
    pub is_falling: bool,
}

impl StateMachine {
    pub fn new(config: Arc<BehaviorConfig>) -> Self {
        let initial = config.behavior.initial_state.clone();
        Self {
            current_state: initial,
            config,
            state_elapsed_ms: 0,
            click_count: 0,
            last_click_ms: 0,
        }
    }

    pub fn on_click(&mut self, button: &str) {
        if button == "left" {
            self.click_count += 1;
            self.last_click_ms = 0;
        }
    }

    pub fn tick(&mut self, dt_ms: u64, ctx: SmContext) -> bool {
        self.state_elapsed_ms += dt_ms;
        self.last_click_ms += dt_ms;
        
        if self.last_click_ms > 400 && self.click_count > 0 {
            self.click_count = 0; // reset combo
        }

        let current_def = match self.config.states.iter().find(|s| s.name == self.current_state) {
            Some(s) => s.clone(),
            None => return false,
        };

        let mut transitioned = false;
        let mut transitions = current_def.transitions;
        transitions.sort_by_key(|t| -t.priority);

        for t in transitions {
            let matches = match &t.condition {
                ConditionDef::CursorNear { distance } => ctx.cursor_distance <= *distance,
                ConditionDef::CursorFar { distance } => ctx.cursor_distance > *distance,
                ConditionDef::CursorFollowing => ctx.is_cursor_following,
                ConditionDef::OnClick { button: _button, count } => self.click_count >= *count,
                ConditionDef::AtEdge { edge } => {
                    if edge == "left" { ctx.at_left_edge } else if edge == "right" { ctx.at_right_edge } else { false }
                },
                ConditionDef::Timer { min_seconds, max_seconds } => {
                    let s = self.state_elapsed_ms as f32 / 1000.0;
                    if s >= *max_seconds { true }
                    else if s >= *min_seconds {
                        let range = max_seconds - min_seconds;
                        let chance = (dt_ms as f32 / 1000.0) / range;
                        rand::thread_rng().gen::<f32>() < chance
                    } else { false }
                },
                ConditionDef::AnimationDone => ctx.animation_done,
                ConditionDef::Falling => ctx.is_falling,
                ConditionDef::Random { probability } => rand::thread_rng().gen::<f32>() < *probability,
            };

            if matches {
                if let Some(target) = &t.target {
                    self.set_state(target);
                } else if let Some(targets) = &t.targets {
                    let total_weight: f32 = targets.iter().map(|w| w.weight).sum();
                    if total_weight > 0.0 {
                        let mut roll = rand::thread_rng().gen_range(0.0..total_weight);
                        for wt in targets {
                            roll -= wt.weight;
                            if roll <= 0.0 {
                                self.set_state(&wt.state);
                                break;
                            }
                        }
                    } else if let Some(first) = targets.first() {
                        self.set_state(&first.state);
                    }
                }
                transitioned = true;
                break;
            }
        }
        transitioned
    }

    fn set_state(&mut self, state: &str) {
        self.current_state = state.to_string();
        self.state_elapsed_ms = 0;
        self.click_count = 0;
    }
}
