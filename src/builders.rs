use ggez::graphics::ZIndex;
use ggez::mint::{ Point2, Vector2 };

use generic_discrete_2d_rotations::Angle;

use crate::{ Tile, Pixel, PixelsPerTile, Rectangle, AtlasSection, PixelDrawParams };

pub struct PDPBuilder {
    pdp: PixelDrawParams
}
impl PDPBuilder {

    pub fn new() -> Self {
        Self { pdp: PixelDrawParams::default() }
    }

    pub fn build(self) -> PixelDrawParams {
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

    pub fn dest(mut self, dest: impl Into<Point2<Pixel>>) -> Self {
        self.pdp.dest = dest.into();
        self
    }

    pub fn atlas_rect(mut self, atlas_rect: impl Into<Rectangle<Pixel>>) -> Self {
        self.pdp.atlas_section = AtlasSection::Rect { rect: atlas_rect.into() };
        self
    }

    pub fn rot_pivot(mut self, rot_pivot: impl Into<Point2<Pixel>>) -> Self {
        self.pdp.rot_pivot = rot_pivot.into();
        self
    }

}

pub struct PDPTileBuilder {
    pdp: PixelDrawParams,
    tile_size: PixelsPerTile,
}
impl PDPTileBuilder {

    pub fn new(tile_size: PixelsPerTile) -> Self {
        assert!(tile_size.is_multiple_of(2));
        Self { pdp: PixelDrawParams::default(), tile_size }
    }

    pub fn build(self) -> PixelDrawParams {
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

    pub fn pixel_dest(mut self, dest: impl Into<Point2<Pixel>>) -> Self {
        self.pdp.dest = dest.into();
        self
    }

    pub fn tile_dest(self, dest: impl Into<Point2<Tile>>) -> Self {
        let ts = self.tile_size as Pixel;
        let Point2 { x, y } = dest.into();
        self.pixel_dest([ x as Pixel * ts, y as Pixel * ts ])
    }

    pub fn pixel_atlas_rect(mut self, atlas_rect: impl Into<Rectangle<Pixel>>) -> Self {
        self.pdp.atlas_section = AtlasSection::Rect { rect: atlas_rect.into() };
        self
    }

    pub fn tile_atlas_rect(self, atlas_rect: impl Into<Rectangle<Tile>>) -> Self {
        let ts = self.tile_size as Pixel;
        let Rectangle { pos: Point2 { x, y }, dim: Vector2 { x: w, y: h } } = atlas_rect.into();
        self.pixel_atlas_rect([ x as Pixel * ts, y as Pixel * ts, w as Pixel * ts, h as Pixel * ts ])
    }

    pub fn pixel_rot_pivot(mut self, rot_pivot: impl Into<Point2<Pixel>>) -> Self {
        self.pdp.rot_pivot = rot_pivot.into();
        self
    }

    pub fn tile_rot_pivot(self, rot_pivot: impl Into<Point2<Tile>>) -> Self {
        let ts = self.tile_size as Pixel;
        let half = ts / 2;
        let Point2 { x, y } = rot_pivot.into();
        self.pixel_rot_pivot([ x as Pixel * ts + half, y as Pixel * ts + half ])
    }

}