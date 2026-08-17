use std::ops::Mul;

use ggez::graphics::ZIndex;
use ggez::mint::{ Point2, Vector2 };

use num_traits::{ Num, NumCast, ToPrimitive, Zero };

use generic_discrete_2d_rotations::Angle;

use crate::{ Rectangle, AtlasSection, PixelDrawParams };

pub struct PDPBuilder<P> {
    pdp: PixelDrawParams<P>
}
impl<P: Zero> PDPBuilder<P> {

    pub fn new() -> Self {
        Self { pdp: PixelDrawParams::default() }
    }

    pub fn build(self) -> PixelDrawParams<P> {
        self.pdp
    }

    pub fn angle(mut self, angle: Angle<4>) -> Self {
        self.pdp.angle = angle; self
    }

    pub fn z(mut self, z: ZIndex) -> Self {
        self.pdp.z = z; self
    }

    pub fn dest(mut self, dest: impl Into<Point2<P>>) -> Self {
        self.pdp.dest = dest.into(); self
    }

    pub fn atlas_rect(mut self, atlas_rect: impl Into<Rectangle<P>>) -> Self {
        self.pdp.atlas_section = AtlasSection::Rect { rect: atlas_rect.into() }; self
    }

    pub fn anchor(mut self, anchor: impl Into<Point2<P>>) -> Self {
        self.pdp.anchor = anchor.into(); self
    }

    pub fn pivot(mut self, pivot: impl Into<Point2<P>>) -> Self {
        self.pdp.pivot = pivot.into(); self
    }

}

pub struct PDPTileBuilder<P> {
    builder: PDPBuilder<P>,
    tile_size: u8,
}
impl<P: Zero + NumCast + Mul<Output = P> + Copy> PDPTileBuilder<P> {

    pub fn new(tile_size: u8) -> Self {
        Self { builder: PDPBuilder::new(), tile_size }
    }

    pub fn build(self) -> PixelDrawParams<P> {
        self.builder.build()
    }

    pub fn angle(self, angle: Angle<4>) -> Self {
        Self { builder: self.builder.angle(angle), ..self }
    }

    pub fn z(self, z: ZIndex) -> Self {
        Self { builder: self.builder.z(z), ..self }
    }

    pub fn pixel_dest(self, dest: impl Into<Point2<P>>) -> Self {
        Self { builder: self.builder.dest(dest), ..self }
    }

    pub fn pixel_atlas_rect(self, atlas_rect: impl Into<Rectangle<P>>) -> Self {
        Self { builder: self.builder.atlas_rect(atlas_rect), ..self }
    }

    pub fn pixel_anchor(self, anchor: impl Into<Point2<P>>) -> Self {
        Self { builder: self.builder.anchor(anchor), ..self }
    }

    pub fn pixel_pivot(self, pivot: impl Into<Point2<P>>) -> Self {
        Self { builder: self.builder.pivot(pivot), ..self }
    }

    pub fn tile_dest<T: ToPrimitive>(self, dest: impl Into<Point2<T>>) -> Self {
        let ts = P::from(self.tile_size).unwrap();
        let Point2 { x, y } = dest.into();
        self.pixel_dest([
            P::from(x).unwrap() * ts,
            P::from(y).unwrap() * ts,
        ])
    }

    pub fn tile_atlas_rect<T: ToPrimitive>(self, atlas_rect: impl Into<Rectangle<T>>) -> Self {
        let ts = P::from(self.tile_size).unwrap();
        let Rectangle { pos: Point2 { x, y }, dim: Vector2 { x: w, y: h } } = atlas_rect.into();
        self.pixel_atlas_rect([
            P::from(x).unwrap() * ts,
            P::from(y).unwrap() * ts,
            P::from(w).unwrap() * ts,
            P::from(h).unwrap() * ts,
        ])
    }

    pub fn tile_anchor<T: ToPrimitive>(self, anchor: impl Into<Point2<T>>) -> Self {
        let ts = P::from(self.tile_size).unwrap();
        let Point2 { x, y } = anchor.into();
        self.pixel_anchor([
            P::from(x).unwrap() * ts,
            P::from(y).unwrap() * ts,
        ])
    }

    pub fn tile_pivot<T: ToPrimitive>(self, pivot: impl Into<Point2<T>>, x_odd: bool, y_odd: bool) -> Self {
        assert!(!(x_odd || y_odd) || self.tile_size.is_multiple_of(2));
        let ts = P::from(self.tile_size).unwrap();
        let half = P::from(self.tile_size / 2).unwrap();
        let Point2 { x, y } = pivot.into();
        self.pixel_pivot([
            P::from(x).unwrap() * ts + if x_odd { half } else { P::zero() },
            P::from(y).unwrap() * ts + if y_odd { half } else { P::zero() },
        ])
    }

    pub fn tile_atlas_rect_and_pivot<T: ToPrimitive + Copy + Num>(self, atlas_rect: impl Into<Rectangle<T>>, pivot: impl Into<Point2<T>>) -> Self {
        let rect = atlas_rect.into();
        let two = T::one() + T::one();
        let x_odd = (rect.dim.x % two).is_one();
        let y_odd = (rect.dim.y % two).is_one();
        self.tile_atlas_rect(rect).tile_pivot(pivot, x_odd, y_odd)
    }

}