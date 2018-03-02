use allegro::{Bitmap, Color};
use allegro_font::FontDrawing;
use allegro_primitives::{PrimType, Vertex};
use std::u8;

pub fn map(p: &::Platform, map: &::MapDef, camera: ::Pos) {
    p.core.hold_bitmap_drawing(true);
    // let y_step = tiles.tile_height/2.0;
    for (x, row) in map.iter().enumerate() {
        for (y, id) in row.iter().enumerate() {
            ::assets::draw_tile(&p.core, *id, xpos(camera.0, x as i32), ypos(camera.1, x as i32, y as i32), None);
        }
    }
    p.core.hold_bitmap_drawing(false);
}

pub fn tile_highlight(p: &::Platform, camera: ::Pos, tile: ::Pos) {
    let color = Color::from_rgb(u8::MAX, u8::MAX, 0);
    let vertices: Vec<Vertex> = tile_vertices(camera, tile.0, tile.1).into_iter()
        .map(|(x,y)| Vertex{x: x as f32, y: y as f32, z: 0.0, u: 0.0, v: 0.0, color: color})
        .collect();
    // p.primitives_addon.draw_rectangle(x1, y1, x2, y2, , 2.0);
    p.primitives_addon.draw_prim(&vertices.as_slice(), None as Option<&Bitmap>, 0, vertices.len() as u32, PrimType::LineStrip);
}

pub fn text<'a>(p: &::Platform, color: ::allegro::Color, pos: ::Pos, align: ::allegro_font::FontAlign, s: &'a str) {
    let font = ::allegro_font::Font::new_builtin(&p.font_addon).unwrap();
    p.core.draw_text(&font, color, pos.0 as f32, pos.1 as f32, align, s);
}

pub fn clicked_tile(map: &::MapDef, camera: ::Pos, mouse: ::Pos) -> Option<(i32, i32)> {
    // In lieu of proper hexagon math, let's just brute-force it.
    for (x, row) in map.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let vertices = tile_vertices(camera, x as i32, y as i32);
            // main body
            if mouse.0 > vertices[0].0 && mouse.0 < vertices[3].0 && mouse.1 > vertices[0].1 && mouse.1 < vertices[3].1 {
                return Some((x as i32, y as i32))
            }
        }
    }
    None
}

/// Calculate all of the (x,y) coordinates for the tile.
fn tile_vertices(camera: ::Pos, x: i32, y: i32) -> Vec<(i32, i32)> {
    let x1 = xpos(camera.0, x);
    let x2 = x1 + ::assets::tile_width();
    let y1 = ypos(camera.1, x, y) + ::assets::tile_height()/2 + 3;
    let y2 = y1 + ::assets::tile_height();
    // p.primitives_addon.draw_rectangle(x1, y1, x2, y2, , 2.0);
    vec![
        (x1 + ::assets::tile_width()/4, y1),
        (x2 - ::assets::tile_width()/4, y1),
        (x2, y1 + ::assets::tile_height()/2),
        (x2 - ::assets::tile_width()/4, y2),
        (x1 + ::assets::tile_width()/4, y2),
        (x1, y1 + ::assets::tile_height()/2),
        (x1 + ::assets::tile_width()/4, y1),
    ]
}

fn xpos(x_start: i32, x: i32) -> i32 {
    // TODO: make x_step lazy_static?
    let x_step = (::assets::tile_width()/4) * 3;
    x_start + (x*x_step)
}

fn ypos(y_start: i32, x: i32, y: i32) -> i32 {
    // NOTE: subtracting half the tile's height at the end seems unnecessary; might be some weird cropping in the image?
    y_start + y*(::assets::tile_height()) + if x%2==1 { ::assets::tile_height()/2 } else { 0 } - ::assets::tile_height()/2
}
