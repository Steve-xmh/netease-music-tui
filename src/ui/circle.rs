use std::slice;
use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

pub struct PointsIterator<'a> {
    iter: slice::Iter<'a, (f64, f64)>,
}

impl<'a> From<&'a [(f64, f64)]> for PointsIterator<'a> {
    fn from(data: &'a [(f64, f64)]) -> PointsIterator<'a> {
        PointsIterator { iter: data.iter() }
    }
}

impl<'a> Iterator for PointsIterator<'a> {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(p) => Some(*p),
            None => None,
        }
    }
}

/// Shape to draw a world map with the given resolution and color
pub struct Circle {
    pub color: Color,
    pub x: isize,
    pub y: isize,
    pub radius: isize,
}

impl Default for Circle {
    fn default() -> Circle {
        Circle {
            color: Color::Reset,
            x: 10,
            y: 10,
            radius: 10
        }
    }
}

#[inline(always)]
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[inline(always)]
fn draw_circle(painter: &mut Painter, color: Color, cx: isize, cy: isize, x: isize, y: isize) {
    painter.paint(max(cx + x, 0) as usize, max(cy + y, 0) as usize, color);
    painter.paint(max(cx - x, 0) as usize, max(cy + y, 0) as usize, color);
    painter.paint(max(cx + x, 0) as usize, max(cy - y, 0) as usize, color);
    painter.paint(max(cx - x, 0) as usize, max(cy - y, 0) as usize, color);
    painter.paint(max(cx + y, 0) as usize, max(cy + x, 0) as usize, color);
    painter.paint(max(cx - y, 0) as usize, max(cy + x, 0) as usize, color);
    painter.paint(max(cx + y, 0) as usize, max(cy - x, 0) as usize, color);
    painter.paint(max(cx - y, 0) as usize, max(cy - x, 0) as usize, color);
}

#[inline(always)]
fn bres_circle(painter: &mut Painter, color: Color, cx: isize, cy: isize, radius: isize) {
    let mut x = 0;
    let mut y = radius;
    let mut d = 3 - 2 * radius as isize;
    draw_circle(painter, color, cx, cy, x, y);
    while y >= x {
        x += 1;
        if d > 0 {
            y -= 1;
            d = d + 4 * (x - y) + 10;
        } else {
            d = d + 4 * x + 6;
        }
        draw_circle(painter, color, cx, cy, x, y)
    }
}

impl<'a> Shape for Circle {
    fn draw(&self, painter: &mut Painter) {
        bres_circle(painter, self.color, self.x, self.y, self.radius)
    }
}
