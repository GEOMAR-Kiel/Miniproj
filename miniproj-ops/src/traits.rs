//This file is licensed under EUPL v1.2

use std::marker::PhantomData;

use crate::ellipsoid::Ellipsoid;

/// Two-dimensional coordinate operation
pub trait Projection: Send + Sync {
    ///Converts from a coordinate in the target coordinate system to lon/lat in EPSG 4326 in radians
    fn projected_to_rad(&self, x: f64, y: f64) -> (f64, f64);

    ///Converts from a geographic coordinate in radians to a projected coordinate `(x, y)`, usually in meters.
    fn rad_to_projected(&self, lon: f64, lat: f64) -> (f64, f64);

    ///Converts from projected coordinates to geographic coordinates `(longitude, latitude)` in decimal degrees.
    fn projected_to_deg(&self, x: f64, y: f64) -> (f64, f64) {
        let tmp = self.projected_to_rad(x, y);
        (tmp.0.to_degrees(), tmp.1.to_degrees())
    }

    ///Converts from a geographic coordinate in degrees to a projected coordinate `(x, y)`, usually in meters.
    fn deg_to_projected(&self, lon: f64, lat: f64) -> (f64, f64) {
        self.rad_to_projected(lon.to_radians(), lat.to_radians())
    }
}

pub trait PseudoSerialize {
    fn to_constructed(&self) -> String;
}

pub trait DbContstruct {
    fn from_database_params(params: &[(u32, f64)], ellipsoid: &Ellipsoid) -> Self;
}

pub trait GetterContstruct: Sized {
    fn with_db_getter<G>(getter: G, ellipsoid: &Ellipsoid) -> Option<Self>
    where
        G: FnMut(u32) -> Option<f64>;
}

pub struct ProjectedCoordinate {
    easting: f64,
    northing: f64,
}
impl ProjectedCoordinate {
    pub fn new(easting: f64, northing: f64) -> Self {
        Self { easting, northing }
    }

    pub fn easting(&self) -> f64 {
        self.easting
    }

    pub fn northing(&self) -> f64 {
        self.northing
    }
}

pub struct ProjectedCoordinateUserVertical {
    easting: f64,
    northing: f64,
    vertical: f64,
}
impl ProjectedCoordinateUserVertical {
    pub fn new(easting: f64, northing: f64, vertical: f64) -> Self {
        Self { easting, northing, vertical }
    }

    pub fn easting(&self) -> f64 {
        self.easting
    }

    pub fn northing(&self) -> f64 {
        self.northing
    }

    pub fn vertical(&self) -> f64 {
        self.vertical
    }
}

pub struct Geographic2DCoordinate {
    longitude: f64,
    latitude: f64,
}
impl Geographic2DCoordinate {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self { longitude, latitude }
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }
}

pub struct Geographic3DCoordinate {
    longitude: f64,
    latitude: f64,
    ell_height: f64,
}
impl Geographic3DCoordinate {
    pub fn new(longitude: f64, latitude: f64, ell_height: f64) -> Self {
        Self { longitude, latitude, ell_height }
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn ellipsoid_height(&self) -> f64 {
        self.ell_height
    }
}

pub struct GeocentricCoordinate {
    x: f64,
    y: f64,
    z: f64,
}
impl GeocentricCoordinate {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

pub trait CoordOperation<F, T> {
    fn op(&self, from: F) -> T;
}

pub struct ConcatenatedCoordOp<A: CoordOperation<F, I>, B: CoordOperation<I, T>, F, I, T> {
    first: A,
    second: B,
    _source: PhantomData<F>,
    _intermediate: PhantomData<I>,
    _dest: PhantomData<T>,
}
impl<A, B, F, I, T> CoordOperation<F, T> for ConcatenatedCoordOp<A, B, F, I, T>
where
    A: CoordOperation<F, I>,
    B: CoordOperation<I, T>,
{
    #[inline]
    fn op(&self, from: F) -> T {
        self.second.op(self.first.op(from))
    }
}
impl<A, B, F, I, T> ConcatenatedCoordOp<A, B, F, I, T>
where
    A: CoordOperation<F, I>,
    B: CoordOperation<I, T>,
{
    pub fn concat(first: A, second: B) -> Self {
        Self {
            first,
            second,
            _source: PhantomData {},
            _intermediate: PhantomData {},
            _dest: PhantomData {},
        }
    }
}
