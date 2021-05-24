#![allow(non_snake_case)]
#![feature(try_blocks)]

use bevy::{prelude::*, window::WindowResizeConstraints};

mod level;
mod main_menu;
use level::LevelAsset;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
	MainMenu,
	InGame,
	Paused,
}

fn main() {
	env_logger::init();
	App::build()
		.insert_resource(WindowDescriptor {
			title: "Grasslandia".to_string(),
			width: 600.,
			height: 600.,
			vsync: true,
			resize_constraints: WindowResizeConstraints {
				min_height: 400.0,
				min_width: 400.0,
				..Default::default()
			},
			..Default::default()
		})
		.add_plugins(DefaultPlugins)
		.add_state(GameState::InGame) // Starting Game State
		// Main Menu Systems
		.init_resource::<main_menu::ButtonMaterials>()
		.add_system_set(
			SystemSet::on_enter(GameState::MainMenu).with_system(main_menu::setup.system()),
		)
		.add_system_set(
			SystemSet::on_update(GameState::MainMenu)
				.with_system(main_menu::button_system.system()), //.with_system(main_menu::text_rotation.system()),
		)
		.add_system_set(
			SystemSet::on_exit(GameState::MainMenu).with_system(main_menu::exit.system()),
		)
		// Game Systems
		.add_asset::<level::LevelAsset>() // Level Asset
		.init_asset_loader::<level::LevelAssetLoader>()
		.init_resource::<InGameState>()
		.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game.system()))
		.add_system_set(
			SystemSet::on_update(GameState::InGame)
				.with_system(update_game.system())
				.with_system(player_movement.system()),
		)
		// Pause Menu
		.add_system_set(
			SystemSet::on_enter(GameState::Paused).with_system(pause_menu_setup.system()),
		)
		.add_system_set(
			SystemSet::on_update(GameState::Paused).with_system(pause_menu_update.system()),
		)
		.add_system_set(SystemSet::on_exit(GameState::Paused).with_system(pause_menu_exit.system()))
		// Universal Systems
		.add_startup_system(main_setup.system())
		.add_system(main_update.system())
		.run();
}

#[derive(Default)]
struct InGameState {
	current_level: Handle<LevelAsset>,
	//next_level: Option<Handle<LevelAsset>>,
	rendered: bool,
}
fn setup_game(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut state: ResMut<InGameState>,
) {
	commands.spawn_bundle(SpriteBundle {
		material: materials.add(asset_server.load("backgrounds/creepybackground.png").into()),
		..Default::default()
	});
	state.current_level = asset_server.load::<LevelAsset, _>("game/levels/level0.glevel");
	asset_server.watch_for_changes().unwrap();
	/* commands
	.spawn_bundle(SpriteSheetBundle {
		texture_atlas: texture_atlas_handle,
		transform: Transform::from_scale(Vec3::splat(6.0)),
		..Default::default()
	})
	.insert(Timer::from_seconds(0.1, true)); */

	log::info!("Wake up...");
}

fn update_game(
	mut state: ResMut<InGameState>,
	levels: Res<Assets<LevelAsset>>,
	mut ev_level: EventReader<AssetEvent<LevelAsset>>
) {
	for ev in ev_level.iter() {
		match ev {
			AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
				if let Some(level) = levels.get(handle) {
					println!("Level Loaded: {:?}", level);
					state.rendered = true;
				} else {
					println!("Level not loaded");
				}
			},
			_ => { println!("Level removed"); },
		}
	}
}
fn player_movement(
	mut camera_transform: Query<&mut Transform, With<MainCamera>>,
	keys: Res<Input<KeyCode>>,
	//mut player: Query<&mut Transform, With<Player>>,
) {
	let mut camera_transform = camera_transform.single_mut().unwrap();
	let speed = 10.0;
	if keys.pressed(KeyCode::Up) {
		camera_transform.translation.y += speed;
	}
	if keys.pressed(KeyCode::Down) {
		camera_transform.translation.y -= speed;
	}
	if keys.pressed(KeyCode::Left) {
		camera_transform.translation.x -= speed;
	}
	if keys.pressed(KeyCode::Right) {
		camera_transform.translation.x += speed;
	}
}

struct MainCamera;
struct GameStateText;
fn main_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	//mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let main_font = asset_server.load("fonts/font.ttf");
	commands
		.spawn_bundle(OrthographicCameraBundle::new_2d())
		.insert(MainCamera);
	commands.spawn_bundle(UiCameraBundle::default());
	commands
		.spawn_bundle(TextBundle {
			style: Style {
				align_self: AlignSelf::FlexEnd,
				position_type: PositionType::Absolute,
				position: Rect {
					bottom: Val::Px(5.0),
					left: Val::Px(5.0),
					..Default::default()
				},
				..Default::default()
			},
			// Use the `Text::with_section` constructor
			text: Text::with_section(
				"GameState: ???",
				TextStyle {
					font: main_font,
					font_size: 20.0,
					color: Color::WHITE,
				},
				Default::default(),
			),
			..Default::default()
		})
		.insert(GameStateText);
}
fn main_update(
	keys: Res<Input<KeyCode>>,
	mut game_state: ResMut<State<GameState>>,
	mut game_state_text: Query<&mut Text, With<GameStateText>>,
) {
	match game_state.current() {
		GameState::MainMenu => {
			if keys.just_pressed(KeyCode::Space) {
				game_state.set(GameState::InGame).unwrap();
			}
		}
		GameState::InGame => {
			if keys.just_pressed(KeyCode::Escape) {
				game_state.push(GameState::Paused).unwrap();
			}
		}
		GameState::Paused => {
			if keys.just_pressed(KeyCode::Escape) {
				game_state.pop().unwrap();
			}
		}
	}
	if game_state.is_changed() {
		let mut game_state_text = game_state_text.single_mut().unwrap();
		game_state_text.sections[0].value = format!("Game State: {:?}", game_state.current());
	}
}

struct PauseMenuItem;
fn pause_menu_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let main_font = asset_server.load("fonts/font.ttf");
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				flex_direction: FlexDirection::Column,
				..Default::default()
			},
			material: materials.add(Color::rgba_u8(0, 0, 0, 120).into()),
			..Default::default()
		})
		.with_children(|cb| {
			cb.spawn_bundle(TextBundle {
				text: Text::with_section(
					"Game\nPaused",
					TextStyle {
						font: main_font,
						font_size: 50.0,
						color: Color::WHITE,
					},
					TextAlignment {
						vertical: VerticalAlign::Center,
						horizontal: HorizontalAlign::Center,
					},
				),
				style: Style {
					..Default::default()
				},
				..Default::default()
			});
		})
		.insert(PauseMenuItem);
}
fn pause_menu_update() {}
fn pause_menu_exit(mut commands: Commands, items: Query<Entity, With<PauseMenuItem>>) {
	for item in items.iter() {
		commands.entity(item).despawn_recursive()
	}
}
