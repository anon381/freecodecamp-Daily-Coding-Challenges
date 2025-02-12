// Space Complexity: O(1)
// Time Complexity: O(n)
// Returns the strength value of a character (a-z, A-Z, 0-9)
fn char_strength(c: char) -> i32 {
    match c {
        'a'..='z' => (c as u8 - b'a' + 1) as i32,
        'A'..='Z' => (c as u8 - b'A' + 27) as i32,
        '0'..='9' => c.to_digit(10).unwrap() as i32,
        _ => 0,
    }
}

// Simulates a battle between two armies and returns the result
fn battle(your: &str, opp: &str) -> &'static str {
    if your.len() > opp.len() {
        return "Opponent retreated";
    } else if your.len() < opp.len() {
        return "We retreated";
    }

    let mut our_wins = 0;
    let mut their_wins = 0;

    for (y, o) in your.chars().zip(opp.chars()) {
        let ys = char_strength(y);
        let os = char_strength(o);
        if ys > os {
            our_wins += 1;
        } else if os > ys {
            their_wins += 1;
        }
    }

    if our_wins > their_wins {
        "We won"
    } else if our_wins < their_wins {
        "We lost"
    } else {
        "It was a tie"
    }
}


