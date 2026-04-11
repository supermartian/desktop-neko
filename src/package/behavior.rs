// Copyright (c) 2026 Desktop Neko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BehaviorConfig {
    pub behavior: BehaviorSettings,
    pub states: Vec<StateDef>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BehaviorSettings {
    pub initial_state: String,
    pub tick_rate_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StateDef {
    pub name: String,
    pub animation: String,
    pub movement: Option<MovementDef>,
    #[serde(default)]
    pub transitions: Vec<TransitionDef>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MovementDef {
    pub direction: String,
    pub speed_px_s: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TransitionDef {
    pub condition: ConditionDef,
    pub target: Option<String>,
    pub targets: Option<Vec<WeightedTarget>>,
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeightedTarget {
    pub state: String,
    pub weight: f32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ConditionDef {
    #[serde(rename = "cursor_near")]
    CursorNear { distance: f32 },
    #[serde(rename = "cursor_far")]
    CursorFar { distance: f32 },
    #[serde(rename = "cursor_following")]
    CursorFollowing,
    #[serde(rename = "on_click")]
    OnClick { button: String, count: u8 },
    #[serde(rename = "at_edge")]
    AtEdge { edge: String },
    #[serde(rename = "timer")]
    Timer { min_seconds: f32, max_seconds: f32 },
    #[serde(rename = "animation_done")]
    AnimationDone,
    #[serde(rename = "falling")]
    Falling,
    #[serde(rename = "random")]
    Random { probability: f32 },
}
