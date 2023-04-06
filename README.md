# Miniproj

This crate implements geographic transformations between different coordinate systems defined by the [European Petroleum Survey Group](https://epsg.org/home.html).

Think of it as a very lightweight [PROJ](https://github.com/OSGeo/PROJ). Conversions between projected and geographic coordinate systems that are assigned an EPSG code are implemented according to the [Guidance Notes](https://epsg.org/guidance-notes.html), with all "dynamically uniform" local variables being calculated at compile time. The conversions are then stored in a static PHF Map for quick access at runtime. Code generation and actual implementation of specific operations are implemented in the `miniproj-epsg-registry` and `miniproj-ops` crates respectively.

Currently, only the transverse mercator and lambert azimuthal equal area coordinate operations are completely implemented.

It was written at the [GEOMAR Helmholtz Centre for Ocean Research](https://www.geomar.de/) as part of the [Digital Earth Project](https://www.digitalearth-hgf.de/).

As many of the other components created in this project, it is licensed under [EUPL v1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12). This license
does not apply to the projections themselves. The database files are extracts from the EPSG
Geodetic Parameter Registry and distributed under [their own Terms of Use](epsg-geodetic-parameter-gen/data/terms.md).

### Usage example
```rust
// Create a boxed converter between WGS84 Lat/Lon and WGS84 UTM zone 32N
use miniproj::{get_coord_transform, CoordTransform};
let converter = get_coord_transform(32632).expect("Coordinate conversion not implemented");

// Coordinates of the office where this converter was written in UTM:
let (x,y) = (576935.86f64, 6020593.46f64);

// To get the latitude and longitude, use the CoordTransform::to_deg method.
let (lon, lat) = converter.to_deg(x,y);

assert!((lon - 10.183034) < 0.000001);
assert!((lat - 54.327389) < 0.000001);
```

### Changelog

#### 0.2.0

* Added Polar Stereographic Method A (10 defined projections)