mod builders;
pub use builders::*;

use ggez::error::GameResult;
use ggez::graphics::{ Canvas, Color, DrawParam, GraphicsContext, Image, Rect as GraphicsRect, Sampler, ZIndex };
use ggez::context::{ Has, HasMut };

use glamour::{ Point2, Rect, Size2, Unit };

use num_traits::AsPrimitive;

pub use generic_discrete_2d_rotations as rotation;
use rotation::Angle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtlasSection<P: Unit> {
    All,
    Rect { rect: Rect<P> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PixelDrawParams<P: Unit> {
    pub dest: Point2<P>,
    pub atlas_section: AtlasSection<P>,
    pub angle: Angle<4>,
    pub anchor: Point2<P>,
    pub pivot: Point2<P>,
    pub z: ZIndex,
}
impl<P: Unit> Default for PixelDrawParams<P> {
    fn default() -> Self {
        Self {
            atlas_section: AtlasSection::All,
            dest: Point2::ZERO,
            angle: Angle::A4_0,
            anchor: Point2::ZERO,
            pivot: Point2::ZERO,
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

    pub fn draw<P: Unit<Scalar: AsPrimitive<f32>>>(&mut self, image: &Image, params: PixelDrawParams<P>) {

        let image_size_f = Size2::<u32> {
            width: image.width(),
            height: image.height(),
        }.as_::<f32>();

        let ps = self.pixel_size as f32;

        let PixelDrawParams { dest, atlas_section, angle, anchor, pivot, z } = params;

        let dest_f = dest.as_::<f32>();
        let pivot_f = pivot.as_::<f32>();
        let anchor_f = anchor.as_::<f32>();

        let atlas_rect_f = match atlas_section {
            AtlasSection::All => Rect { origin: Point2::ZERO, size: image_size_f },
            AtlasSection::Rect { rect } => rect.as_::<f32>(),
        };

        self.canvas.draw(image, DrawParam::default().src(GraphicsRect {
            x: atlas_rect_f.origin.x    / image_size_f.width,
            y: atlas_rect_f.origin.y    / image_size_f.height,
            w: atlas_rect_f.size.width  / image_size_f.width,
            h: atlas_rect_f.size.height / image_size_f.height,
        }).dest([
            (dest_f.x + pivot_f.x - anchor_f.x) * ps,
            (dest_f.y + pivot_f.y - anchor_f.y) * ps,
        ]).rotation(
            angle.to_rad()
        ).scale(
            [ ps, ps ]
        ).offset([
            pivot_f.x / atlas_rect_f.size.width,
            pivot_f.y / atlas_rect_f.size.height,
        ]).z(
            z
        ));

    }

}