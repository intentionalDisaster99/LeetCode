/*

    Author:     Sam Whitlock
    Date:       March 26, 2025

    Completed?  Yes
    Notes:
        Not technically a LeetCode problem, this one is from CodeWars.
        Also, this is a very simple solution. I just saw it and wanted to finish it.


*/

fn main() {

    println!("{} should be (123) 456-7890", create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));

}


fn create_phone_number(numbers: &[u8]) -> String {
    format!("({}{}{}) {}{}{}-{}{}{}{}", numbers[0],numbers[1], numbers[2],numbers[3],numbers[4],numbers[5], numbers[6], numbers[7], numbers[8], numbers[9])
}
