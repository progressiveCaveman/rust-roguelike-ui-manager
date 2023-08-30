/*

Notes on perlin examples:

This could be adopted to coasts by setting large gradient ares to certain ranges, then a quick gradient change between e.g. water to coast
    // Create a jade palette.
    let jade_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.000, [24, 146, 102, 255])
        .add_gradient_point(0.000, [78, 154, 115, 255])
        .add_gradient_point(0.250, [128, 204, 165, 255])
        .add_gradient_point(0.375, [78, 154, 115, 255])
        .add_gradient_point(1.000, [29, 135, 102, 255]);



==========================

This is basically a mashup of several layers of Perlin noise, all floats in 0-1 range:

    A layer for heights - differentiates between water, plains and mountains. Biased towards the center, because I want an island

    A layer for humidity - "plains" with humidity above a certain threshold result in swamps. I could also do deserts but decided not to, this time

    A layer for forests - values above a certain threshold become forests. I decided not to mess with more complex forest-planting algorithms for now; various implementations of flood fill came to mind but I'll probably try that later

    A layer for temperature - values below a certain threshold result in snowy areas, be it forest or plains. This layer is actually flattened a little and combined with a vertical gradient, so that north is colder but there's still some variance


3 yr. ago

That's great, the result is very nice!

I know it's unrequested advice, but you mentioned forest-planting algorithms and I have a favourite for that. What I find very simple and satisfying to use for vegetation is cellular automata: sprinkle your trees randomly and make a few generations pass. On top of that, add some probability weights to the generational algorithm to make the trees more likely to survive/thrive in temperate areas or close to water. It's a simple algorithm but it gives nice "organic" results!

3 yr. ago

That sounds like a good approach. Might also work for towns and cities perhaps too...


*/
use noise::{utils::*, *};

use crate::Image;

pub fn generate_wood() -> Image {
    // Base wood texture. Uses concentric cylinders aligned on the z-axis, like a log.
    let base_wood = Cylinders::new().set_frequency(16.0);

    // Basic Multifractal noise to use for the wood grain.
    let wood_grain_noise = BasicMulti::<Perlin>::new(0)
        .set_frequency(48.0)
        .set_persistence(0.5)
        .set_lacunarity(2.20703125)
        .set_octaves(3);

    // Stretch the perlin noise in the same direction as the center of the log. Should
    // produce a nice wood-grain texture.
    let scaled_base_wood_grain = ScalePoint::new(wood_grain_noise).set_z_scale(0.25);

    // Scale the wood-grain values so that they can be added to the base wood texture.
    let wood_grain = ScaleBias::new(scaled_base_wood_grain)
        .set_scale(0.25)
        .set_bias(0.125);

    // Add the wood grain texture to the base wood texture.
    let combined_wood = Add::new(base_wood, wood_grain);

    // Slightly perturb the wood to create a more realistic texture.
    let perturbed_wood = Turbulence::<_, Perlin>::new(combined_wood)
        .set_seed(1)
        .set_frequency(4.0)
        .set_power(1.0 / 256.0)
        .set_roughness(4);

    // Cut the wood texture a small distance from the center of the log.
    let translated_wood = TranslatePoint::new(perturbed_wood).set_y_translation(1.48);

    // Set the cut on a angle to produce a more interesting texture.
    let rotated_wood = RotatePoint::new(translated_wood).set_angles(84.0, 0.0, 0.0, 0.0);

    // Finally, perturb the wood texture again to produce the final texture.
    let final_wood: Turbulence<
        RotatePoint<
            TranslatePoint<
                Turbulence<
                    Add<f64, Cylinders, ScaleBias<f64, ScalePoint<BasicMulti<Perlin>>, 2>, 2>,
                    Perlin,
                >,
            >,
        >,
        Perlin,
    > = Turbulence::<_, Perlin>::new(rotated_wood)
        .set_seed(2)
        .set_frequency(2.0)
        .set_power(1.0 / 64.0)
        .set_roughness(4);

    let planar_texture = PlaneMapBuilder::new(final_wood)
        .set_size(1024, 1024)
        .build();

    // Create a wood palette.
    let wood_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.000, [189, 94, 4, 255])
        .add_gradient_point(0.500, [144, 48, 6, 255])
        .add_gradient_point(1.0, [60, 10, 8, 255]);

    let mut renderer = ImageRenderer::new().set_gradient(wood_gradient);

    //  utils::write_image_to_file(&renderer.render(&planar_texture), "texture_wood_planar.png");

    let image = &renderer.render(&planar_texture);
    noise_image_to_image(image)
}

