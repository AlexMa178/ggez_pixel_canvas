mod builders;
pub use builders::*;

use ggez::error::GameResult;
use ggez::graphics::{ Canvas, Color, DrawParam, GraphicsContext, Image, Rect, Sampler, Transform, ZIndex };
use ggez::context::{ Has, HasMut };

use ggez::mint::{ Point2, Vector2 };

pub use generic_discrete_2d_rotations::*;

pub type Tile = u8;
pub type Pixel = u32;
pub type ScreenPixel = f32;
pub type UV = f32;

pub type PixelsPerTile = u8;
pub type ScreenPixelsPerPixel = u8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rectangle<T> {
    pos: Point2<T>, dim: Vector2<T>
}
impl<T, V: Into<[ T; 4 ]>> From<V> for Rectangle<T> {
    fn from(value: V) -> Self {
        let [ x, y, w, h ] = value.into();
        Rectangle { pos: Point2 { x, y }, dim: Vector2 { x: w, y: h } }
    }
}

pub enum AtlasSection {
    All,
    Rect { rect: Rectangle<Pixel> },
}

pub struct PixelDrawParams {
    dest: Point2<Pixel>,
    atlas_section: AtlasSection,
    angle: Angle<4>,
    rot_pivot: Point2<Pixel>,
    z: ZIndex,
}
impl Default for PixelDrawParams {
    fn default() -> Self {
        Self {
            atlas_section: AtlasSection::All,
            dest: Point2 { x: 0, y: 0 },
            angle: Angle::A4_0,
            rot_pivot: Point2 { x: 0, y: 0 },
            z: 0,
        }
    }
}

pub struct PixelCanvas {
    canvas: Canvas,
    pixel_size: ScreenPixelsPerPixel,
}
impl PixelCanvas {

    pub fn new_frame(gfx: &impl Has<GraphicsContext>, pixel_size: ScreenPixelsPerPixel) -> Self {
        let mut canvas = Canvas::from_frame(gfx, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());
        Self { canvas, pixel_size }
    }

    pub fn finish(self, gfx: &mut impl HasMut<GraphicsContext>) -> GameResult {
        self.canvas.finish(gfx)
    }

    pub fn draw(&mut self, image: Image, params: PixelDrawParams) {

        let ps = self.pixel_size as f32;
        let PixelDrawParams { dest, atlas_section, angle, rot_pivot, z } = params;

        let image_dim = Vector2 { x: image.width(), y: image.height() };

        let atlas_rect = match atlas_section {
            AtlasSection::All => Rectangle { pos: Point2 { x: 0, y: 0 }, dim: image_dim },
            AtlasSection::Rect { rect } => rect,
        };

        self.canvas.draw(&image, DrawParam {
            src: Rect {
                x: atlas_rect.pos.x as f32 / image_dim.x as f32,
                y: atlas_rect.pos.y as f32 / image_dim.y as f32,
                w: atlas_rect.dim.x as f32 / image_dim.x as f32,
                h: atlas_rect.dim.y as f32 / image_dim.y as f32,
            },
            color: Color::WHITE,
            transform: Transform::Values {
                dest: Point2 {
                    x: (dest.x + rot_pivot.x) as f32 * ps,
                    y: (dest.y + rot_pivot.y) as f32 * ps,
                },
                rotation: angle.to_rad(),
                scale: Vector2 { x: ps, y: ps },
                offset: Point2 {
                    x: rot_pivot.x as f32 / atlas_rect.dim.x as f32,
                    y: rot_pivot.y as f32 / atlas_rect.dim.y as f32,
                },
            },
            z,
        });

    }

}