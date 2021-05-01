use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::util::degrees_to_radians;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&lookfrom - &lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;
        Camera {
            lower_left_corner: &origin
                - &horizontal / 2.0
                - &vertical / 2.0
                - focus_dist * &w,
            origin,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = &self.u * rd.x + &self.v * rd.y;
        Ray {
            origin: &self.origin + &offset,
            direction: &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin - &offset,
        }
    }
}
