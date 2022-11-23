
#[derive(Debug)]
pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl TryFrom<&str> for Present {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split("x").collect::<Vec<&str>>();

        return Ok(Present {
            length: parts.get(0).unwrap().parse().unwrap(),
            width: parts.get(1).unwrap().parse().unwrap(),
            height: parts.get(2).unwrap().parse().unwrap(),
        });
    }
}

impl Present {
    pub fn calculate_amount_of_wrapping_paper(&self) -> u32 {
        let mut total = 0;
        
        let side1 = self.length * self.width;
        let side2 = self.width * self.height;
        let side3 = self.length * self.height;

        total += 2 * side1;
        total += 2 * side2;
        total += 2 * side3;

        if side1 < side2 && side1 < side3 {
            total += side1;
        } else if side2 < side3 {
            total += side2;
        } else {
            total += side3;
        }

        total
    }

    pub fn calculate_amount_of_ribbon(&self) -> u32 {
        let mut total = 0;
        
        if self.length > self.width && self.length > self.height {
            total += 2 * (self.width + self.height);
        } else if self.width > self.height {
            total += 2 * (self.length + self.height);
        } else {
            total += 2 * (self.length + self.width);
        }

        total += self.length * self.width * self.height;

        total
    }
}