//This file is licensed under EUPL v1.2 as part of the Digital Earth Viewer

pub trait CoordTransform: Send + Sync{
    fn to_rad(&self, x: f64, y: f64) -> (f64, f64);
    fn from_rad(&self, lon: f64, lat: f64) -> (f64, f64);
    fn to_deg(&self, x: f64, y: f64) -> (f64, f64) {
        let tmp = self.to_rad(x, y);
        (tmp.0.to_degrees(), tmp.1.to_degrees())
    }
    fn from_deg(&self, lon: f64, lat: f64) -> (f64, f64) {
        self.from_rad(lon.to_radians(), lat.to_radians())
    }
 }