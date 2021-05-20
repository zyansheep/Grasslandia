use bevy::prelude::*;

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
) {
	for (interaction, mut material) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				*material = button_materials.pressed.clone();
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

pub fn text_rotation(mut query: Query<&mut Transform, With<TitleText>>, time: Res<Time>) {
	let mut text_transform = query.single_mut().unwrap();
	text_transform.rotation = Quat::from_rotation_z(time.seconds_since_startup().cos() as f32 * 20.0);
}

pub struct Components {
	title_text: Entity,
	button: Entity,
}

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	button_materials: Res<ButtonMaterials>,
) {
	// Main Menu Text
	commands
		.spawn_bundle(Text2dBundle {
			text: Text::with_section(
				"Grasslandia",
				TextStyle {
					font: asset_server.load("fonts/monkey.ttf"),
					font_size: 60.0,
					color: Color::BLACK,
				},
				TextAlignment {
					vertical: VerticalAlign::Center,
					horizontal: HorizontalAlign::Center,
				},
			),
			..Default::default()
		})
    	.insert(MenuItem)
		.insert(TitleText);

	// Main Button
	commands
		.spawn_bundle(ButtonBundle {
			style: Style {
				size: Size::new(Val::Px(280.0), Val::Px(80.0)),
				// center button
				margin: Rect::all(Val::Auto),
				// horizontally center child text
				justify_content: JustifyContent::Center,
				// vertically center child text
				align_items: AlignItems::Center,
				..Default::default()
			},
			material: button_materials.normal.clone(),
			..Default::default()
		})
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					"Start Game",
					TextStyle {
						font: asset_server.load("fonts/font.ttf"),
						font_size: 40.0,
						color: Color::rgb(0.9, 0.9, 0.9),
					},
					Default::default(),
				),
				..Default::default()
			});
		}).insert(MenuItem);
}
pub fn exit(mut commands: Commands, items: Query<Entity, With<MenuItem>>) {
	for entity in items.iter() {
		commands.entity(entity).despawn_recursive();
	}
}