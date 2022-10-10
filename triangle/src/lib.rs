pub struct Triangle([u64; 3]);

// 是否构成三角形
fn is_triangle(sides: [u64; 3]) -> bool {
    let [a, b, c] = sides;
    let is_all_side_gt_zero = sides.iter().all(|side| side > &0);
    // 边长大于0
    if !is_all_side_gt_zero {
        return false;
    }
    // 任意两边之和大于第三边
    if a + b > c && a + c > b && b + c > a {
        return true;
    }
    false
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        // unimplemented!("Construct new Triangle from following sides: {:?}. Return None if the sides are invalid.", sides);
        if is_triangle(sides) {
            Some(Self(sides))
        } else {
            None
        }
    }

    // 等边
    pub fn is_equilateral(&self) -> bool {
        // unimplemented!("Determine if the Triangle is equilateral.");
        let [a, b, c] = self.0;
        a == b && b == c
    }
    // 不等边
    pub fn is_scalene(&self) -> bool {
        // unimplemented!("Determine if the Triangle is scalene.");
        let [a, b, c] = self.0;
        a != b && a != c && b != c
    }
    // 等腰
    pub fn is_isosceles(&self) -> bool {
        // unimplemented!("Determine if the Triangle is isosceles.");
        let [a, b, c] = self.0;
        a == b || b == c || a == c
    }
}
