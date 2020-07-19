use super::*;

pub fn cornell_box(config: &Config) -> (World, Arc<Camera>) {
    let lookfrom = Vec3(278.0, 278.0, -800.0);
    let lookat = Vec3(278.0, 278.0, 0.0);
    let camera = Arc::new(Camera::new(
        (lookfrom, lookat, Vec3(0.0, 1.0, 0.0)),
        40.0,
        config.image_width as f64 / config.image_height as f64,
        0.0,
        10.0,
        (0.0, 1.0),
    ));

    let mut world = HitList::new();

    let red = Lambertian::from_rgb(0.65, 0.05, 0.05);
    let white = Lambertian::from_rgb(0.73, 0.73, 0.73);
    let green = Lambertian::from_rgb(0.12, 0.45, 0.15);
    let light = DiffuseLight::from_texture(SolidColor::new(15.0, 15.0, 15.0));

    world.add(FlipFace::new(AxisRectangle::new(
        "X",
        (555.0, 555.0),
        (0.0, 555.0),
        (0.0, 555.0),
        green.clone(),
    )));

    world.add(AxisRectangle::new(
        "X",
        (0.0, 0.0),
        (0.0, 555.0),
        (0.0, 555.0),
        red.clone(),
    ));

    world.add(AxisRectangle::new(
        "Y",
        (213.0, 343.0),
        (554.0, 554.0),
        (227.0, 332.0),
        light,
    ));

    world.add(FlipFace::new(AxisRectangle::new(
        "Y",
        (0.0, 555.0),
        (0.0, 0.0),
        (0.0, 555.0),
        white.clone(),
    )));

    world.add(AxisRectangle::new(
        "Y",
        (0.0, 555.0),
        (555.0, 555.0),
        (0.0, 555.0),
        white.clone(),
    ));

    world.add(FlipFace::new(AxisRectangle::new(
        "Z",
        (0.0, 555.0),
        (0.0, 555.0),
        (555.0, 555.0),
        white.clone(),
    )));

    let cube1 = Cube::new(Vec3::zero(), Vec3(165.0, 330.0, 165.0), white.clone());
    let cube1 = YRotation::new(cube1, 15.0);
    let cube1 = Translation::new(cube1, Vec3(265.0, 0.0, 295.0));
    world.add(cube1);

    let cube2 = Cube::new(Vec3::zero(), Vec3(165.0, 165.0, 165.0), white.clone());
    let cube2 = YRotation::new(cube2, -18.0);
    let cube2 = Translation::new(cube2, Vec3(130.0, 0.0, 65.0));
    world.add(cube2);

    let world = World::new(Arc::new(world), SolidColor::new(0.0, 0.0, 0.0));
    (world, camera)
}