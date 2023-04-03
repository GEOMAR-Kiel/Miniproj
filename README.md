# EPSG Geodetic Parameters

[Documentation](https://flemming-staebler.pages.geomar.de/epsg-geodetic-parameters/epsg-geodetic-parameters/)

This crate implements geographic transformations between different coordinate systems defined by the [European Petroleum Survey Group](https://epsg.org/home.html).

Think of it as a very lightweight [PROJ](https://github.com/OSGeo/PROJ).

Currently, only the transverse mercator and lambert azimuthal equal area coordinate operations are completely implemented.

It was written at the [GEOMAR Helmholtz Centre for Ocean Research](https://www.geomar.de/) as part of the [Digital Earth Project](https://www.digitalearth-hgf.de/).

As many of the other components created in this project, it is licensed under [EUPL v1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12). This license does not apply to the projections themselves. The database files [parameters.sqlite](epsg-geodetic-parameter-gen/data/parameters.sqlite) and [gen_req.sql](epsg-geodetic-parameter-gen/data/gen_req.sql) are extracts from the EPSG Geodetic Parameter Registry and distributed under [their own Terms of Use](epsg-geodetic-parameter-gen/data/terms.md).

Usage example:
```rust
//Create a boxed converter between WGS84 Lat/Lon and WGS84 UTM zone 32N
use epsg_geodetic_parameters::{get_coord_transform, CoordTransform};
let converter = get_coord_transform(32632).expect("Coordinate conversion not implemented");

//Coordinates of the office where this converter was written in UTM:
let (x,y) = (576935.86f64, 6020593.46f64);

//To get the latitude and longitude, use the CoordTransform::to_deg method.
let (lon, lat) = converter.to_deg(x,y);

assert!((lon - 10.183034) < 0.000001);
assert!((lat - 54.327389) < 0.000001);
```
