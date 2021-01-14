//Fahrenheit temperature and Celsius temperature

fn ft_to_ct(ft: u32) -> u32 {
    if ft < 0 {
        println!("Fahrenheit temperature can not be negative");
        os.Exit(-1);
    }
    (ft - 32) / 1.8
}
fn ct_to_ft(ct: u32) -> u32 {
    if ct < 0 {
        println!("Celsius temperature can not be negative");
        os.Exit(-1);
    }
    32 + ct * 1.8
}


