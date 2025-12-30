//This file is licensed under EUPL v1.2

use std::marker::PhantomData;

use crate::ellipsoid::{self, Ellipsoid};

#[deprecated]
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

pub trait DbContstruct: Sized {
    fn from_db<G>(getter: G) -> Option<Self>
    where
        G: FnMut(u32) -> Option<f64>;
}

/// A projected coordinate, in map units, e.g. meters. The axes are not necessarily geographically aligned. Depending on the CRS, they might also be called Southing and Westing or X and Y.
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

/// A projected coordinate like [`ProjectedCoordinate`] but with a vertical component. The vertical component's meaning is user-defined.
pub struct ProjectedCoordinateUserVertical {
    easting: f64,
    northing: f64,
    vertical: f64,
}
impl ProjectedCoordinateUserVertical {
    pub fn new(easting: f64, northing: f64, vertical: f64) -> Self {
        Self {
            easting,
            northing,
            vertical,
        }
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

/// A geographic coordinate in angular units. The angles are relative to the reference meridian and the equator.
pub struct Geographic2DCoordinate {
    longitude: f64,
    latitude: f64,
}
impl Geographic2DCoordinate {
    /// Construct the geographic coordinate with angles in degrees.
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude: longitude.to_radians(),
            latitude: latitude.to_radians(),
        }
    }

    /// Construct the geographic coordinate with angles in radians.
    pub fn new_rad(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
        }
    }

    /// Longitude in radians.
    pub fn longitude_rad(&self) -> f64 {
        self.longitude
    }

    /// Longitude in degrees.
    pub fn longitude(&self) -> f64 {
        self.longitude.to_degrees()
    }

    /// Latidude in radians.
    pub fn latitude_rad(&self) -> f64 {
        self.latitude
    }

    /// Latitude in degrees.
    pub fn latitude(&self) -> f64 {
        self.latitude.to_degrees()
    }
}

/// A geographic coordinate like [`Geographic2DCoordinate`] but with a vertical component. The vertical component's meaning is user-defined.
pub struct Geographic2DCoordinateUserVertical {
    longitude: f64,
    latitude: f64,
    vertical: f64,
}
impl Geographic2DCoordinateUserVertical {
    /// Construct the geographic coordinade with angles in degrees.
    pub fn new(longitude: f64, latitude: f64, vertical: f64) -> Self {
        Self {
            longitude: longitude.to_radians(),
            latitude: latitude.to_radians(),
            vertical,
        }
    }

    /// Construct the geographic coordinate with angles in radians.
    pub fn new_rad(longitude: f64, latitude: f64, vertical: f64) -> Self {
        Self {
            longitude,
            latitude,
            vertical,
        }
    }

    /// Longitude in radians.
    pub fn longitude_rad(&self) -> f64 {
        self.longitude
    }

    /// Longitude in degrees.
    pub fn longitude(&self) -> f64 {
        self.longitude.to_degrees()
    }

    /// Latidude in radians.
    pub fn latitude_rad(&self) -> f64 {
        self.latitude
    }

    /// Latitude in degrees.
    pub fn latitude(&self) -> f64 {
        self.latitude.to_degrees()
    }

    /// The user-defined vertical coordinate.
    pub fn vertical(&self) -> f64 {
        self.vertical
    }
}

