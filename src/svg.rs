#![allow(dead_code)]

use std::fmt::Write as _;

// Draw geometric elements for debug purposes

use std::io;
use std::path::Path;
use std::{fs::File, io::Write};

use robust::{Coord, orient2d};

use crate::prelude::*;

/// Utility for generating SVG output for visualization of geometric operations.
///
/// This struct provides methods to render various geometric primitives (points, lines,
/// arcs, circles) to an SVG file. It's primarily used for debugging and visualizing
/// the results of geometric operations.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let mut svg = SVG::new(200.0, 200.0, Some("/tmp/debug.svg"));
/// // ... render geometric elements
/// svg.write(); // Save to file
/// ```
pub struct SVG {
    // f: File,
    writer: Box<dyn Write>,
    s: String,
    pub xsize: f64,
    pub ysize: f64,
}

impl SVG {
    /// Creates a new SVG context with specified dimensions and output file.
    ///
    /// # Arguments
    ///
    /// * `xsize` - Width of the SVG viewport
    /// * `ysize` - Height of the SVG viewport  
    /// * `file` - Path to the output SVG file (empty string defaults to "/tmp/out.svg")
    ///
    /// # Examples
    ///
    /// ```
    /// use togo::prelude::*;
    ///
    /// let svg = SVG::new(800.0, 600.0, Some("/tmp/visualization.svg"));
    /// ```
    #[inline]
    pub fn new(xsize: f64, ysize: f64, out: Option<&str>) -> Self {
        let out_writer = match out {
            Some(x) => {
                let path = Path::new(x);
                Box::new(File::create(&path).unwrap()) as Box<dyn Write>
            }
            None => Box::new(io::stdout()) as Box<dyn Write>,
        };

        // let f = if file.is_empty() {
        //     File::create("/tmp/out.svg").expect("creation failed")
        // } else {
        //     File::create(file).expect("creation failed")
        // };
        let s = String::new();
        SVG { writer: out_writer, s, xsize, ysize }
    }
}

/// Creates a new SVG context.
///
/// The default output path is "/tmp/out.svg".
/// This is a convenience function equivalent to `SVG::new(xsize, ysize, "")`.
///
/// # Arguments
///
/// * `xsize` - Width of the SVG viewport
/// * `ysize` - Height of the SVG viewport
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let svg_context = svg(400.0, 300.0);
/// ```
#[inline]
#[must_use]
pub fn svg(xsize: f64, ysize: f64) -> SVG {
    SVG::new(xsize, ysize, None)
}

impl SVG {
    pub fn write(&mut self) {
        self.write_stroke_width(0.2);
    }
    /// Writes the SVG content to the output file.
    pub fn write_stroke_width(&mut self, stroke_width: f64) {
        let mut header = String::new();
        write!(
            &mut header,
            r#"<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg" fill="none" stroke-width="{}" stroke-linecap="round">"#,
            self.xsize, self.ysize, stroke_width
        ).unwrap();
        write!(
            &mut header,
            "\n<rect width=\"100%\" height=\"100%\" fill=\"#ffffffff\" />\n"
        )
        .unwrap();
        header.push('\n');

        header.push_str(self.s.as_str());

        header.push_str("</svg>\n");
        self.writer.write_all(header.as_bytes()).expect("write failed");
    }

    pub fn circle(&mut self, circle: &Circle, color: &str) {
        let mut s = String::new();
        write!(
            &mut s,
            r#"<circle cx="{}" cy="{}" r="{}" stroke="{}" />"#,
            circle.c.x,
            self.ysize - circle.c.y,
            circle.r,
            color
        )
        .unwrap();
        self.s.push_str(&s);
        self.s.push('\n');
    }

    pub fn rect(&mut self, rect: &Rect, color: &str) {
        let mut s = String::new();
        let width = rect.p2.x - rect.p1.x;
        let height = rect.p2.y - rect.p1.y;
        write!(
            &mut s,
            r#"<rect x="{}" y="{}" width="{}" height="{}" stroke="{}" />"#,
            rect.p1.x,
            self.ysize - rect.p2.y,
            width,
            height,
            color
        )
        .unwrap();
        self.s.push_str(&s);
        self.s.push('\n');
    }

    pub fn text(&mut self, p: Point, text: &str, color: &str) {
        let mut s = String::new();
        write!(
            &mut s,
            r#"<text x="{}" y="{}" fill="{}" font-size="2.0">{}</text>"#,
            p.x + 0.0,
            self.ysize - p.y + 0.0,
            color,
            text
        )
        .unwrap();
        self.s.push_str(&s);
        self.s.push('\n');
    }

    pub fn segment(&mut self, segment: &Segment, color: &str) {
        let mut s = String::new();
        write!(
            &mut s,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" id="{}"/>"#,
            segment.a.x,
            self.ysize - segment.a.y,
            segment.b.x,
            self.ysize - segment.b.y,
            color,
            segment.id
        )
        .unwrap();
        self.s.push_str(&s);
        self.s.push('\n');
    }