pub fn generate_jade() -> Image {
    // Primary jade texture. The ridges from the ridged-multifractal function
    // produces the veins.
    let primary_jade = RidgedMulti::<Perlin>::new(0)
        .set_frequency(2.0)
        .set_lacunarity(2.20703125)
        .set_octaves(6);

    // Base of the secondary jade texture. The base texture uses concentric
    // cylinders aligned on the z axis, which will eventually be perturbed.
    let base_secondary_jade = Cylinders::new().set_frequency(2.0);

    // Rotate the base secondary jade texture so that the cylinders are not
    // aligned with any axis. This produces more variation in the secondary
    // jade texture since the texture is parallel to the y-axis.
    let rotated_base_secondary_jade =
        RotatePoint::new(base_secondary_jade).set_angles(90.0, 25.0, 5.0, 0.0);

    // Slightly perturb the secondary jade texture for more realism.
    let perturbed_base_secondary_jade = Turbulence::<_, Perlin>::new(rotated_base_secondary_jade)
        .set_seed(1)
        .set_frequency(4.0)
        .set_power(1.0 / 4.0)
        .set_roughness(4);

    // Scale the secondary jade texture so it makes a small contribution to the
    // final jade texture.
    let secondary_jade: ScaleBias<_, Turbulence<RotatePoint<Cylinders>, Perlin>, 2> =
        ScaleBias::new(perturbed_base_secondary_jade)
            .set_scale(0.25)
            .set_bias(0.0);

    // Add the two jade textures together. These two textures were produced
    // using different combinations of coherent noise, so the final texture
    // will have a lot of variation.
    let combined_jade = Add::new(primary_jade, secondary_jade);

    // Finally, perturb the combined jade texture to produce the final jade
    // texture. A low roughness produces nice veins.
    let final_jade = Turbulence::<_, Perlin>::new(combined_jade)
        .set_seed(2)
        .set_frequency(4.0)
        .set_power(1.0 / 16.0)
        .set_roughness(2);

    let planar_texture = PlaneMapBuilder::new(&final_jade)
        .set_size(1024, 1024)
        .build();

    // let seamless_texture = PlaneMapBuilder::new(final_jade)
    //     .set_size(1024, 1024)
    //     .set_is_seamless(true)
    //     .build();

    // Create a jade palette.
    let jade_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.000, [24, 146, 102, 255])
        .add_gradient_point(0.000, [78, 154, 115, 255])
        .add_gradient_point(0.250, [128, 204, 165, 255])
        .add_gradient_point(0.375, [78, 154, 115, 255])
        .add_gradient_point(1.000, [29, 135, 102, 255]);

    let mut renderer = ImageRenderer::new().set_gradient(jade_gradient);

    let image = &renderer.render(&planar_texture);
    noise_image_to_image(image)
}

pub fn basic() -> Image {
    let perlin = Perlin::default();
    let turbulence = Turbulence::<_, Perlin>::new(perlin);
    let noisemap =
        PlaneMapBuilder::<noise::Turbulence<noise::Perlin, noise::Perlin>, 2>::new(turbulence)
            .set_size(1024, 1024)
            .set_is_seamless(true)
            .build();

    // let hybrid_multi = HybridMulti::<Perlin>::default();
    // let noisemap = PlaneMapBuilder::<noise::HybridMulti<noise::Perlin>, 2>::new(hybrid_multi)
    //     .set_size(1024, 1024)
    //     .set_is_seamless(true)
    //     .build();

    // let basicmulti = BasicMulti::<Perlin>::default();
    // let noisemap = PlaneMapBuilder::<noise::BasicMulti<noise::Perlin>, 2>::new(basicmulti)
    //     .set_size(1024, 1024)
    //     .set_is_seamless(true)
    //     .build();

    // Create a jade palette.
    let land_gradient = ColorGradient::new().build_terrain_gradient();
    // .clear_gradient()
    // .add_gradient_point(-1.000, COLOR_DARK_BLUE)
    // .add_gradient_point(-0.25, COLOR_DARKER_BLUE)
    // .add_gradient_point(-0.25, COLOR_DARKER_GREEN)
    // .add_gradient_point(0.0, COLOR_DARKEST_GREEN)
    // .add_gradient_point(0.0, COLOR_LIGHT_GREY)
    // .add_gradient_point(0.5, COLOR_GREY);

    let mut renderer = ImageRenderer::new().set_gradient(land_gradient);

    let image = &renderer.render(&noisemap);
    noise_image_to_image(image)
}

pub fn noise_image_to_image(ni: &NoiseImage) -> Image {
    let size = ni.size();
    let mut out: Vec<[u8; 4]> = vec![];
    for qwe in ni.iter() {
        out.push(*qwe);
    }

    (out, size)
}
