use std::str::FromStr;

fn getDict() -> [String; 10] {
    [
        "rzecz".to_string(),
        String::from_str("desko").unwrap(),
        String::from_str("Scheiß").unwrap(),
        String::from_str("ding").unwrap(),
        String::from_str("pospolita").unwrap(),
        String::from_str("tego").unwrap(),
        String::from_str("dla").unwrap(),
        String::from_str("rolka").unwrap(),
        String::from_str("grotten").unwrap(),
        String::from_str("hässlich").unwrap(),
    ]
}

fn main() {
    // d1.into_iter().for_each(|x| println!("{}", x));
    println!();

    let x = is_concat("ScheißScheiß", &getDict());
    let y = is_concat("Scheißrolka", &getDict());
    let z = is_concat("ScheißScheiß", &getDict());

    println! {"{} {} {}", x, y, z};
}

/// # Examples
///
/// ```
/// assert_true();
/// ```
fn is_concat(s: &str, d2: &[String; 10]) -> bool {
    d2.into_iter()
        .filter(|f| {
            d2.into_iter()
                .filter(|elo| elo != f)
                .filter(|ending| (format!("{}{}", f, ending) == s))
                .count()
                > 0
        })
        .count()
        > 0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(false, is_concat("ScheißScheiß", &getDict()));
    }

    #[test]
    fn test_two() {
        assert_eq!(true, is_concat("deskorolka", &getDict()));
    }

}
