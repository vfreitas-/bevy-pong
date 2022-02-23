use bevy::prelude::*;
use crate::{GameState, level::GoalLineSide};

pub struct OnScore(pub GoalLineSide);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Scoreboard {
        player_left: 0,
        player_right: 0,
      })
      .add_event::<OnScore>()
      .add_system(score_update_ui);
  }
}

#[derive(Default, Debug)]
pub struct Scoreboard {
  pub player_left: usize,
  pub player_right: usize,
}

fn score_update_ui (
  mut scoreboard: ResMut<Scoreboard>,
  mut events: EventReader<OnScore>,
) {
  for ev in events.iter() {
    match ev.0 {
      GoalLineSide::Left => {
        scoreboard.player_left += 1;
      }
      GoalLineSide::Right => {
        scoreboard.player_right += 1;
      }
    };
  }
}