pub struct Triangle<T> {
    sides: [T; 3]
}

impl<T> Triangle<T>
where 
    T: Copy + PartialOrd + PartialEq + std::ops::Add<Output = T> + From<u8>
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let [a, b, c] = sides;

        let no_zeros = sides.iter().all(|&x| x != 0.into());

        // Check the condition for each number.
        let valid_sides = a <= b + c &&
        b <= a + c &&
        c <= a + b;

        println!("{}", valid_sides);

        if no_zeros && valid_sides {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().all(|&x| x == self.sides[0])
    }

    pub fn is_scalene(&self) -> bool {
        let [a, b, c] = self.sides;
        a != b && b != c && a != c
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b || a == c || b == c 
    }
}
