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

/// Helmert 15-Parameter (Position Vector, Time-dependent)
/// Geocentric: 1053
/// Geographic3D (Concatenated): 1055 (9602, 1053, 9602)
/// Geographic2D (Concatenated): 1054 (9659, 9602, 1053, 9602, 9659)
#[allow(non_snake_case)]
pub struct HelmertPositionVectorTimeDependent {
    rX: f64,
    rY: f64,
    rZ: f64,
    tX: f64,
    tY: f64,
    tZ: f64,
    dS: f64,
    drX: f64,
    drY: f64,
    drZ: f64,
    dtX: f64,
    dtY: f64,
    dtZ: f64,
    ddS: f64,
    reference_epoch: f64,
}
impl HelmertPositionVectorTimeDependent {
    pub fn at_epoch(&self, epoch: f64) -> HelmertPositionVector {
        let dt = epoch - self.reference_epoch;
        HelmertPositionVector {
            M: 1.0 + self.dS + self.ddS * dt,
            rX: self.rX + self.drX * dt,
            rY: self.rY + self.drY * dt,
            rZ: self.rZ + self.drZ * dt,
            tX: self.tX + self.dtX * dt,
            tY: self.tY + self.dtY * dt,
            tZ: self.tZ + self.dtZ * dt,
        }
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
/// Helmert 15-Parameter (Coordinate Frame, Time-dependent)
/// Geocentric: 1056
/// Geographic3D (Concatenated): 1058 (9602, 1056, 9602)
/// Geographic2D (Concatenated): 1057 (9659, 9602, 1056, 9602, 9659)
#[allow(non_snake_case)]
pub struct HelmertCoordinateFrameTimeDependent {
    rX: f64,
    rY: f64,
    rZ: f64,
    tX: f64,
    tY: f64,
    tZ: f64,
    dS: f64,
    drX: f64,
    drY: f64,
    drZ: f64,
    dtX: f64,
    dtY: f64,
    dtZ: f64,
    ddS: f64,
    reference_epoch: f64,
}
impl HelmertCoordinateFrameTimeDependent {
    pub fn at_epoch(&self, epoch: f64) -> HelmertCoordinateFrame {
        let dt = epoch - self.reference_epoch;
        HelmertCoordinateFrame {
            M: 1.0 + self.dS + self.ddS * dt,
            rX: self.rX + self.drX * dt,
            rY: self.rY + self.drY * dt,
            rZ: self.rZ + self.drZ * dt,
            tX: self.tX + self.dtX * dt,
            tY: self.tY * self.dtY * dt,
            tZ: self.tZ + self.dtZ * dt,
        }
    }
}
