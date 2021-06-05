#![allow(dead_code)]

use std::{collections::HashMap, path::Path};
use bevy_tilemap::{Tilemap, prelude::GridTopology};
use ndarray::Array3;
use bevy::{asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset}, prelude::*, reflect::TypeUuid, utils::BoxedFuture};
use anyhow::anyhow;

#[derive(Debug)]
pub enum BlockInteraction {
	/// Wall, can not pass through
	Wall,
	/// Air, can pass through
	Air,
	/// Goal, teleports to another level
	Goal(String),
	/// Hole, move to a lower layer
	Hole,
	/// Unknown Block Type
	Unknown(String),
}
#[derive(Debug)]
pub struct Block {
	pub interaction: BlockInteraction,
	pub texture: Option<Handle<Texture>>,
}
#[derive(Debug)]
pub enum LevelFlags {
	SoundFile(Handle<AudioSource>),
	VerticalSize(u32),
	Unknown(String),
}

#[derive(Debug, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct LevelAsset {
	// Name of the level
	pub name: String,
	// Specific Level Flags (i.e. background sound, color, etc.)
	pub flags: Vec<LevelFlags>,
	// 3D Tilemap
	pub tiles: Array3<usize>,
	// List of all block types
	pub blocks: Vec<Block>,
	/// Store the tilemap
	pub tilemap: Tilemap,
}

#[derive(Default)]
pub struct LevelAssetLoader;

impl AssetLoader for LevelAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
			// Load from bytes
			use std::io::BufRead;
			let lines = bytes.lines().filter_map(|x|x.ok()).collect::<Vec<String>>();
			
			let mut dep_asset_paths: Vec<AssetPath<'_>> = Vec::new();
			// let texture_assets: Assets<Texture>::;

			let mut dims = lines[1].split("x");
			let dimensions = ndarray::Dim([
				dims.next().ok_or(anyhow!("no dimensions"))?.parse::<usize>()?,
				dims.next().ok_or(anyhow!("no y dimension"))?.parse::<usize>()?,
				dims.next().and_then(|s|s.parse::<usize>().ok()).unwrap_or(1),
			]);
			
			let mut iden_map = HashMap::<&str, usize>::new(); // Map |d|d|1|1| to indexes into block type array
			let blocks = lines[4..].iter().enumerate().filter_map(|(i,s)| try {
				let mut split = s.split(",");
				let block_char = split.next()?;
				iden_map.insert(block_char, i); // Save to char map
				let flags = split.next()?;
				
				let mut block_texture = None;
				let mut block_interaction = BlockInteraction::Wall;
				for flag in flags.split(",") {
					let mut flag_split = flag.split("=");
					match flag_split.next()? {
						"air" => {
							block_interaction = BlockInteraction::Air;
						},
						"texture" => {
							if let Some(texture_path) = flag_split.next() {
								dep_asset_paths.push(Path::new(texture_path).to_owned().into());
								let handle = load_context.get_handle(texture_path);
								block_texture = Some(handle);
							}
						},
						_ => {},
					}
				}
				Block {
					texture: block_texture,
					interaction: block_interaction,
				}
			}).collect::<Vec<Block>>();


			let tilemap = Tilemap::builder()
				.auto_chunk()
				.topology(GridTopology::Square)
				.dimensions(3, 3)
				.chunk_dimensions(32, 32, 1)
				.texture_dimensions(32, 35)
				.z_layers(3)
				//.texture_atlas(atlas_handle)
				.finish()
				.unwrap();

			let level = LevelAsset {
				name: lines[0].clone(),
				flags: {
					lines[2].split(",").filter_map(|s| try {
						let mut split = s.split("=");
						let key = split.next()?;
						let value = split.next()?;
						match key {
							"vsize" => LevelFlags::VerticalSize(value.parse::<u32>().ok()?),
							"sound" => {
								dep_asset_paths.push(Path::new(value).to_owned().into());
								LevelFlags::SoundFile(load_context.get_handle(value))
							},
							_ => { LevelFlags::Unknown(value.to_owned()) }
						}
					}).collect::<Vec<LevelFlags>>()
				},
				
				tiles: Array3::from_shape_vec(
					dimensions,
					lines[3].split("|").map(|s|*iden_map.get(&s).unwrap_or(&0)).collect::<Vec<usize>>() // Map char to index and collect into 3D array
				)?,
				blocks,
				tilemap,
			};
            //let custom_asset = ron::de::from_bytes::<CustomAsset>(bytes)?;
			let loaded_asset = LoadedAsset::new(level);
			let loaded_asset = loaded_asset.with_dependencies(dep_asset_paths);
            load_context.set_default_asset(loaded_asset);

			log::info!("Loaded Level");
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["glevel"]
    }
}
