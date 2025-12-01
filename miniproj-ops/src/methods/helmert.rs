use crate::{CoordOperation, GeocentricCoordinate};


/// Helmert 7-Parameter (Position Vector)
/// Geocentric: 1033
/// Geographic3D (Concatenated): 1037 (9602, 1033, 9602)
/// Geographic2D (Concatenated): 9606 (9659, 9602, 1033, 9602, 9659)
#[allow(non_snake_case)]
pub struct HelmertPositionVector {
    M: f64,
    rX: f64,
    rY: f64,
    rZ: f64,
    tX: f64,
    tY: f64,
    tZ: f64,
}

impl CoordOperation<GeocentricCoordinate, GeocentricCoordinate> for HelmertPositionVector {
    #[allow(non_snake_case)]
    fn op(&self, from: GeocentricCoordinate) -> GeocentricCoordinate {
        let Xs = from.x();
        let Ys = from.y();
        let Zs = from.z();

        let Xt = self.M * (Xs - Ys * self.rZ + Zs * self.rY) + self.tX;
        let Yt = self.M * (Xs * self.rZ + Ys - Zs * self.rX) + self.tY;
        let Zt = self.M * (Ys * self.rX - Xs * self.rY + Zs) + self.tZ;
        GeocentricCoordinate::new(Xt, Yt, Zt)
    }
}
/// Helmert 7-Parameter (Coordinate Frame)
/// Geocentric: 1032
/// Geographic3D (Concatenated): 1038 (9602, 1032, 9602)
/// Geographic2D (Concatenated): 9607 (9659, 9602, 1032, 9602, 9659)
#[allow(non_snake_case)]
pub struct HelmertCoordinateFrame {
    M: f64,
    rX: f64,
    rY: f64,
    rZ: f64,
    tX: f64,
    tY: f64,
    tZ: f64,
}

impl CoordOperation<GeocentricCoordinate, GeocentricCoordinate> for HelmertCoordinateFrame {
    #[allow(non_snake_case)]
    fn op(&self, from: GeocentricCoordinate) -> GeocentricCoordinate {
        let Xs = from.x();
        let Ys = from.y();
        let Zs = from.z();

        let Xt = self.M * (Xs + Ys * self.rZ - Zs * self.rY) + self.tX;
        let Yt = self.M * (Ys - Xs * self.rZ + Zs * self.rX) + self.tY;
        let Zt = self.M * (Xs * self.rY - Ys * self.rX + Zs) + self.tZ;
        GeocentricCoordinate::new(Xt, Yt, Zt)
    }
}
