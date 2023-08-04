
use crate::structs::vector2::*;

    pub struct Ray{
        pub startPos: Vector2,
        pub angle: f32,
        pub distance: f32,
    }

    pub mod Raycasters {
        use crate::structs::{vector2::{Vector2, get_distance_from}, WorldObj::{World, Wall}};

        use super::Ray;

        pub fn shootRay(ray: &Ray, world: &World) -> Option<RayHit>{
            let rayEndPos = calculate_end_position(ray);
            let mut bestWall: Option<(Wall, Vector2)> = None;
            for i in 0..world.walls.len() {
                // Uses the line to line intersection formula from: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
                let t_up = (ray.startPos.x-world.walls[i].p1.x)*(world.walls[i].p1.y-world.walls[i].p2.y)-(ray.startPos.y-world.walls[i].p1.y)*(world.walls[i].p1.x-world.walls[i].p2.x);
                //--------------------------------
                let den = (ray.startPos.x-rayEndPos.x)*(world.walls[i].p1.y-world.walls[i].p2.y)-(ray.startPos.y-rayEndPos.y)*(world.walls[i].p1.x-world.walls[i].p2.x);
                let t = t_up/den;

                let u_up = (ray.startPos.x-world.walls[i].p1.x)*(ray.startPos.y-rayEndPos.y)-(ray.startPos.y-world.walls[i].p1.y)*(ray.startPos.x-rayEndPos.x);
                let u = u_up/den;

                if(0.<=t && t<=1.0)&&(0.<=u && u<=1.){
                    let newHitPoint = Vector2{
                        x: ray.startPos.x+t*(rayEndPos.x-ray.startPos.x), // Some more info: https://wikimedia.org/api/rest_v1/media/math/render/svg/47fdf67cdcdd9e9cc4be4ed0bc638c959df90635
                        y: ray.startPos.y+t*(rayEndPos.y-ray.startPos.y)
                    };
                    if let Some((_wall, bestWallHitPoint)) = bestWall{
                        if get_distance_from(bestWallHitPoint, ray.startPos) > get_distance_from(newHitPoint, ray.startPos){
                            bestWall = Some((world.walls[i], newHitPoint));
                        }
                    }else{
                        bestWall = Some((world.walls[i], newHitPoint));
                    }
                }


            }

            if let Some((wall, bestWallHitPoint)) = bestWall{
                return Some( RayHit{
                    wall: Wall { p1: wall.p1, p2: wall.p2},
                    point: bestWallHitPoint,
                    distance: get_distance_from(bestWallHitPoint, ray.startPos),
                });
            }
            None
        }

        fn calculate_end_position(ray: &Ray) -> Vector2 {
            // Convert the angle from degrees to radians
            let angle_radians = ray.angle.to_radians();

            // Calculate the end position of the ray
            let x2 = ray.startPos.x + ray.distance * angle_radians.sin();
            let y2 = ray.startPos.y + ray.distance * angle_radians.cos();

            Vector2 { x: x2, y: y2 }
        }
        pub struct RayHit{
            pub wall: Wall,
            pub point: Vector2,
            pub distance: f32,
        }
    }