# Miniproj

This crate implements geographic coordinate projections between projected
coordinate systems and their underlying geographic coordinate systems, for
projected coordinate reference systems defined by the
[European Petroleum Survey Group Geodesy](https://epsg.org/home.html). It was
originally developped at the
[GEOMAR Helmholtz Centre for Ocean Research](https://www.geomar.de/) as part of
the [Digital Earth Project](https://www.digitalearth-hgf.de/), and continues to
provide reprojection functionality to the Digital Earth Viewer.

The projections are implemented according to the
[Guidance Notes](https://epsg.org/guidance-notes.html), with all "dynamically
uniform" local variables calculated at compile time. The projections are then
stored in a static [`PHFMap`](https://crates.io/crates/phf) for quick access at
runtime. Code generation is split out into the `miniproj-epsg-registry` crate,
while the operations themselves are implemented in `miniproj-ops`.

Miniproj is **not** related to or derived from Proj.

### Scope

#### Projections

EPSG Code | Operation Method Name                 | # of Projected CRS covered
----------|---------------------------------------|---------------------------
9807      | Transverse Mercator                   | 3591
9802      | Lambert Conic Conformal (2SP)         | 949
9801      | Lambert Conic Conformal (1SP)         | 215
9822      | Albers Equal Area                     | 36
9809      | Oblique Stereographic                 | 20
9820      | Lambert Azimuthal Equal Area          | 14
9810      | Polar Stereographic (Variant A)       | 10
1024      | Popular Visualisation Pseudo-Mercator | 1

#### Conversions

EPSG Code | Operation Method Name
----------|----------------------------------
9602      | Geographic/Geocentric Conversions

#### Transformations

EPSG Code | Operation Method Name
----------|----------------------
   None   | Under Development

### Usage example

```rust
// Get the WGS84 UTM zone 32N projection
use miniproj::{
    get_projection,
    Projection,
    get_ellipsoid_code,
    get_ellipsoid,
    get_reference_system_name,
    Ellipsoid
};
let projection = get_projection(32632)
    .expect("Projection not implemented.");

// Coordinates of the office where this crate was written in UTM:
let (easting, northing) = (576935.86f64, 6020593.46f64);

// To get the latitude and longitude, use the Projection::to_deg
// method. Note that the order of the returned tuple is not 
// alphabetical, but instead follows the axis order (X for
// Longitude, Y for Latitude).
let (lon, lat) = projection.projected_to_deg(easting, northing);

assert!((lon - 10.183034).abs() < 0.000001);
assert!((lat - 54.327389).abs() < 0.000001);

// To convert this geographic position to a geocentric position
// (a position in euclidian space), get the underlying ellipsoid:

let ellipsoid = get_ellipsoid_code(32632)
    .and_then(|c| get_ellipsoid(c))
    .expect("No associated ellipsoid.");

// Do the actual conversion. Axis order applies as explained above.
// Height as per GPS altitude.
let (x, y, z) = ellipsoid.deg_to_geocentric(lon, lat, 53.7);

assert!((x - 3668985.10).abs() < 0.1);
assert!((y - 659033.08).abs() < 0.1);
assert!((z - 5158122.64).abs() < 0.1);

let name = get_reference_system_name(32632)
    .expect("Name is unknown");
assert_eq!("WGS 84 / UTM zone 32N", name);

```

### Limitations

Miniproj is still under development and missing some important functionality. If
you are looking for a refined, proven library, check out
[PROJ](https://crates.io/crates/proj).

## Changelog

#### 0.10.0

* Expose constructing projections for a given projection method code.
    This can be used to create a projection instance for parameter sets or ellipsoids that aren't part of the EPSG registry.

#### 0.9.0

* Removed Dependency `rusqlite`
* `miniproj` can now be built with stable `rustc` (1.71.0)
* No longer special-case "Identity" Projection, now it allows more geographic coordinate systems to be treated as if they were projected (606 defined CRS)
* For now, filter CRS that do not use the Greenwich Meridian

#### 0.8.0

* Added Albers Equal Area (36 defined CRS)

#### 0.7.0

* Added Oblique Stereographic (20 defined CRS)

#### 0.6.0

* Expose Ellipsoids
* Added an interface to access `Ellipsoid`s by EPSG code
* Added an interface to find the underlying ellipsoid for a projection by EPSG code

#### 0.5.0

* Added Lambert Conic Conformal (1SP) (233 defined CRS)
* Cleared up some terminology

#### 0.4.0

* Added Popular Visualisation Pseudo-Mercator (1 defined CRS).
    This method might be the most popular, as it is the map
    projection used by Google, OpenStreetMap etc.

#### 0.3.0

* Added Lambert Conic Conformal (2SP) (950 defined CRS)
* Fixed some major bugs in Polar Stereographic A

#### 0.2.0

* Added Polar Stereographic Method A (10 defined CRS)

#### 0.1.1

* Initial release

## Roadmap

#### Before 1.0.0

* Implement transformations
* Implement refined handling of nonstandard meridians
* Add feature gating for operation methods

#### Long-Term

* Implement all missing projections
* Implement parsing of WKT and similar
* Make a subset of miniproj `no-std`
* Add interoperability with crates like `geo`

## License

As many of the other components of the Digital Earth Viewer, **Miniproj** is licensed under **[EUPL v1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)**, which is a **copyleft license similar and compatible to GPLv2** and available in 23 languages. This license does not apply to the projections themselves. The database files are extracts from the EPSG Geodetic Parameter Registry and redistributed under [their own Terms of Use](epsg-geodetic-parameter-gen/data/terms.md).

