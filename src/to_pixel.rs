use glamour::{ Box2, Box3, Point2, Point3, Point4, Rect, Size2, Size3, Vector2, Vector3, Vector4 };

use crate::AsPixel;

pub trait ToPixel {
    type Output;
    fn to_pixel(self) -> Self::Output;
}

macro_rules! to_pixel_with_map {
    ( $type_name:ident ) => {
        impl<T: AsPixel> ToPixel for $type_name<T> {
            type Output = $type_name<T::PixelType>;
            fn to_pixel(self) -> Self::Output {
                self.as_().map(|v| v * T::SIZE)
            }
        }
    };
}
to_pixel_with_map!(Point2);
to_pixel_with_map!(Point3);
to_pixel_with_map!(Point4);
to_pixel_with_map!(Vector2);
to_pixel_with_map!(Vector3);
to_pixel_with_map!(Vector4);
to_pixel_with_map!(Size2);
to_pixel_with_map!(Size3);

macro_rules! to_pixel_with_array_of_points {
    ( $type_name:ident ) => {
        impl<T: AsPixel> ToPixel for $type_name<T> {
            type Output = $type_name<T::PixelType>;
            fn to_pixel(self) -> Self::Output {
                $type_name::from_array(self.to_array().map(|p| p.to_pixel()))
            }
        }
    };
}
to_pixel_with_array_of_points!(Box2);
to_pixel_with_array_of_points!(Box3);

impl<T: AsPixel> ToPixel for Rect<T> {
    type Output = Rect<T::PixelType>;
    fn to_pixel(self) -> Self::Output {
        let (o, s) = self.to_tuple();
        Rect::new(o.to_pixel(), s.to_pixel())
    }
}