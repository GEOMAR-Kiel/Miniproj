# EPSG Geodetic Parameters

[Documentation](https://flemming-staebler.pages.geomar.de/epsg-geodetic-parameters/epsg_geodetic_parameters/)


Usage example:
```rust
// Create a boxed converter between WGS84 Lat/Lon and WGS84 UTM zone 32N
use miniproj::{get_coord_transform, CoordTransform};
let converter = get_coord_transform(32632).expect("Coordinate conversion not implemented");

Coordinates of the office where this converter was written in UTM:
let (x,y) = (576935.86f64, 6020593.46f64);

// To get the latitude and longitude, use the CoordTransform::to_deg method.
let (lon, lat) = converter.to_deg(x,y);

assert!((lon - 10.183034) < 0.000001);
assert!((lat - 54.327389) < 0.000001);
```