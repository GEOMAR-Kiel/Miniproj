# Miniproj

This workspace contains crates that implement geographic coordinate projections between projected
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
