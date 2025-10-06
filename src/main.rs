mod framebuffer;
mod raytracer;
mod ray_intersect;
mod camera;
mod light;
mod color;
mod texture;
mod cube;

use framebuffer::Framebuffer;
use nalgebra::Vector3;
use ray_intersect::Material;
use camera::Camera;
use light::Light;
use color::Color;
use std::f32::consts::PI;
use std::time::Duration;
use std::io::{self, Write};
use minifb::{Key, Window, WindowOptions};
use std::path::Path;
use std::time::Instant;
use crate::raytracer::render;
use crate::texture::Texture;
use crate::ray_intersect::RayIntersect;
use crate::cube::Cube;


fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer_high = Framebuffer::new(width, height);
    let mut framebuffer_medium = Framebuffer::new(width / 2, height / 2);
    let mut framebuffer_low = Framebuffer::new(width / 6, height / 6); // Ultra low for movement
    let mut framebuffer_ultra_low = Framebuffer::new(width / 8, height / 8); // Emergency performance mode

    let mut window = Window::new(
        "Minecraft Diorama Raytracing",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let agua_texture = Texture::load_from_file("assets/agua.jpg");
    let tierra_texture = Texture::load_from_file("assets/tierra.jpeg");
    let tierra_grama_texture = Texture::load_from_file("assets/tierra2.png");
    let grama_texture = Texture::load_from_file("assets/grama.png");
    let madera_texture = Texture::load_from_file("assets/madera.jpg");
    let hoja_texture = Texture::load_from_file("assets/hoja_arbol.jpg");
    let piedra_texture = Texture::load_from_file("assets/piedra.png");
    let arena_texture = Texture::load_from_file("assets/arena.png");

    //let textura_solida = Color::new(255, 0, 0);
    //let material_prueba = Material::new(textura_solida, 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, false, None);

    let mut camera = Camera::new(
        Vector3::new(0.0, 5.0, -10.0),  
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.0, 1.0, 0.0),  
    );

    // Definimos la luz
    let lights = vec![
        Light::new(Vector3::new(100.0, 100.0, -100.0), Color::new(255, 255, 255), 2.0, 5.0), 
        Light::new(Vector3::new(-100.0, -100.0, 100.0), Color::new(255, 255, 255), 2.0, 5.0),
    ];

    // Definimos los materiales 
    let tierra_grama = Material::new(Color::new(255, 255, 255), 32.0, [0.9, 0.1, 0.0, 0.0], 1.0, true, Some(tierra_grama_texture.clone()));
    let tierra = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(tierra_texture.clone()));
    let grama = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(grama_texture.clone()));
    let agua = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(agua_texture.clone()));
    let madera = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(madera_texture.clone()));
    let piedra = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(piedra_texture.clone()));
    let hoja = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(hoja_texture.clone()));
    let arena = Material::new(Color::new(255, 255, 255), 32.0, [1.0, 0.1, 0.0, 0.0], 1.0, true, Some(arena_texture.clone()));


    let mut objects: Vec<Box<dyn RayIntersect>> = Vec::new();

    // Base de 8x8 cubos
    let grid_size = 8;
    let cube_size = 1.0;
    
    for z in 0..grid_size {
        for x in 0..grid_size {
            let x_pos = x as f32 * cube_size - (grid_size as f32 / 2.0) * cube_size;
            let z_pos = z as f32 * cube_size - (grid_size as f32 / 2.0) * cube_size;
            
            objects.push(Box::new(Cube {
                center: Vector3::new(x_pos, 0.0, z_pos),  
                size: cube_size,                        
                materials: [
                    tierra.clone(),  // Derecha
                    tierra.clone(),  // Izquierda
                    tierra.clone(),  // Abajo
                    tierra.clone(),  // Arriba
                    tierra.clone(),  // Frente
                    tierra.clone(),  // Atrás
                ],
            }));
        }
    }
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(), arena.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(), agua.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, -1.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, -2.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(0.0, 1.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra.clone(), tierra.clone(), tierra.clone(), grama.clone(), tierra.clone(), tierra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 2.0, -2.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 2.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 2.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 2.0, -3.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 2.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 2.0, -4.0),  
        size: 1.0,                          
        materials: [
            tierra_grama.clone(), tierra_grama.clone(), tierra.clone(), grama.clone(), tierra_grama.clone(), tierra_grama.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-1.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 1.0, 0.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 1.0, 1.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 1.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-2.0, 2.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 2.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 2.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 2.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 2.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 2.0, 1.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-3.0, 3.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 3.0, 3.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(-4.0, 3.0, 2.0),  
        size: 1.0,                          
        materials: [
            piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(), piedra.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 2.0, -2.0),  
        size: 1.0,                          
        materials: [
            madera.clone(), madera.clone(), madera.clone(), madera.clone(), madera.clone(), madera.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 3.0, -2.0),  
        size: 1.0,                          
        materials: [
            madera.clone(), madera.clone(), madera.clone(), madera.clone(), madera.clone(), madera.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 6.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 5.0, -1.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 5.0, -3.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 5.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 5.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 4.0, -1.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(2.0, 4.0, -3.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 4.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 4.0, -2.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 4.0, -1.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 4.0, -1.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(1.0, 4.0, -3.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));
    objects.push(Box::new(Cube {
        center: Vector3::new(3.0, 4.0, -3.0),  
        size: 1.0,                          
        materials: [
            hoja.clone(), hoja.clone(), hoja.clone(), hoja.clone(),hoja.clone(), hoja.clone(),
        ],
    }));

    let glass = Material::new(Color::new(255, 255, 255), 125.0, [0.0, 0.5, 0.1, 0.8], 1.5, false, None); // Vidrio, 80% transparente, índice de refracción 1.5
    
    let mut needs_render = true;
    let mut camera_moved = false;
    let animation_start = Instant::now();
    let mut last_animation_update = Instant::now();
    let mut last_frame_time = Instant::now();
    let mut movement_frames = 0;
    let mut frame_count = 0;
    
    // Render initial frame at medium quality for faster startup
    render(&mut framebuffer_medium, &objects, &camera, &lights);
    let scaled_initial = upscale_framebuffer(framebuffer_medium.get_buffer(), framebuffer_medium.width, framebuffer_medium.height, width, height);
    window.update_with_buffer(&scaled_initial, width, height).unwrap();
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        camera_moved = false;

        if window.is_key_down(Key::W) {
            camera.mover_enfrente(0.4);
            camera_moved = true;
        }
        if window.is_key_down(Key::S) {
            camera.mover_atras(0.4);
            camera_moved = true;
        }
        if window.is_key_down(Key::A) {
            camera.mover_izq(0.4);
            camera_moved = true;
        }
        if window.is_key_down(Key::D) {
            camera.mover_der(0.4);
            camera_moved = true;
        }

        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -0.1);
            camera_moved = true;
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, 0.1);
            camera_moved = true;
        }
        if window.is_key_down(Key::Left) {
            camera.orbit(-0.1, 0.0);
            camera_moved = true;
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(0.1, 0.0);
            camera_moved = true;
        }

        if camera_moved {
            movement_frames += 1;
        } else {
            movement_frames = 0;
        }
        
        frame_count += 1;
        let frame_time = last_frame_time.elapsed();
        let target_fps = Duration::from_millis(33); // Target ~30 FPS
        
        let should_animate = !camera_moved && last_animation_update.elapsed() >= Duration::from_millis(200);
        if should_animate {
            let elapsed_time = animation_start.elapsed().as_secs_f32();
            for (i, object) in objects.iter_mut().enumerate() {
                if let Some(cube) = object.as_any_mut().downcast_mut::<Cube>() {
                    if cube.materials.iter().any(|m| m.has_texture && m.texture.as_ref().map_or(false, |t| t == &agua_texture)) {
                        let desfase = i as f32 * 0.3;
                        cube.center.x += (elapsed_time * 0.3 + desfase).sin() * 0.03;
                    }
                }
            }
            last_animation_update = Instant::now();
        }

        if camera_moved || should_animate {
            if camera_moved {
                if frame_time > Duration::from_millis(50) || movement_frames > 10 {
                    render(&mut framebuffer_ultra_low, &objects, &camera, &lights);
                    let scaled_framebuffer = upscale_framebuffer(framebuffer_ultra_low.get_buffer(), framebuffer_ultra_low.width, framebuffer_ultra_low.height, width, height);
                    window.update_with_buffer(&scaled_framebuffer, width, height).unwrap();
                } else if frame_time > Duration::from_millis(25) {
                    render(&mut framebuffer_low, &objects, &camera, &lights);
                    let scaled_framebuffer = upscale_framebuffer(framebuffer_low.get_buffer(), framebuffer_low.width, framebuffer_low.height, width, height);
                    window.update_with_buffer(&scaled_framebuffer, width, height).unwrap();
                } else {
                    render(&mut framebuffer_medium, &objects, &camera, &lights);
                    let scaled_framebuffer = upscale_framebuffer(framebuffer_medium.get_buffer(), framebuffer_medium.width, framebuffer_medium.height, width, height);
                    window.update_with_buffer(&scaled_framebuffer, width, height).unwrap();
                }
            } else {
                if frame_count % 3 == 0 {
                    render(&mut framebuffer_high, &objects, &camera, &lights);
                    window.update_with_buffer(framebuffer_high.get_buffer(), width, height).unwrap();
                }
            }
        }
        
        last_frame_time = Instant::now();

        if !camera_moved {
            std::thread::sleep(Duration::from_millis(100)); 
        } else {
            std::thread::sleep(Duration::from_millis(8)); 
        }
    }
}

fn upscale_framebuffer(low_res_buffer: &[u32], low_width: usize, low_height: usize, high_width: usize, high_height: usize) -> Vec<u32> {
    let mut high_res_buffer = vec![0; high_width * high_height];

    for y in 0..high_height {
        let src_y = y * low_height / high_height;
        for x in 0..high_width {
            let src_x = x * low_width / high_width;
            let src_index = src_y * low_width + src_x;
            let dst_index = y * high_width + x;
            high_res_buffer[dst_index] = low_res_buffer[src_index];
        }
    }

    high_res_buffer
}