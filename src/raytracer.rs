use nalgebra::Vector3;
use rayon::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect, Material};
use crate::camera::Camera;
use crate::light::Light;
use crate::color::Color;

pub fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn RayIntersect>], camera: &Camera, lights: &[Light]) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let aspect_ratio = width as f32 / height as f32;
    let chunk_size = 32; 

    // Use chunked parallelization which is more efficient for raytracing
    framebuffer.buffer.par_chunks_mut(width * chunk_size).enumerate().for_each(|(chunk_idx, chunk)| {
        let base_y = chunk_idx * chunk_size;
        
        for (local_y, row) in chunk.chunks_mut(width).enumerate() {
            let y = base_y + local_y;
            if y >= height { break; }
            
            let screen_y = -((2.0 * y as f32) / height as f32 - 1.0);

            for (x, pixel) in row.iter_mut().enumerate() {
                let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
                let screen_x = screen_x * aspect_ratio;

                let ray_direction = camera.base_change(&Vector3::new(screen_x, screen_y, -1.0).normalize());

                let mut pixel_color = Color::new(0, 0, 0);

                if let Some(light) = lights.first() {
                    let light_contrib = cast_ray(&camera.eye, &ray_direction, objects, light, 0);
                    pixel_color = light_contrib;
                }

                *pixel = ((pixel_color.r as u32) << 16)
                    | ((pixel_color.g as u32) << 8)
                    | (pixel_color.b as u32);
            }
        }
    });
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let distance_to_light = (light.position - intersect.point).magnitude();
    let shadow_ray_origin = intersect.point + intersect.normal * 1e-3;

    let max_objects = objects.len().min(10);
    for object in &objects[..max_objects] {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < distance_to_light {
            return 0.8; 
        }
    }

    0.0 
}

fn refract(incident: &Vector3<f32>, normal: &Vector3<f32>, eta_t: f32) -> Vector3<f32> {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);

    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

pub fn cast_ray(
    ray_origin: &Vector3<f32>,
    ray_direction: &Vector3<f32>,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
    depth: u32,
) -> Color {
    if depth > 1 {
        return Color::new(0, 0, 0);  
    }

    let mut closest_intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let intersect = object.ray_intersect(ray_origin, ray_direction);
        if intersect.is_intersecting && intersect.distance < zbuffer {
            zbuffer = intersect.distance;
            closest_intersect = intersect;
        }
    }

    if !closest_intersect.is_intersecting {
        return Color::new(4, 12, 36);  
    }

    let diffuse_color = closest_intersect.material.get_diffuse_color(closest_intersect.u, closest_intersect.v);
    let light_dir = (light.position - closest_intersect.point).normalize();
    let view_dir = (ray_origin - closest_intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &closest_intersect.normal);

    let shadow_intensity = cast_shadow(&closest_intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = light_dir.dot(&closest_intersect.normal).max(0.0).min(1.0);
    let diffuse = diffuse_color.scale(closest_intersect.material.albedo[0] * diffuse_intensity * light_intensity);

    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(closest_intersect.material.specular);
    let specular = light.color.scale(closest_intersect.material.albedo[1] * specular_intensity * light_intensity);

    diffuse + specular
}

fn reflect(incident: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    incident - 2.0 * incident.dot(normal) * normal
}