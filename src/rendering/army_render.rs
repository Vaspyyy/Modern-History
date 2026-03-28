use crate::army::Army;
use bevy::prelude::*;

#[derive(Component)]
pub struct ArmyVisual;

#[derive(Component)]
pub struct ArmyText;

pub fn attach_army_visuals(
    mut commands: Commands,
    query: Query<(Entity, &Army), Without<ArmyVisual>>,
) {
    for (entity, army) in &query {
        commands
            .entity(entity)
            .try_insert((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::splat(4.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(army.position.x, army.position.y, 2.0),
                    ..default()
                },
                ArmyVisual,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            format!("{}", army.strength as i32),
                            TextStyle {
                                font_size: 10.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(0.0, 8.0, 3.0),
                        ..default()
                    },
                    ArmyText,
                ));
            });
    }
}

pub fn update_army_visuals(
    mut query: Query<(&Army, &mut Transform, &mut Sprite), With<ArmyVisual>>,
) {
    for (army, mut transform, mut sprite) in &mut query {
        transform.translation.x = army.position.x;
        transform.translation.y = army.position.y;

        let size = (army.strength.sqrt() * 0.1).clamp(3.0, 8.0);
        sprite.custom_size = Some(Vec2::splat(size));
    }
}

pub fn cleanup_orphan_army_text(
    mut commands: Commands,
    text_query: Query<(Entity, &Parent), With<ArmyText>>,
    armies: Query<&Army>,
) {
    for (text_entity, parent) in &text_query {
        if armies.get(parent.get()).is_err() {
            commands.entity(text_entity).despawn();
        }
    }
}

pub fn update_army_text(
    query: Query<(&Army, &Children), With<ArmyVisual>>,
    mut text_query: Query<&mut Text, With<ArmyText>>,
) {
    for (army, children) in &query {
        for &child in children {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.sections[0].value = format!("{}", army.strength as i32);
            }
        }
    }
}