/// This struct wraps a 2D projection and carries over the `UserVertical` component unchanged.
pub struct ProjectionUserVertical<P> {
    projection: P,
}
impl<P: CoordOperation<ProjectedCoordinate, Geographic2DCoordinate>>
    CoordOperation<ProjectedCoordinateUserVertical, Geographic2DCoordinateUserVertical>
    for ProjectionUserVertical<P>
{
    #[inline]
    fn op(
        &self,
        ProjectedCoordinateUserVertical {
            easting,
            northing,
            vertical,
        }: ProjectedCoordinateUserVertical,
    ) -> Geographic2DCoordinateUserVertical {
        let Geographic2DCoordinate {
            longitude,
            latitude,
        } = self
            .projection
            .op(ProjectedCoordinate { easting, northing });
        Geographic2DCoordinateUserVertical {
            longitude,
            latitude,
            vertical,
        }
    }
}
impl<P: CoordOperation<Geographic2DCoordinate, ProjectedCoordinate>>
    CoordOperation<Geographic2DCoordinateUserVertical, ProjectedCoordinateUserVertical>
    for ProjectionUserVertical<P>
{
    #[inline]
    fn op(
        &self,
        Geographic2DCoordinateUserVertical {
            longitude,
            latitude,
            vertical,
        }: Geographic2DCoordinateUserVertical,
    ) -> ProjectedCoordinateUserVertical {
        let ProjectedCoordinate { easting, northing } =
            self.projection.op(Geographic2DCoordinate {
                longitude,
                latitude,
            });
        ProjectedCoordinateUserVertical {
            easting,
            northing,
            vertical,
        }
    }
}
impl<P: CoordOperation<Geographic2DCoordinate, Geographic2DCoordinate>>
    CoordOperation<Geographic2DCoordinateUserVertical, Geographic2DCoordinateUserVertical>
    for ProjectionUserVertical<P>
{
    fn op(
        &self,
        Geographic2DCoordinateUserVertical {
            longitude,
            latitude,
            vertical,
        }: Geographic2DCoordinateUserVertical,
    ) -> Geographic2DCoordinateUserVertical {
        let Geographic2DCoordinate {
            longitude,
            latitude,
        } = self.projection.op(Geographic2DCoordinate {
            longitude,
            latitude,
        });
        Geographic2DCoordinateUserVertical {
            longitude,
            latitude,
            vertical,
        }
    }
}
impl<P: CoordOperation<ProjectedCoordinate, ProjectedCoordinate>>
    CoordOperation<ProjectedCoordinateUserVertical, ProjectedCoordinateUserVertical>
    for ProjectionUserVertical<P>
{
    fn op(
        &self,
        ProjectedCoordinateUserVertical {
            easting,
            northing,
            vertical,
        }: ProjectedCoordinateUserVertical,
    ) -> ProjectedCoordinateUserVertical {
        let ProjectedCoordinate { easting, northing } = self
            .projection
            .op(ProjectedCoordinate { easting, northing });
        ProjectedCoordinateUserVertical {
            easting,
            northing,
            vertical,
        }
    }
}

/// A geographic coordinate with a vertical component. The vertical component is relative to the reference ellipsoid and its units are given by the underlying reference system.
pub struct Geographic3DCoordinate {
    longitude: f64,
    latitude: f64,
    ell_height: f64,
}
impl Geographic3DCoordinate {
    /// Construct the geographic coordinate with angles in degrees.
    pub fn new(longitude: f64, latitude: f64, ell_height: f64) -> Self {
        Self {
            longitude: longitude.to_radians(),
            latitude: latitude.to_radians(),
            ell_height,
        }
    }

    /// Construct the geographic coordinate with angles in radians.
    pub fn new_rad(longitude: f64, latitude: f64, ell_height: f64) -> Self {
        Self {
            longitude,
            latitude,
            ell_height,
        }
    }

    /// Longitude in degrees.
    pub fn longitude(&self) -> f64 {
        self.longitude.to_degrees()
    }

    /// Longitude in radians.
    pub fn longitude_rad(&self) -> f64 {
        self.longitude
    }

    /// Latitude in degrees.
    pub fn latitude(&self) -> f64 {
        self.latitude.to_degrees()
    }

    /// Latidude in radians.
    pub fn latitude_rad(&self) -> f64 {
        self.latitude
    }

    /// Ellipsoid height.
    pub fn ellipsoid_height(&self) -> f64 {
        self.ell_height
    }
}

/// A geocentric coordinate. The geocentric coordiante system is a cartesic coordinate system where the X-axis passes through the intersection of reference meridian and equator, the Z-axis passes through the poles and the Y-axis is perpendicular such that XYZ is a right-hand system.
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

pub struct GeocentricConversion {
    ell: Ellipsoid,
}
impl GeocentricConversion {
    pub fn new(ellipsoid: Ellipsoid) -> Self {
        Self { ell: ellipsoid }
    }
}
impl CoordOperation<Geographic3DCoordinate, GeocentricCoordinate> for GeocentricConversion {
    fn op(&self, from: Geographic3DCoordinate) -> GeocentricCoordinate {
        self.ell.radians_to_geocentric(from)
    }
}
impl CoordOperation<GeocentricCoordinate, Geographic3DCoordinate> for GeocentricConversion {
    fn op(&self, from: GeocentricCoordinate) -> Geographic3DCoordinate {
        self.ell.geocentric_to_radians(from)
    }
}

/// A specific operation on coordinates. This can be a projection, a transformation, a unit conversion, etc.
pub trait CoordOperation<F, T> {
    fn op(&self, from: F) -> T;
}

/// A specific operation that is achieved by executing one operation and then another on the results of the first.
pub struct ConcatenatedCoordOp<A: CoordOperation<F, I>, B: CoordOperation<I, T>, F, I, T> {
    first: A,
    second: B,
    _from: PhantomData<F>,
    _intermediate: PhantomData<I>,
    _to: PhantomData<T>,
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
            _from: PhantomData {},
            _intermediate: PhantomData {},
            _to: PhantomData {},
        }
    }
}