    /// Draws an arc in the SVG format.
    pub fn arc(&mut self, arc: &Arc, color: &str) {
        let mut s = String::new();
        // Side of line test
        // let diff_pa = arc.c - arc.a;
        // let diff_ba = arc.b - arc.a;
        //let perp = diff_pa.perp(diff_ba);
        let pa = Coord {
            x: arc.a.x,
            y: arc.a.y,
        };
        let pb = Coord {
            x: arc.b.x,
            y: arc.b.y,
        };
        let pc = Coord {
            x: arc.c.x,
            y: arc.c.y,
        };

        let large_arc_flag: i32 = (orient2d(pa, pb, pc) < 0.0).into();
        write!(
            &mut s,
            r#"<path d="M {} {} A {} {} {} {} {} {} {}" stroke="{}" id="{}"/>"#,
            arc.a.x,
            self.ysize - arc.a.y,
            arc.r,
            arc.r,
            0,
            large_arc_flag,
            0, // always 0 because arc_circle_parametrization always creates CCW arcs
            arc.b.x,
            self.ysize - arc.b.y,
            color,
            arc.id
        )
        .unwrap();
        self.s.push_str(&s);
        self.s.push('\n');
    }

    /// Draws a vertex in the SVG format.
    pub fn pvertex(&mut self, p0: Point, p1: Point, g: f64, color: &str) {
        if g == 0f64 {
            // line segment
            let seg = segment(p0, p1);
            self.segment(&seg, color);
        } else {
            let arc = arc_from_bulge(p0, p1, g);
            self.arc(&arc, color);
        }
        // self.circle(&circle(p0, 0.5), "blue");
        // self.circle(&circle(p1, 0.5), "blue");
    }

    /// Draws a polyline in the SVG format.
    pub fn polyline(&mut self, pline: &Polyline, color: &str) {
        if pline.len() < 2 {
            return; // Nothing to draw
        }
        let last = pline.len() - 2;
        for i in 0..=last {
            let p0 = pline[i];
            let p1 = pline[i + 1];
            self.pvertex(p0.p, p1.p, p0.b, color);
        }
        // close pline
        let p0 = pline.last().unwrap();
        self.pvertex(p0.p, pline[0].p, p0.b, color);
    }

    pub fn polylines(&mut self, plines: &Vec<Polyline>, color: &str) {
        for p in plines {
            self.polyline(p, color);
        }
    }

    /// Draws an arc segment in the SVG format.
    pub fn arcsegment(&mut self, off: &Arc, color: &str) {
        if off.is_seg() {
            // line segment
            let seg = segment(off.a, off.b);
            self.segment(&seg, color);
        } else {
            self.arc(off, color);
        }
    }

    pub fn arcline_segment_points(&mut self, off: &Arc, _color: &str) {
        self.circle(&circle(off.a, 0.1), "green");
        self.circle(&circle(off.b, 0.1), "green");
        //self.text(off.a, &off.id.to_string(), color);
    }

    pub fn arcline(&mut self, offs: &Arcline, color: &str) {
        for s in offs {
            self.arcsegment(s, color);
        }
    }

    pub fn arcline_single_points(&mut self, offs: &Arcline, color: &str) {
        for s in offs {
            self.arcline_segment_points(s, color);
        }
    }

    pub fn arclines(&mut self, offs: &Vec<Arcline>, color: &str) {
        for s in offs {
            self.arcline(s, color);
        }
    }

    // pub fn polysegment(&mut self, off: &Arc, color: &str) {
    //     if off.is_line() {
    //         // line segment
    //         let seg = segment(off.a, off.b);
    //         self.segment(&seg, color);
    //     } else {
    //         self.arc(off, color);
    //     }
    //     //self.circle(&circle(off.a, 0.3), "green");
    //     //self.circle(&circle(off.b, 0.2), "green");
    // }

    // pub fn arcline(&mut self, vseg: &Vec<Arc>, color: &str) {
    //     for s in vseg.iter() {
    //         self.offset_segment(s, color);
    //     }
    // }

    // pub fn arclines(&mut self, vseg: &Vec<Vec<Arc>>, color: &str) {
    //     for s in vseg.iter() {
    //         self.arcline(s, color);
    //     }
    // }
}

#[cfg(test)]
mod test_svg {

    use super::*;

    #[test]
    fn test_circle_svg_std_out() {
        let mut svg = svg(400.0, 300.0);
        let c = circle(point(200.0, 150.0), 50.0);
        svg.circle(&c, "red");
        svg.write(); // to stdout
    }

    #[test]
    #[ignore = "writes to file, not stdout"]
    fn test_circle_svg_to_file() {
        let mut svg = SVG::new(400.0, 300.0, Some("/tmp/test.svg"));
        let c = circle(point(200.0, 150.0), 50.0);
        svg.circle(&c, "red");
        svg.write(); // to stdout
    }
}
