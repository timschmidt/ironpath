fn main() {
    // 1) Create a sample geometry (a cube).
    let csg_cube = CSG::cube(Some((&[0.0, 0.0, 0.0], &[10.0, 10.0, 10.0])));

    // 2) Instantiate the slicer objects.
    let additive_slicer = AdditiveToolpathGenerator;
    let subtractive_slicer = SubtractiveToolpathGenerator;

    // 3) Prepare configurations
    let additive_cfg = AdditiveConfig {
        layer_height: 1.0,
        min_z: 0.0,
        max_z: 10.0,
    };
    let subtractive_cfg = SubtractiveConfig {
        step_down: 2.0,
        min_z: 0.0,
        max_z: 10.0,
    };

    // 4) Generate toolpaths
    let additive_paths = additive_slicer.generate_toolpaths(&csg_cube, &additive_cfg);
    println!("Additive paths: {:?}", additive_paths);

    let subtractive_paths = subtractive_slicer.generate_toolpaths(&csg_cube, &subtractive_cfg);
    println!("Subtractive paths: {:?}", subtractive_paths);

    // From here, we'll:
    // - Convert the `ToolpathSet` into actual G-code,
    // - Apply tool compensation, feed rates, etc.
    // - Possibly visualize or analyze the paths.
}
