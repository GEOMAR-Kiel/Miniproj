# Miniproj

This crate implements geographic coordinate transformations between projected
coordinate systems and their underlying geographic coordinate systems, for
projected coordinate reference systems defined by the
[European Petroleum Survey Group](https://epsg.org/home.html). It was
originally developped at the
[GEOMAR Helmholtz Centre for Ocean Research](https://www.geomar.de/) as part of
the [Digital Earth Project](https://www.digitalearth-hgf.de/).

The conversions are implemented according to the
[Guidance Notes](https://epsg.org/guidance-notes.html), with all "dynamically
uniform" local variables calculated at compile time. The conversions are then
stored in a static [`PHFMap`](https://crates.io/crates/phf) for quick access at
runtime. Code generation is split out into the `miniproj-epsg-registry` crate,
while the operations themselves are implemented in `miniproj-ops`.

### Scope

EPSG Code | Operation Name                        | # of Projected CRS covered
-----     |-----                                  |-----
9807      | Transverse Mercator                   | 3615
9802      | Lambert Conic Conformal (2SP)         | 950
9820      | Lambert Azimuthal Equal Area          | 14
9810      | Polar Stereographic (Variant A)       | 10
1024      | Popular Visualisation Pseudo-Mercator | 1

### Usage example
```rust
// Create a boxed converter between WGS84 Lat/Lon and WGS84 UTM zone 32N
use miniproj::{get_coord_transform, CoordTransform};
let converter = get_coord_transform(32632).expect("Coordinate conversion not implemented");

// Coordinates of the office where this converter was written in UTM:
let (easting, northing) = (576935.86f64, 6020593.46f64);

// To get the latitude and longitude, use the CoordTransform::to_deg method.
// Note that the order of the returned tuple is not alphabetical, but instead
// follows the axis order (X for Longitude, Y for Latitude)
let (lon, lat) = converter.to_deg(easting, northing);

assert!((lon - 10.183034) < 0.000001);
assert!((lat - 54.327389) < 0.000001);
```

## Changelog

#### 0.1.1

* Initial release

#### 0.2.0

* Added Polar Stereographic Method A (10 defined CRS)

#### 0.3.0

* Added Lambert Conic Conformal (2SP) (950 defined CRS)
* Fixed some major bugs in Polar Stereographic A

#### 0.4.0

* Added Popular Visualisation Pseudo-Mercator (1 defined CRS).
    This method might be the most popular, as it is the map
    projection used by Google, OpenStreetMap etc.

## License

As many of the other components of the Digital Earth Viewer, **Miniproj** is licensed under **[EUPL v1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)**, which is a **copyleft license similar and compatible to GPLv2** and available in 23 languages. This license does not apply to the projections themselves. The database files are extracts from the EPSG Geodetic Parameter Registry and redistributed under [their own Terms of Use](epsg-geodetic-parameter-gen/data/terms.md).