//Fahrenheit temperature and Celsius temperature

pub fn ft_to_ct(ft: u32) -> u32 {
    if ft < 0 {
        println!("Fahrenheit temperature can not be negative");
        os.Exit(-1);
    }
    return (ft - 32) / 1.8;
}
pub fn ct_to_ft(ct: u32) -> u32 {
    if ct < 0 {
        println!("Celsius temperature can not be negative");
        os.Exit(-1);
    }
    return 32 + ct * 1.8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ft_to_ct_test() {
        let ct = ft_to_ct(22);
        println!("CT is {}", ct);
    }

    #[test]
    fn ct_to_ft_test() {
        ct_to_ft(22);
    }

}
