use rand::seq::{IteratorRandom, SliceRandom as _};

const UPPER: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &str = "abcdefghijkmnpqrstuvwxyz";
const DIGIT: &str = "0123456789";
const SYMBOL: &str = "!#$%&*+-?@^_|~";

pub async fn process_genpass(
    length: usize,
    include_upper: bool,
    include_lower: bool,
    include_digit: bool,
    include_symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut pool = String::new();
    let mut password_chars = Vec::new();

    if include_upper {
        pool.push_str(UPPER);
        password_chars.push(UPPER.chars().choose(&mut rng).unwrap());
    }
    if include_lower {
        pool.push_str(LOWER);
        password_chars.push(LOWER.chars().choose(&mut rng).unwrap());
    }
    if include_digit {
        pool.push_str(DIGIT);
        password_chars.push(DIGIT.chars().choose(&mut rng).unwrap());
    }
    if include_symbol {
        pool.push_str(SYMBOL);
        password_chars.push(SYMBOL.chars().choose(&mut rng).unwrap());
    }
    if pool.is_empty() {
        return Err(anyhow::anyhow!("no characters to choose from"));
    }

    while password_chars.len() < length {
        password_chars.push(pool.chars().choose(&mut rng).unwrap());
    }

    password_chars.shuffle(&mut rng);
    let password = password_chars.iter().collect::<String>();

    Ok(password)
}
