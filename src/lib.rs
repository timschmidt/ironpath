#![allow(dead_code)]
#![allow(unused_imports)]
#![forbid(unsafe_code)]

use csgrs::float_types::{PI, Real};
use nalgebra::{Point3, Vector3};
use csgrs::polygon::Polygon;
use csgrs::vertex::Vertex;
use csgrs::plane::Plane;

type CSG = csgrs::csg::CSG<()>;

/// A simplified structure representing a toolpath as polylines in 3D.
/// In more advanced designs, you might store feed rates, speeds, 
/// tool orientation, or arcs, etc.
#[derive(Debug, Clone)]
pub struct ToolpathSegment {
    pub points: Vec<Point3<Real>>,
}

/// A collection of toolpaths (e.g. for each layer in additive, or each pass in subtractive).
#[derive(Debug, Clone)]
pub struct ToolpathSet {
    pub segments: Vec<ToolpathSegment>,
}

/// A common trait for any toolpath generator, taking a CSG and producing a set of paths.
pub trait ToolpathGenerator {
    type Config;
    
    /// Primary entry point to produce toolpaths.
    fn generate_toolpaths(&self, model: &CSG, config: &Self::Config) -> ToolpathSet;
}

/// Configuration for additive manufacturing (3D printing).
#[derive(Debug, Clone)]
pub struct AdditiveConfig {
    pub layer_height: Real,
    pub min_z: Real,
    pub max_z: Real,
    // You could add nozzle diameter, infill %, speeds, etc.
}

/// Configuration for subtractive manufacturing (CNC).
#[derive(Debug, Clone)]
pub struct SubtractiveConfig {
    pub step_down: Real,
    pub min_z: Real,
    pub max_z: Real,
    // You could add tool diameter, offset strategies, step-over, etc.
}

/// Toolpath generator for additive layer-based slicing.
pub struct AdditiveToolpathGenerator;

impl ToolpathGenerator for AdditiveToolpathGenerator {
    type Config = AdditiveConfig;

    fn generate_toolpaths(&self, model: &CSG, cfg: &AdditiveConfig) -> ToolpathSet {
        let mut all_segments = Vec::new();
        
        // 1) We iterate over z-layers from min_z up to max_z in increments of cfg.layer_height
        let mut z = cfg.min_z;
        while z <= cfg.max_z + 1e-7 {
            // 2) Slice the CSG with a plane z=0, but we first translate the model 
            //    so that plane is at `z` in the original coordinate system.
            //    Then we call `project(cut_at_z0=true)` to get the cross-section.
            //    Another approach is to transform a plane, but here we cheat with a translation.
            
            // Translate the model by (0,0, -z) so that the plane z=0 cuts at original z= your layer.
            let model_shifted = model.translate(Vector3::new(0.0, 0.0, -z));
            // Now slice/cut at z=0
            let cross_section = model_shifted.slice(Plane { normal: Vector3::z(), w: 0.0 });
            
            // 3) Convert cross-section polygons into polylines.
            //    Each polygon is in Z=0 after slicing. We'll then translate back up by +z.
            for poly in &cross_section.polygons {
                if poly.vertices.len() < 3 {
                    continue;
                }
                
                // Convert the polygon (assumed planar at z=0) to a 2D polyline
                let pline2d = poly.to_polyline();
                // Then convert that 2D polyline to a 3D path at z
                let mut points_3d = Vec::new();
                for v2d in pline2d.vertex_data {
                    points_3d.push(Point3::new(v2d.x, v2d.y, z));
                }
                // Form a path segment
                all_segments.push(ToolpathSegment {
                    points: points_3d,
                });
            }

            z += cfg.layer_height;
        }
        
        ToolpathSet {
            segments: all_segments,
        }
    }
}

/// Toolpath generator for subtractive z-level (very naive approach).
pub struct SubtractiveToolpathGenerator;

impl ToolpathGenerator for SubtractiveToolpathGenerator {
    type Config = SubtractiveConfig;

    fn generate_toolpaths(&self, model: &CSG, cfg: &SubtractiveConfig) -> ToolpathSet {
        let mut all_segments = Vec::new();

        // Example approach:
        // We'll produce "contour passes" at multiple Z levels. 
        // Real CNC often does waterline offsets or more advanced strategies.

        let mut z = cfg.max_z;
        // Move downward in step_down increments
        while z >= cfg.min_z - 1e-7 {
            // "Contour" at this Z means: 
            //  1) Intersect the part with plane z in the same manner as additive. 
            //  2) Possibly offset outward by tool radius to get a cutting path, etc.
            // For simplicity, just show the direct cross-section.

            let model_shifted = model.translate(Vector3::new(0.0, 0.0, -z));
            let cross_section = model_shifted.slice(Plane { normal: Vector3::z(), w: 0.0 });

            for poly in &cross_section.polygons {
                if poly.vertices.len() < 3 {
                    continue;
                }
                let pline2d = poly.to_polyline();
                let mut points_3d = Vec::new();
                for v2d in pline2d.vertex_data {
                    points_3d.push(Point3::new(v2d.x, v2d.y, z));
                }
                all_segments.push(ToolpathSegment {
                    points: points_3d,
                });
            }

            z -= cfg.step_down;
        }

        ToolpathSet {
            segments: all_segments,
        }
    }
}
