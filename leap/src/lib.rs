pub fn is_leap_year(year: u64) -> bool {
    //if year % 4 == 0 {
    //if year % 100 == 0 {
    //if year % 400 == 0 {
    //return true;
    //} else {
    //return false;
    //}
    //} else {
    //return true;
    //}
    //} else {
    //return false;
    //}
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}
