fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let mut maxi=input[0];
    let mut mini=input[0];
    for c in input.iter()
    {
        
        maxi = std::cmp::max(maxi,*c);
        mini = std::cmp::min(mini,*c);

    }

    println!("{} is largest and {} is smallest",maxi,mini);
}
