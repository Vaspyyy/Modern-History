use crate::army::Army;
use crate::city::Capital;
use bevy::prelude::*;

pub fn apply_supply(mut armies: Query<&mut Army>, capitals: Query<(&Capital, &Transform)>) {
    let capitals_vec: Vec<_> = capitals.iter().collect();

    for mut army in &mut armies {
        let mut nearest_dist = f32::MAX;

        for (capital, capital_transform) in &capitals_vec {
            if capital.faction == army.faction {
                let capital_pos = capital_transform.translation.truncate();
                let dist = army.position.distance(capital_pos);

                if dist < nearest_dist {
                    nearest_dist = dist;
                }
            }
        }

        if nearest_dist < 200.0 {
            army.strength += 2.0;
        } else {
            army.strength -= 1.0;
        }

        army.strength = army.strength.max(100.0);
    }
}
