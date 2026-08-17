mod builders;
pub use builders::*;

use std::ops::Sub;

use ggez::error::GameResult;
use ggez::graphics::{ Canvas, Color, DrawParam, GraphicsContext, Image, Rect, Sampler, Transform, ZIndex };
use ggez::context::{ Has, HasMut };

use ggez::mint::{ Point2, Vector2 };

use num_traits::{ NumCast, Zero };

pub use generic_discrete_2d_rotations::*;

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
    pub pivot: Point2<P>,
    pub z: ZIndex,
}
impl<P: Zero> Default for PixelDrawParams<P> {
    fn default() -> Self {
        Self {
            atlas_section: AtlasSection::All,
            dest: Point2 { x: P::zero(), y: P::zero() },
            angle: Angle::A4_0,
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

    pub fn draw<P: NumCast + Zero + Sub<Output = P> + Copy>(&mut self, image: &Image, params: PixelDrawParams<P>) {

        let ps = self.pixel_size as f32;
        let PixelDrawParams { dest, atlas_section, angle, pivot: rot_pivot, z } = params;

        let image_dim = Vector2 {
            x: P::from(image.width() ).unwrap(),
            y: P::from(image.height()).unwrap(),
        };

        let atlas_rect = match atlas_section {
            AtlasSection::All => Rectangle { pos: Point2 { x: P::zero(), y: P::zero() }, dim: image_dim },
            AtlasSection::Rect { rect } => rect,
        };

        self.canvas.draw(image, DrawParam {
            src: Rect {
                x: atlas_rect.pos.x.to_f32().unwrap() / image_dim.x.to_f32().unwrap(),
                y: atlas_rect.pos.y.to_f32().unwrap() / image_dim.y.to_f32().unwrap(),
                w: atlas_rect.dim.x.to_f32().unwrap() / image_dim.x.to_f32().unwrap(),
                h: atlas_rect.dim.y.to_f32().unwrap() / image_dim.y.to_f32().unwrap(),
            },
            color: Color::WHITE,
            transform: Transform::Values {
                dest: Point2 {
                    x: dest.x.to_f32().unwrap() * ps,
                    y: dest.y.to_f32().unwrap() * ps,
                },
                rotation: angle.to_rad(),
                scale: Vector2 { x: ps, y: ps },
                offset: Point2 {
                    x: rot_pivot.x.to_f32().unwrap() / atlas_rect.dim.x.to_f32().unwrap(),
                    y: rot_pivot.y.to_f32().unwrap() / atlas_rect.dim.y.to_f32().unwrap(),
                },
            },
            z,
        });

    }

}