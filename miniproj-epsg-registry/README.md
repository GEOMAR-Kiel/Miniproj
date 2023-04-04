# Miniproj EPSG Registry

This crate provides the actual projection parameters for Miniproj. The parameters are converted to an intermediate set of values depending on the coordinate operation (implemented in `miniproj-ops`) and output as rust source code for inclusion via build script in `miniproj`.