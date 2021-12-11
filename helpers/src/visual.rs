use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::fs::create_dir_all;

use image::GrayImage;
use itertools::Itertools;

use super::display;

pub fn display_2d_vec<V>(grid: &[Vec<V>])
where
	V: Display,
{
	display(string_2d_vec(grid));
}

pub fn string_2d_vec<V>(grid: &[Vec<V>]) -> String
where
	V: Display,
{
	let width =
		grid.iter()
			.flatten()
			.map(|v| v.to_string().len())
			.max()
			.unwrap() + 1;
	let mut screen = String::new();
	for y in grid {
		for v in y {
			write!(screen, "{:>width$}", v).unwrap();
		}
		screen.push('\n');
	}
	screen
}

pub fn image_2d_vec(grid: &[Vec<usize>], name: &str) {
	let (vmin, vmax) = grid
		.iter()
		.flatten()
		.copied()
		.minmax()
		.into_option()
		.unwrap();
	let vmax = vmax.max(1);
	let mut img = GrayImage::new(grid[0].len() as u32, grid.len() as u32);
	for (grid_row, img_row) in grid.iter().zip(img.rows_mut()) {
		for (&v, pixel) in grid_row.iter().zip(img_row) {
			let color = (v - vmin) * 255 / vmax;
			pixel.0 = [color as u8];
		}
	}
	create_dir_all("visualizations").unwrap();
	img.save(format!("visualizations/{name}.png")).unwrap();
}

pub fn display_2d_map<V>(map: &HashMap<[isize; 2], V>, default: &str)
where
	V: Display,
{
	display(string_2d_map(map, default));
}

pub fn string_2d_map<V>(map: &HashMap<[isize; 2], V>, default: &str) -> String
where
	V: Display,
{
	let (xmin, xmax) = map.keys().map(|&[x, _]| x).minmax().into_option().unwrap();
	let (ymin, ymax) = map.keys().map(|&[_, y]| y).minmax().into_option().unwrap();
	let width = map.values().map(|v| v.to_string().len()).max().unwrap() + 1;
	let mut screen = String::new();
	for y in ymin..=ymax {
		for x in xmin..=xmax {
			if let Some(v) = map.get(&[x, y]) {
				write!(screen, "{:>width$}", v).unwrap();
			} else {
				write!(screen, "{:>width$}", default).unwrap();
			}
		}
		screen.push('\n');
	}
	screen
}

pub fn image_2d_map(map: &HashMap<[isize; 2], isize>, background: isize, name: &str) {
	let (xmin, xmax) = map.keys().map(|&[x, _]| x).minmax().into_option().unwrap();
	let (ymin, ymax) = map.keys().map(|&[_, y]| y).minmax().into_option().unwrap();
	let (vmin, vmax) = map
		.values()
		.copied()
		.chain(Some(background))
		.minmax()
		.into_option()
		.unwrap();
	let mut img = GrayImage::new((xmax - xmin + 1) as u32, (ymax - ymin + 1) as u32);
	for (y, row) in (ymin..=ymax).zip(img.rows_mut()) {
		for (x, pixel) in (xmin..=xmax).zip(row) {
			let map_value = map.get(&[x, y]).copied().unwrap_or(background);
			let color = (map_value - vmin) * 255 / vmax;
			pixel.0 = [color as u8];
		}
	}
	create_dir_all("visualizations").unwrap();
	img.save(format!("visualizations/{name}.png")).unwrap();
}
