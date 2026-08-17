mod builders;
pub use builders::*;

use ggez::error::GameResult;
use ggez::graphics::{ Canvas, Color, DrawParam, GraphicsContext, Image, Rect, Sampler, Transform, ZIndex };
use ggez::context::{ Has, HasMut };

use ggez::mint::{ Point2, Vector2 };

use num_traits::{ NumCast, Zero };

pub use generic_discrete_2d_rotations as rotation;
use rotation::Angle;

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

pub enum AtlasSection<P> {
    All,
    Rect { rect: Rectangle<P> },
}

pub struct PixelDrawParams<P> {
    pub dest: Point2<P>,
    pub atlas_section: AtlasSection<P>,
    pub angle: Angle<4>,
    pub anchor: Point2<P>,
    pub pivot: Point2<P>,
    pub z: ZIndex,
}
impl<P: Zero> Default for PixelDrawParams<P> {
    fn default() -> Self {
        Self {
            atlas_section: AtlasSection::All,
            dest: Point2 { x: P::zero(), y: P::zero() },
            angle: Angle::A4_0,
            anchor: Point2 { x: P::zero(), y: P::zero() },
            pivot: Point2 { x: P::zero(), y: P::zero() },
            z: 0,
        }
    }
}

pub struct PixelCanvas {
    canvas: Canvas,
    pixel_size: u8,
}
impl PixelCanvas {

    pub fn new_frame(gfx: &impl Has<GraphicsContext>, pixel_size: u8) -> Self {
        let mut canvas = Canvas::from_frame(gfx, Color::BLACK);
        canvas.set_sampler(Sampler::nearest_clamp());
        Self { canvas, pixel_size }
    }

    pub fn finish(self, gfx: &mut impl HasMut<GraphicsContext>) -> GameResult {
        self.canvas.finish(gfx)
    }

    pub fn draw<P: NumCast>(&mut self, image: &Image, params: PixelDrawParams<P>) {

        let f = |p: P| p.to_f32().unwrap();

        let image_dim = Vector2 {
            x: image.width()  as f32,
            y: image.height() as f32,
        };

        let ps = self.pixel_size as f32;

        let PixelDrawParams { dest, atlas_section, angle, anchor, pivot, z } = params;

        let atlas_rect = match atlas_section {
            AtlasSection::All => Rectangle { pos: Point2 { x: 0., y: 0. }, dim: image_dim },
            AtlasSection::Rect { rect } => Rectangle {
                pos: Point2  { x: f(rect.pos.x), y: f(rect.pos.y) },
                dim: Vector2 { x: f(rect.dim.x), y: f(rect.dim.y) },
            },
        };

        let pivot = Point2 {
            x: f(pivot.x),
            y: f(pivot.y),
        };

        self.canvas.draw(image, DrawParam {
            src: Rect {
                x: atlas_rect.pos.x / image_dim.x,
                y: atlas_rect.pos.y / image_dim.y,
                w: atlas_rect.dim.x / image_dim.x,
                h: atlas_rect.dim.y / image_dim.y,
            },
            color: Color::WHITE,
            transform: Transform::Values {
                dest: Point2 {
                    x: (f(dest.x) + pivot.x - f(anchor.x)) * ps,
                    y: (f(dest.y) + pivot.y - f(anchor.y)) * ps,
                },
                rotation: angle.to_rad(),
                scale: Vector2 { x: ps, y: ps },
                offset: Point2 {
                    x: pivot.x / atlas_rect.dim.x,
                    y: pivot.y / atlas_rect.dim.y,
                },
            },
            z,
        });

    }

}