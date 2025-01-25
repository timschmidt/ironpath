# ironpath
An additive and subtractive manufacturing toolpath generator with advanced geometry functions

# Trait ToolpathGenerator

    Defines a single method generate_toolpaths(&self, model: &CSG<()>, config: &Self::Config) -> ToolpathSet.
    Each concrete generator (additive or subtractive) has its own Config type.

# AdditiveToolpathGenerator

    Implements a simple layer-based slicing approach:
        We loop from min_z to max_z using layer_height steps.
        For each layer z, we “shift” the model down by z so the plane z=0 cuts right at that slice.
        project(true) obtains cross-section polygons, which we convert to polylines in XY, then shift them back up to z.
        We store each cross-section as a ToolpathSegment.

# SubtractiveToolpathGenerator

    Implements a naive z-level approach in reverse (from top down).
    For each “step_down” layer, we gather cross-sections (again using project(true)) and store them.
    Real CNC code would offset these paths outward (to account for the tool radius), might do additional passes, etc.

# ToolpathSet

    Bundles all the path data. In real software, you’d have a more elaborate data structure with feed/speed, arcs, G-Code parameters, etc.

# Extending

    To add new slicing strategies, we can implement new structs that implement ToolpathGenerator.
    For example, AdaptiveToolpathGenerator or InfillPathGenerator for advanced 3D printing patterns.
    For CNC, we might add “waterline offset” or “spiral finishing.”
    
# Todo
- 2.5D Milling: we can offset polygons at each slice using offset_2d from the library (or a more advanced offset library) to account for tool diameter.
- Infill Generation (Additive): we might transform the cross-sections into line patterns, grids, or honeycombs.
- Multi-Axis CNC: The logic becomes more complex (tilting the tool, dynamic slices, etc.). The overall pattern remains the same: implement a new ToolpathGenerator that enumerates pass surfaces.
- G-Code Export: Create a function to convert a ToolpathSet into textual G-code commands (e.g. G1 X... Y... Z... F...).
- Performance: For large or complex models, we might want to accelerate the slicing with spatial data structures. The Node BSP from the CSG library can help, or we might rely on bounding-volume hierarchies from parry3d to optimize intersection.
- bricklaying layers
- https://en.m.wikipedia.org/wiki/Schwarz_minimal_surface
- Hilbert and space filling curves
- 
