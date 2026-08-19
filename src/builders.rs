use std::marker::PhantomData;

use ggez::graphics::ZIndex;

use glamour::{ Point2, Rect, Unit };

use generic_discrete_2d_rotations::Angle;
use num_traits::{ AsPrimitive, ConstZero, NumCast };

use crate::{ AtlasSection, PixelDrawParams };

pub struct PDPBuilder<P: Unit> {
    pdp: PixelDrawParams<P>
}
impl<P: Unit> PDPBuilder<P> {

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

    pub fn dest(mut self, dest: impl Into<[ P::Scalar; 2 ]>) -> Self {
        self.pdp.dest = Point2::from_array(dest.into()); self
    }

    pub fn atlas_rect(mut self, atlas_rect: impl Into<[ P::Scalar; 4 ]>) -> Self {
        let [ x, y, w, h ] = atlas_rect.into();
        self.pdp.atlas_section = AtlasSection::Rect { rect: Rect::from_origin_and_size([ x, y ], [ w, h ]) }; self
    }

    pub fn anchor(mut self, anchor: impl Into<[ P::Scalar; 2 ]>) -> Self {
        self.pdp.anchor = Point2::from_array(anchor.into()); self
    }

    pub fn pivot(mut self, pivot: impl Into<[ P::Scalar; 2 ]>) -> Self {
        self.pdp.pivot = Point2::from_array(pivot.into()); self
    }

}

pub struct PDPTileBuilder<P: Unit, T: Unit<Scalar: AsPrimitive<P::Scalar>>> {
    builder: PDPBuilder<P>,
    tile_size: P::Scalar,
    phantom: PhantomData<T>,
}
impl<P: Unit, T: Unit<Scalar: AsPrimitive<P::Scalar>>> PDPTileBuilder<P, T> {

    pub fn new(tile_size: P::Scalar) -> Self {
        Self { builder: PDPBuilder::new(), tile_size, phantom: PhantomData }
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

    pub fn pixel_dest(self, dest: impl Into<[ P::Scalar; 2 ]>) -> Self {
        Self { builder: self.builder.dest(dest), ..self }
    }

    pub fn pixel_atlas_rect(self, atlas_rect: impl Into<[ P::Scalar; 4 ]>) -> Self {
        Self { builder: self.builder.atlas_rect(atlas_rect), ..self }
    }

    pub fn pixel_anchor(self, anchor: impl Into<[ P::Scalar; 2 ]>) -> Self {
        Self { builder: self.builder.anchor(anchor), ..self }
    }

    pub fn pixel_pivot(self, pivot: impl Into<[ P::Scalar; 2 ]>) -> Self {
        Self { builder: self.builder.pivot(pivot), ..self }
    }

    pub fn tile_dest(self, dest: impl Into<[ T::Scalar; 2 ]>) -> Self {
        let ts = self.tile_size;
        let [ x, y ] = dest.into();
        self.pixel_dest([
            x.as_() * ts,
            y.as_() * ts,
        ])
    }

    pub fn tile_atlas_rect(self, atlas_rect: impl Into<[ T::Scalar; 4 ]>) -> Self {
        let ts = self.tile_size;
        let [ x, y, w, h ] = atlas_rect.into();
        self.pixel_atlas_rect([
            x.as_() * ts,
            y.as_() * ts,
            w.as_() * ts,
            h.as_() * ts,
        ])
    }

    pub fn tile_anchor(self, anchor: impl Into<[ T::Scalar; 2 ]>) -> Self {
        let ts = self.tile_size;
        let [ x, y ] = anchor.into();
        self.pixel_anchor([
            x.as_() * ts,
            y.as_() * ts,
        ])
    }

    pub fn tile_pivot(self, pivot: impl Into<[ T::Scalar; 2 ]>, x_odd: bool, y_odd: bool) -> Self {
        let ts = self.tile_size;
        let half = ts / NumCast::from(2).unwrap();
        let [ x, y ] = pivot.into();
        self.pixel_pivot([
            x.as_() * ts + if x_odd { half } else { ConstZero::ZERO },
            y.as_() * ts + if y_odd { half } else { ConstZero::ZERO },
        ])
    }

}