use super::*;

pub mod cornell_box;

pub use cornell_box::*;

type ThreadHittable = dyn Hittable + Send + Sync;

pub fn earth() -> Arc<ThreadHittable> {
    let earth_texture =
        ImageTexture::from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/earthmap.jpg"));
    let earth_surface = Lambertian::from_texture(earth_texture);
    Sphere::new(Vec3::zero(), 2.0, earth_surface)
}

pub fn two_perlin_spheres() -> Arc<ThreadHittable> {
    let mut world = HitList::new();

    let perlin_texture = NoiseTexture::new(10.0);
    world.add(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(perlin_texture.clone()),
    ));
    world.add(Sphere::new(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        Lambertian::from_texture(perlin_texture.clone()),
    ));

    Arc::new(world)
}

pub fn simple_light(config: &Config) -> (World, Arc<Camera>) {
    let lookfrom = Vec3(30.0, 0.0, 5.0) + Vec3(0.0, 4.0, 0.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let camera = Arc::new(Camera::new(
        (lookfrom, lookat, Vec3(0.0, 1.0, 0.0)),
        20.0,
        config.image_width as f64 / config.image_height as f64,
        0.0,
        10.0,
        (0.0, 1.0),
    ));

    let mut world = HitList::new();

    let pertext = NoiseTexture::new(4.0);
    world.add(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(pertext.clone()),
    ));
    world.add(Sphere::new(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        Lambertian::from_texture(pertext.clone()),
    ));

    let difflight = DiffuseLight::from_texture(SolidColor::new(4.0, 4.0, 4.0));
    world.add(Sphere::new(Vec3(0.0, 7.0, 0.0), 2.0, difflight.clone()));
    world.add(AxisRectangle::new(
        "Z",
        (3.0, 5.0),
        (1.0, 3.0),
        (-2.0, -2.0),
        difflight.clone(),
    ));

    let world = World::new(Arc::new(world), SolidColor::new(0.0, 0.0, 0.0));

    (world, camera)
}

pub fn random_scene(config: &Config) -> (World, Arc<Camera>) {
    let lookfrom = Vec3(30.0, 1.0, 20.0);
    let lookat = Vec3(0.0, 1.0, 0.0);
    let camera = Arc::new(Camera::new(
        (lookfrom, lookat, Vec3(0.0, 1.0, 0.0)),
        20.0,
        config.image_width as f64 / config.image_height as f64,
        0.0,
        10.0,
        (0.0, 1.0),
    ));

    let mut world = HitList::new();

    let checkered = CheckerTexture::new(
        5.0,
        SolidColor::new(0.2, 0.3, 0.1),
        SolidColor::new(0.9, 0.9, 0.9),
    );
    let noise = NoiseTexture::new(10.0);
    let light = DiffuseLight::from_texture(SolidColor::new(1.0, 1.0, 1.0));
    world.add(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(checkered.clone()),
    ));

    world.add(Sphere::new(Vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    //world.add(Arc::new(Sphere::new(
    //    Vec3(0.0, 2.0, 6.0),
    //    2.0,
    //    light.clone(),
    //)));
    world.add(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::from_color3(Vec3(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vec3(0.7, 0.6, 0.5), 0.0),
    ));

    //world.add(SkySphere::from_texture(noise.clone()));
    //let world = BVH::from_hit_list(world, (0.0, 1.0));
    let world = World::new(Arc::new(world), noise.clone());

    (world, camera)
}

pub fn two_spheres() -> Arc<dyn Hittable + Send + Sync> {
    let mut world = HitList::new();
    let checkered = CheckerTexture::new(
        10.0,
        SolidColor::new(0.2, 0.3, 0.1),
        SolidColor::new(0.9, 0.9, 0.9),
    );
    let checker_matte = Lambertian::from_texture(checkered);

    world.add(Sphere::new(
        Vec3(0.0, -10.0, 0.0),
        10.0,
        checker_matte.clone(),
    ));
    world.add(Sphere::new(
        Vec3(0.0, 10.0, 0.0),
        10.0,
        checker_matte.clone(),
    ));

    Arc::new(world)
}