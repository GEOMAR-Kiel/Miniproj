use crate::{CoordOperation, GeocentricCoordinate};

/// Molodensky-Badekas (Position Vector)
/// Geocentric: 1061
/// Geographic3D (Concatenated): 1062 (9602, 1061, 9602)
/// Geographic2D (Concatenated): 1063 (9659, 9602, 1061, 9602, 9659)
#[allow(non_snake_case)]
pub struct MolodenskyBadekasPointVector {
    M: f64,
    rX: f64,
    rY: f64,
    rZ: f64,
    XP: f64,
    YP: f64,
    ZP: f64,
    tX: f64,
    tY: f64,
    tZ: f64,
}

impl CoordOperation<GeocentricCoordinate, GeocentricCoordinate> for MolodenskyBadekasPointVector {
    #[allow(non_snake_case)]
    fn op(&self, from: GeocentricCoordinate) -> GeocentricCoordinate {
        let Xs_ = from.x() - self.XP;
        let Ys_ = from.y() - self.YP;
        let Zs_ = from.z() - self.ZP;

        let Xt = self.M * (Xs_ - Ys_ * self.rZ + Zs_ * self.rY) + self.tX + self.XP;
        let Yt = self.M * (Xs_ * self.rZ + Ys_ - Zs_ * self.rX) + self.tY + self.YP;
        let Zt = self.M * (Ys_ * self.rX - Xs_ * self.rY + Zs_) + self.tZ + self.ZP;
        GeocentricCoordinate::new(Xt, Yt, Zt)
    }
}

#[allow(non_snake_case)]
pub struct MolodenskyBadekasCoordinateFrame {
    M: f64,
    rX: f64,
    rY: f64,
    rZ: f64,
    XP: f64,
    YP: f64,
    ZP: f64,
    tX: f64,
    tY: f64,
    tZ: f64,
}
/// Molodensky-Badekas (Coordinate Frame)
/// Geocentric: 1034
/// Geographic3D (Concatenated): 1039 (9602, 1034, 9602)
/// Geographic2D (Concatenated): 9636 (9659, 9602, 1034, 9602, 9659)
impl CoordOperation<GeocentricCoordinate, GeocentricCoordinate>
    for MolodenskyBadekasCoordinateFrame
{
    #[allow(non_snake_case)]
    fn op(&self, from: GeocentricCoordinate) -> GeocentricCoordinate {
        let Xs_ = from.x() - self.XP;
        let Ys_ = from.y() - self.YP;
        let Zs_ = from.z() - self.ZP;

        let Xt = self.M * (Xs_ + Ys_ * self.rZ - Zs_ * self.rY) + self.tX + self.XP;
        let Yt = self.M * (Ys_ - Xs_ * self.rZ + Zs_ * self.rX) + self.tY + self.YP;
        let Zt = self.M * (Xs_ * self.rY - Ys_ * self.rX + Zs_) + self.tZ + self.ZP;
        GeocentricCoordinate::new(Xt, Yt, Zt)
    }
}
