pub fn is_leap_year(year: u64) -> bool {
    let divisable_by_4 = year % 4 == 0;
    let divisable_by_100 = year % 100 == 0;
    let divisable_by_400 = year % 400 == 0;

    if divisable_by_4 {
        if divisable_by_100 {
            if divisable_by_400 {
                return true;
            }
            return false;
        }
        return true;
    }
    false
}
