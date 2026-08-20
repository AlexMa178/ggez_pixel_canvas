use ggez::error::GameResult;
use ggez::graphics::{ Canvas, Color, DrawParam, GraphicsContext, Image, Rect as GraphicsRect, Sampler, ZIndex };
use ggez::context::{ Has, HasMut };

use glamour::{ Point2, Rect, Scalar, Size2, Unit };

use num_traits::AsPrimitive;

pub use generic_discrete_2d_rotations as rotation;
use rotation::Angle;

pub trait AsPixel<P: Unit>: Unit<Scalar: AsPrimitive<P::Scalar>> {
    const SIZE: P::Scalar;
}

pub struct PixelCanvas {
    canvas: Canvas,
    image: Image,
}
impl PixelCanvas {

    pub fn new<T: AsPixel<impl Unit<Scalar: AsPrimitive<u32>>>>(gfx: &impl Has<GraphicsContext>, size: impl Into<[ T::Scalar; 2 ]>) -> Self {
        let [ width, height ] = size.into();
        let image = Image::new_canvas_image(gfx, Scalar::as_(Scalar::as_(width) * T::SIZE), Scalar::as_(Scalar::as_(height) * T::SIZE), 1);
        let mut canvas = Canvas::from_image(gfx, image.clone(), Color::from_rgba(0, 0, 0, 0));
        canvas.set_sampler(Sampler::nearest_clamp());
        Self { canvas, image }
    }

    pub fn finish(self, gfx: &mut impl HasMut<GraphicsContext>) -> GameResult<Image> {
        self.canvas.finish(gfx)?;
        Ok(self.image)
    }

    pub fn draw<P: Unit<Scalar: AsPrimitive<f32>>>(&mut self, image: &Image, params: PixelDrawParams<P>) {

        let image_size_f = Size2::<u32> {
            width: image.width(),
            height: image.height(),
        }.as_::<f32>();

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
impl<P: Unit> PixelDrawParams<P> {

    pub fn dest<T: AsPixel<P>>(self, dest: impl Into<[ T::Scalar; 2 ]>) -> Self {
        let [ x, y ] = dest.into();
        Self { dest: Point2::new(
            Scalar::as_(x) * T::SIZE,
            Scalar::as_(y) * T::SIZE,
        ), ..self }
    }

    pub fn atlas_rect<T: AsPixel<P>>(self, atlas_rect: (impl Into<[ T::Scalar; 2 ]>, impl Into<[ T::Scalar; 2 ]>)) -> Self {
        let (into_origin, into_size) = atlas_rect;
        let ([ x, y ], [ w, h ]) = (into_origin.into(), into_size.into());
        Self { atlas_section: AtlasSection::Rect { rect: Rect::new(
            Point2::new(
                Scalar::as_(x) * T::SIZE,
                Scalar::as_(y) * T::SIZE,
            ),
            Size2::new(
                Scalar::as_(w) * T::SIZE,
                Scalar::as_(h) * T::SIZE,
            ),
        ) }, ..self }
    }

    pub fn angle(self, angle: Angle<4>) -> Self {
        Self { angle, ..self }
    }

    pub fn anchor<T: AsPixel<P>>(self, anchor: impl Into<[ T::Scalar; 2 ]>) -> Self {
        let [ x, y ] = anchor.into();
        Self { anchor: Point2::new(
            Scalar::as_(x) * T::SIZE,
            Scalar::as_(y) * T::SIZE,
        ), ..self }
    }

    pub fn pivot<T: AsPixel<P>>(self, pivot: impl Into<[ T::Scalar; 2 ]>) -> Self {
        let [ x, y ] = pivot.into();
        Self { pivot: Point2::new(
            Scalar::as_(x) * T::SIZE,
            Scalar::as_(y) * T::SIZE,
        ), ..self }
    }

    pub fn z(self, z: ZIndex) -> Self {
        Self { z, ..self }
    }

}