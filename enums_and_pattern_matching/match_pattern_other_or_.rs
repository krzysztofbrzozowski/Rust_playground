// For the first two arms, the patterns are the literal values 3 and 7.
// For the last arm that covers every other possible value, the pattern is the variable we’ve chosen to name other. 
// The code that runs for the other arm uses the variable by passing it to the move_player function.
fn samle_0() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

// Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all 
// pattern: _ is a special pattern that matches any value and does not bind to that value. 
// This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
fn sample_1() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}