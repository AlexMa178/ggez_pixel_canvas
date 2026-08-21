mod to_pixel;
pub use to_pixel::*;

use ggez::error::GameResult;
use ggez::graphics::{ Canvas, Color, DrawParam, GraphicsContext, Image, Rect as GraphicsRect, Sampler, ZIndex };
use ggez::context::{ Has, HasMut };

use glamour::{ Point2, Rect, Size2, Unit };

use num_traits::AsPrimitive;

pub use generic_discrete_2d_rotations as rotation;
use rotation::Angle;

pub trait AsPixel: Unit<Scalar: AsPrimitive<<Self::PixelType as Unit>::Scalar>> {
    type PixelType: Unit;
    const SIZE: <Self::PixelType as Unit>::Scalar;
}

pub struct PixelCanvas {
    canvas: Canvas,
    image: Image,
}
impl PixelCanvas {

    pub fn new<T: AsPixel<PixelType: Unit<Scalar: AsPrimitive<u32>>>>(gfx: &impl Has<GraphicsContext>, size: impl Into<[ T::Scalar; 2 ]>) -> Self {
        let Size2 { width, height } = Size2::<T>::from_array(size.into()).to_pixel().as_::<u32>();
        let image = Image::new_canvas_image(gfx, width, height, 1);
        let mut canvas = Canvas::from_image(gfx, image.clone(), Color::from_rgba(0, 0, 0, 0));
        canvas.set_sampler(Sampler::nearest_clamp());
        Self { canvas, image }
    }

    pub fn finish(self, gfx: &mut impl HasMut<GraphicsContext>) -> GameResult<Image> {
        self.canvas.finish(gfx)?;
        Ok(self.image)
    }

    pub fn draw<P: Unit<Scalar: AsPrimitive<f32>>>(&mut self, image: &Image, params: PixelDrawParams<P>) {

        let image_size_f = Size2::<f32>::new(image.width() as f32, image.height() as f32);

        let PixelDrawParams { dest, atlas_rect, angle, anchor, pivot, z } = params;

        let dest_f = dest.as_::<f32>();
        let pivot_f = pivot.as_::<f32>();
        let anchor_f = anchor.as_::<f32>();

        let atlas_rect_f = match atlas_rect {
            None => Rect { origin: Point2::ZERO, size: image_size_f },
            Some(rect) => rect.as_::<f32>(),
        };

        self.canvas.draw(image, DrawParam::default().src(GraphicsRect {
            x: atlas_rect_f.origin.x    / image_size_f.width,
            y: atlas_rect_f.origin.y    / image_size_f.height,
            w: atlas_rect_f.size.width  / image_size_f.width,
            h: atlas_rect_f.size.height / image_size_f.height,
        }).dest(
            dest_f + pivot_f.to_vector() - anchor_f.to_vector()
        ).rotation(
            angle.to_rad()
        ).offset([
            pivot_f.x / atlas_rect_f.size.width,
            pivot_f.y / atlas_rect_f.size.height,
        ]).z(
            z
        ));

    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct PixelDrawParams<P: Unit> {
    pub dest: Point2<P>,
    pub atlas_rect: Option<Rect<P>>,
    pub angle: Angle<4>,
    pub anchor: Point2<P>,
    pub pivot: Point2<P>,
    pub z: ZIndex,
}
impl<P: Unit> Default for PixelDrawParams<P> {
    fn default() -> Self {
        Self {
            atlas_rect: None,
            dest: Point2::ZERO,
            angle: Angle::A4_0,
            anchor: Point2::ZERO,
            pivot: Point2::ZERO,
            z: 0,
        }
    }
}
impl<P: Unit> PixelDrawParams<P> {

    pub fn dest<T: AsPixel<PixelType = P>>(self, dest: impl Into<Point2<T>>) -> Self {
        Self { dest: dest.into().to_pixel(), ..self }
    }

    pub fn atlas_rect<T: AsPixel<PixelType = P>>(self, atlas_rect: (impl Into<Point2<T>>, impl Into<Size2<T>>)) -> Self {
        Self { atlas_rect: Some(Rect::<T>::new(atlas_rect.0.into(), atlas_rect.1.into()).to_pixel()), ..self }
    }

    pub fn angle(self, angle: Angle<4>) -> Self {
        Self { angle, ..self }
    }

    pub fn anchor<T: AsPixel<PixelType = P>>(self, anchor: impl Into<Point2<T>>) -> Self {
        Self { anchor: anchor.into().to_pixel(), ..self }
    }

    pub fn pivot<T: AsPixel<PixelType = P>>(self, pivot: impl Into<Point2<T>>) -> Self {
        Self { pivot: pivot.into().to_pixel(), ..self }
    }

    pub fn z(self, z: ZIndex) -> Self {
        Self { z, ..self }
    }

}