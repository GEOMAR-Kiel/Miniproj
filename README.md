# EPSG Geodetic Parameters
This crate implements geographic transformations between different coordinate systems defined by the European Petroleum Survey Group.

Think of it as a very lightweight [PROJ](https://github.com/OSGeo/PROJ)

Currently, only the transverse mercator, stereographic and lamber azimuthal equal area coordinate systems are defined.

It was written at the [GEOMAR Helmholtz Centre for Ocean Research](https://www.geomar.de/) as part of the [Digital Earth Project](https://www.digitalearth-hgf.de/).

As many of the other components created in this project, it is licensed under [EUPL v1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)

Usage example:
```rust
//Create a boxed converter between WGS84 Lat/Lon and WGS84 UTM zone 32N
use epsg_geodetic_parameters::{get_coord_transform, CoordTransform};
let converter = get_coord_transform(32632).expect("Coordinate conversion not implemented");

//Coordinates of the office where this converter was written in UTM:
let (x,y) = (1133571.07f64,7232406.06f64);

//To get the latitude and longitude, use the CoordTransform::to_deg method.
let (lon, lat) = converter.to_deg(x,y);

assert_eq!((lon, lat), (10.1830402, 54.3274021));
```