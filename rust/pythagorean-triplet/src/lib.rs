
// ab + 1000c = 500000
pub fn find() -> Option<u32> {
    for a in 1..1001 {
        for b in 1..1001 {
            for c in 1..1001 {
                if (a+b+c == 1000) && (a*a + b*b == c*c)  {
                    return Some(a*b*c);
                }
            }
        }
    }
    None
}
