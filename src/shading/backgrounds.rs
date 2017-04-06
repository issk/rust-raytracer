use structures::vec3::Vec3;
use structures::ray::Ray;

pub fn gen_lerp(origin: Vec3, lower_left_corner: Vec3,
        horizontal: Vec3, vertical: Vec3,
        nx: u64, ny: u64) -> Vec<Vec<Vec3>> {
    let mut bg: Vec<Vec<Vec3>> = Vec::new();
    for y in (0..(ny-1)).rev() {
        let mut row: Vec<Vec3> = Vec::new();
        for x in 0..nx {
            let u: f64 = x as f64/nx as f64;
            let v: f64 = y as f64/ny as f64;
            let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col: Vec3 = lerp(&r);
            row.push(col);
        }
        bg.push(row);
    }
    bg
}

fn lerp(r: &Ray) -> Vec3 {
    let unit_direction: Vec3 = r.direction().unit_vector();
    let t: f64 = 0.5*(unit_direction.y() + 1.0);
    (1.0 - t) * Vec3{elements:[1.0, 1.0, 1.0]} + t * Vec3{elements:[0.5, 0.7, 1.0]}
}

#[test]
fn test_lerp () {
    let vec: Vec<Vec<Vec3>> = gen_lerp(Vec3{elements:[0.0, 0.0, 0.0]},
                                       Vec3{elements:[-2.0, -1.0, -1.0]},
                                       Vec3{elements:[4.0, 0.0, 0.0]},
                                       Vec3{elements:[0.0, 2.0, 0.0]},
                                       10, 5);
    let res = [[Vec3 { elements: [0.7277282298406313, 0.8366369379043788, 1.0] },
Vec3 { elements: [0.7236476861652635, 0.8341886116991581, 1.0] },
Vec3 { elements: [0.7182499682499524, 0.8309499809499714, 1.0] },
Vec3 { elements: [0.711424162509477, 0.8268544975056862, 1.0] },
Vec3 { elements: [0.7043564535412361, 0.8226138721247417, 1.0] },
Vec3 { elements: [0.700970966215454, 0.8205825797292724, 1.0] },
Vec3 { elements: [0.7043564535412361, 0.8226138721247417, 1.0] },
Vec3 { elements: [0.711424162509477, 0.8268544975056862, 1.0] },
Vec3 { elements: [0.7182499682499524, 0.8309499809499714, 1.0] },
Vec3 { elements: [0.7236476861652635, 0.8341886116991581, 1.0] }],
[Vec3 { elements: [0.7722717701593687, 0.8633630620956212, 1.0] },
Vec3 { elements: [0.7763523138347365, 0.8658113883008418, 1.0] },
Vec3 { elements: [0.7817500317500476, 0.8690500190500285, 1.0] },
Vec3 { elements: [0.788575837490523, 0.8731455024943138, 1.0] },
Vec3 { elements: [0.7956435464587639, 0.8773861278752583, 1.0] },
Vec3 { elements: [0.799029033784546, 0.8794174202707276, 1.0] },
Vec3 { elements: [0.7956435464587639, 0.8773861278752583, 1.0] },
Vec3 { elements: [0.788575837490523, 0.8731455024943138, 1.0] },
Vec3 { elements: [0.7817500317500476, 0.8690500190500285, 1.0] },
Vec3 { elements: [0.7763523138347365, 0.8658113883008418, 1.0] }],
[Vec3 { elements: [0.814790131918602, 0.8888740791511611, 1.0] },
Vec3 { elements: [0.8257614408414158, 0.8954568645048495, 1.0] },
Vec3 { elements: [0.8396421457000796, 0.9037852874200477, 1.0] },
Vec3 { elements: [0.8560660171779821, 0.9136396103067893, 1.0] },
Vec3 { elements: [0.8716660658480719, 0.9229996395088431, 1.0] },
Vec3 { elements: [0.8786239388568816, 0.927174363314129, 1.0] },
Vec3 { elements: [0.8716660658480719, 0.9229996395088431, 1.0] },
Vec3 { elements: [0.8560660171779821, 0.9136396103067893, 1.0] },
Vec3 { elements: [0.8396421457000796, 0.9037852874200477, 1.0] },
Vec3 { elements: [0.8257614408414158, 0.8954568645048495, 1.0] }],
[Vec3 { elements: [0.8520620726159658, 0.9112372435695795, 1.0] },
Vec3 { elements: [0.8670732264477117, 0.920243935868627, 1.0] },
Vec3 { elements: [0.884790966504298, 0.9308745799025787, 1.0] },
Vec3 { elements: [0.9038643637241659, 0.9423186182344995, 1.0] },
Vec3 { elements: [0.9201034543599429, 0.9520620726159656, 1.0] },
Vec3 { elements: [0.9267766952966369, 0.9560660171779821, 1.0] },
Vec3 { elements: [0.9201034543599429, 0.9520620726159656, 1.0] },
Vec3 { elements: [0.9038643637241659, 0.9423186182344995, 1.0] },
Vec3 { elements: [0.884790966504298, 0.9308745799025788, 1.0] },
Vec3 { elements: [0.8670732264477117, 0.920243935868627, 1.0] }]].to_vec();
    assert_eq!(vec, res);
}