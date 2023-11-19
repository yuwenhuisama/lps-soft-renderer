use crate::lps::common::math::vec4::Vec4;
use crate::lps::common::{color::Color, math::vec2::Vec2};
use crate::lps::rasterize::vt_output::VertexShaderOutputPositionAndLerp;

use super::render_target::RenderTarget;

pub struct RenderUtil;

impl RenderUtil {
    pub fn draw_triangle<Vertex, F>(
        render_target: &mut RenderTarget,
        v0: &Vertex,
        v1: &Vertex,
        v2: &Vertex,
        mut get_color: F,
    ) where
        Vertex: VertexShaderOutputPositionAndLerp,
        F: FnMut(&Vertex, &Vertex, f32) -> Vec4,
    {
        let mut arr = [
            (v0.position(), v0),
            (v1.position(), v1),
            (v2.position(), v2),
        ];

        // reorder vertices: v0.y <= v1.y <= v2.y
        if arr[0].0.y > arr[1].0.y {
            (arr[0], arr[1]) = (arr[1], arr[0]);
        }

        if arr[1].0.y > arr[2].0.y {
            (arr[1], arr[2]) = (arr[2], arr[1]);
        }

        if arr[0].0.y > arr[1].0.y {
            (arr[0], arr[1]) = (arr[1], arr[0]);
        }

        if (arr[1].0.y - arr[2].0.y).abs() < f32::EPSILON {
            RenderUtil::draw_down_triangle(
                render_target,
                arr[1].1,
                arr[2].1,
                arr[0].1,
                &mut get_color,
            );
        } else if (arr[1].0.y - arr[0].0.y).abs() < f32::EPSILON {
            RenderUtil::draw_up_triangle(
                render_target,
                arr[1].1,
                arr[0].1,
                arr[2].1,
                &mut get_color,
            );
        } else {
            // split into 2 triangles
            let weight = (arr[2].0.y - arr[1].0.y) / (arr[2].0.y - arr[0].0.y);
            let new_edge = Vertex::lerp(arr[2].1, arr[0].1, weight);

            RenderUtil::draw_up_triangle(
                render_target,
                arr[1].1,
                &new_edge,
                arr[2].1,
                &mut get_color,
            );
            RenderUtil::draw_down_triangle(
                render_target,
                arr[1].1,
                &new_edge,
                arr[0].1,
                &mut get_color,
            );
        }
    }

    fn vec4_to_color(color: &Vec4) -> Color {
        Color::new_rgba(color.x as u8, color.y as u8, color.z as u8, color.w as u8)
    }

    fn draw_scan_line(
        render_target: &mut RenderTarget,
        left_x: i32,
        right_x: i32,
        y: i32,
        color_left: &Vec4,
        color_right: &Vec4,
    ) {
        let mut x0 = left_x;
        let mut x1 = right_x;
        let mut color0 = color_left;
        let mut color1 = color_right;

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut color0, &mut color1);
        }

        let mut draw_x;
        let length = x1 - x0;

        for i in 0..length {
            draw_x = x0 + i;

            let lerp_color = Vec4::lerp(*color0, *color1, i as f32 / length as f32);
            let color = RenderUtil::vec4_to_color(&lerp_color);

            render_target.draw_point(draw_x, y, &color);
        }
    }

    fn draw_up_triangle<Vertex, F>(
        render_target: &mut RenderTarget,
        v0: &Vertex,
        v1: &Vertex,
        v2: &Vertex,
        get_color: &mut F,
    ) where
        Vertex: VertexShaderOutputPositionAndLerp,
        F: FnMut(&Vertex, &Vertex, f32) -> Vec4,
    {
        let p0 = Vec2::new(v0.position().x, v0.position().y);
        let p1 = Vec2::new(v1.position().x, v1.position().y);
        let p2 = Vec2::new(v2.position().x, v2.position().y);

        let mut left = if p0.x > p1.x { p0 } else { p1 };
        let left_v = if p0.x > p1.x { &v0 } else { &v1 };

        let right = if p0.x > p1.x { p1 } else { p0 };
        let right_v = if p0.x > p1.x { &v1 } else { &v0 };

        let top = p2;

        left.x = left.x.floor();

        let dy = (top.y - left.y) as i32;
        let mut curr_y = top.y as i32;

        // lerp from top to bottom
        let mut i = dy;
        while i >= 0 {
            let mut weight = 0.0;
            if dy != 0 {
                weight = i as f32 / dy as f32;
            }

            let mut new_left = Vec2::lerp(left, top, weight);
            let mut new_right = Vec2::lerp(right, top, weight);
            new_left.x = new_left.x.floor();
            new_right.x = (new_right.x + 0.5).floor();

            new_left.y = curr_y as f32;
            new_right.y = curr_y as f32;

            let new_color_left = get_color(*left_v, v2, weight);
            let new_color_right = get_color(*right_v, v2, weight);

            RenderUtil::draw_scan_line(
                render_target,
                new_left.x as i32,
                new_right.x as i32,
                new_left.y as i32,
                &new_color_left,
                &new_color_right,
            );

            i -= 1;
            curr_y -= 1;
        }
    }

    fn draw_down_triangle<Vertex, F>(
        render_target: &mut RenderTarget,
        v0: &Vertex,
        v1: &Vertex,
        v2: &Vertex,
        get_color: &mut F,
    ) where
        Vertex: VertexShaderOutputPositionAndLerp,
        F: FnMut(&Vertex, &Vertex, f32) -> Vec4,
    {
        let p0 = Vec2::new(v0.position().x, v0.position().y);
        let p1 = Vec2::new(v1.position().x, v1.position().y);
        let p2 = Vec2::new(v2.position().x, v2.position().y);

        let mut left = if p0.x > p1.x { p1 } else { p0 };
        let left_v = if p0.x > p1.x { &v1 } else { &v0 };

        let right = if p0.x > p1.x { p0 } else { p1 };
        let right_v = if p0.x > p1.x { &v0 } else { &v1 };

        let bottom = p2;

        left.x = left.x.floor();

        let dy = (left.y - bottom.y) as i32;
        let mut curr_y = left.y as i32;

        // lerp from top to bottom
        let mut i = 0;
        while i < dy {
            let mut weight = 0.0;
            if dy != 0 {
                weight = i as f32 / dy as f32;
            }

            let mut new_left = Vec2::lerp(left, bottom, weight);
            let mut new_right = Vec2::lerp(right, bottom, weight);
            new_left.x = new_left.x.floor();
            new_right.x = (new_right.x + 0.5).floor();

            new_left.y = curr_y as f32;
            new_right.y = curr_y as f32;

            let new_color_left = get_color(*left_v, v2, weight);
            let new_color_right = get_color(*right_v, v2, weight);
            RenderUtil::draw_scan_line(
                render_target,
                new_left.x as i32,
                new_right.x as i32,
                new_left.y as i32,
                &new_color_left,
                &new_color_right,
            );

            i += 1;
            curr_y -= 1;
        }
    }
}
