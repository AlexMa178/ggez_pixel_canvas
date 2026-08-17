use std::ops::Mul;

use ggez::graphics::ZIndex;
use ggez::mint::{ Point2, Vector2 };

use num_traits::{ NumCast, ToPrimitive, Zero };

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
        self.pdp.angle = angle;
        self
    }

    pub fn z(mut self, z: ZIndex) -> Self {
        self.pdp.z = z;
        self
    }

    pub fn dest(mut self, dest: impl Into<Point2<P>>) -> Self {
        self.pdp.dest = dest.into();
        self
    }

    pub fn atlas_rect(mut self, atlas_rect: impl Into<Rectangle<P>>) -> Self {
        self.pdp.atlas_section = AtlasSection::Rect { rect: atlas_rect.into() };
        self
    }

    pub fn pivot(mut self, pivot: impl Into<Point2<P>>) -> Self {
        self.pdp.pivot = pivot.into();
        self
    }

}

pub struct PDPTileBuilder<P> {
    pdp: PixelDrawParams<P>,
    tile_size: u8,
}
impl<P: Zero + NumCast + Mul<Output = P> + Copy> PDPTileBuilder<P> {

    pub fn new(tile_size: u8) -> Self {
        assert!(tile_size.is_multiple_of(2));
        Self { pdp: PixelDrawParams::default(), tile_size }
    }

    pub fn build(self) -> PixelDrawParams<P> {
        self.pdp
    }

    pub fn angle(mut self, angle: Angle<4>) -> Self {
        self.pdp.angle = angle;
        self
    }

    pub fn z(mut self, z: ZIndex) -> Self {
        self.pdp.z = z;
        self
    }

    pub fn pixel_dest(mut self, dest: impl Into<Point2<P>>) -> Self {
        self.pdp.dest = dest.into();
        self
    }

    pub fn pixel_atlas_rect(mut self, atlas_rect: impl Into<Rectangle<P>>) -> Self {
        self.pdp.atlas_section = AtlasSection::Rect { rect: atlas_rect.into() };
        self
    }

    pub fn pixel_pivot(mut self, pivot: impl Into<Point2<P>>) -> Self {
        self.pdp.pivot = pivot.into();
        self
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

    pub fn tile_pivot<T: ToPrimitive>(self, pivot: impl Into<Point2<T>>) -> Self {
        let ts = P::from(self.tile_size).unwrap();
        let half = P::from(self.tile_size / 2).unwrap();
        let Point2 { x, y } = pivot.into();
        self.pixel_pivot([
            P::from(x).unwrap() * ts + half,
            P::from(y).unwrap() * ts + half,
        ])
    }

}