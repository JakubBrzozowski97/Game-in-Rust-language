use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};


#[derive(Debug)]
pub struct Player  {
    pub position: Point,
    pub size: Rect,
    pub t_size: u32,
    pub color : Color,
}

#[derive(Debug)]
pub struct Enemy  {
    pub position: Point,
    pub size: Rect,
    pub t_size: u32,
    pub color : Color,
}

#[derive(Debug)]
pub struct Projectile  {
    pub position: Point,
    pub size: Rect,
    pub t_size: u32,
    pub color : Color,
}

#[derive(Debug)]
pub struct Lives  {
    pub position: Point,
    pub size: Rect,
    pub t_size: u32,
    pub color : Color,
}
#[derive(Debug)]
pub struct Liveup  {
    pub position: Point,
    pub size: Rect,
    pub t_size: u32,
    pub color : Color,
}

#[derive(Debug)]
pub struct Bulletup  {
    pub position: Point,
    pub size: Rect,
    pub t_size: u32,
    pub color : Color,
}

#[derive(Debug)]
pub struct Pointsymbol  {
    pub position: Point,
    pub size: Rect,
    pub t_size: u32,
    pub color : Color,
}