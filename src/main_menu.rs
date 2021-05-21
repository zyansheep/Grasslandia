use bevy::prelude::*;

use crate::GameState;

pub struct ButtonMaterials {
	normal: Handle<ColorMaterial>,
	hovered: Handle<ColorMaterial>,
	pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
	fn from_world(world: &mut World) -> Self {
		let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
		ButtonMaterials {
			normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
			hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
			pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
		}
	}
}
pub fn button_system(
	button_materials: Res<ButtonMaterials>,
	mut interaction_query: Query<
		(&Interaction, &mut Handle<ColorMaterial>),
		(Changed<Interaction>, With<Button>),
	>,
	mut state: ResMut<State<GameState>>,
) {
	for (interaction, mut material) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				*material = button_materials.pressed.clone();
				state.set(GameState::InGame).unwrap();

			}
			Interaction::Hovered => {
				*material = button_materials.hovered.clone();
			}
			Interaction::None => {
				*material = button_materials.normal.clone();
			}
		}
	}
}

pub struct TitleText;
pub struct MenuItem;

pub fn text_rotation(mut query: Query<&mut GlobalTransform, With<TitleText>>, time: Res<Time>) {
	let mut text_transform = query.single_mut().unwrap();
	text_transform.rotation = Quat::from_rotation_z(time.seconds_since_startup().cos() as f32);
}

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	button_materials: Res<ButtonMaterials>,
) {
	// Main Menu Container
	commands.spawn_bundle(NodeBundle {
		style: Style {
			size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			flex_direction: FlexDirection::ColumnReverse,
			..Default::default()
		},
		//material: materials.add(Color::rgba_u8(0, 0, 0, 120).into()),
		..Default::default()
	}).with_children(|cb| {
		// Title Text
		cb.spawn_bundle(TextBundle {
			text: Text::with_section(
				"Grasslandia",
				TextStyle {
					font: asset_server.load("fonts/monkey.ttf"),
					font_size: 60.0,
					color: Color::BLACK,
				},
				/* TextAlignment {
					vertical: VerticalAlign::Center,
					horizontal: HorizontalAlign::Center,
				}, */
				Default::default()
			),
			style: Style {
				//align_self: AlignSelf::Auto,
				..Default::default()
			},
			..Default::default()
		}).insert(TitleText);
		// Main Button
		cb.spawn_bundle(ButtonBundle {
			style: Style {
				size: Size::new(Val::Px(280.0), Val::Px(80.0)),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				//align_self: AlignSelf::FlexEnd,
				..Default::default()
			},
			material: button_materials.normal.clone(),
			..Default::default()
		}).with_children(|cb| {
			cb.spawn_bundle(TextBundle {
				text: Text::with_section(
					"Start Game",
					TextStyle {
						font: asset_server.load("fonts/font.ttf"),
						font_size: 40.0,
						color: Color::rgb(0.9, 0.9, 0.9),
					},
					//TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center },
					Default::default()
				),
				..Default::default()
			});
		});
	}).insert(MenuItem);
}
pub fn exit(mut commands: Commands, items: Query<Entity, With<MenuItem>>) {
	for entity in items.iter() {
		commands.entity(entity).despawn_recursive();
	}
}