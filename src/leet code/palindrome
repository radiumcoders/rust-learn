// check if the number is palindrome or not without cinverting to string

fn main() {
    let num = -12341;
    let mut rev = 0;
    let mut n = num;
    if num < 0 {
        return false;
    } else {
        while n > 0 {
            let rem = n % 10; // extract the last digit
            n = n / 10; // remove the last digit
            rev = rev * 10 + rem; // add the last digit in the reversed
        }
    }
    return num == rev;
}
