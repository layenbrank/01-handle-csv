use rand::seq::SliceRandom;

// const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+";

const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8] = b"0123456789";
const SPECIALS: &[u8] = b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn generate_password(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    special: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        password.push(
            *UPPERCASE
                .choose(&mut rng)
                .expect("UPPERCASE won't be empty in this context"),
        );
    }
    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        password.push(
            *LOWERCASE
                .choose(&mut rng)
                .expect("LOWERCASE won't be empty in this context"),
        );
    }
    if digits {
        chars.extend_from_slice(DIGITS);
        password.push(
            *DIGITS
                .choose(&mut rng)
                .expect("DIGITS won't be empty in this context"),
        );
    }
    if special {
        chars.extend_from_slice(SPECIALS);
        password.push(
            *SPECIALS
                .choose(&mut rng)
                .expect("SPECIALS won't be empty in this context"),
        );
    }

    for _ in 0..(length - password.len() as u8) {
        // password.push(chars[rng.gen_range(0..chars.len())].clone() );
        let char = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*char);
    }
    password.shuffle(&mut rng);

    //TODO make sure the password has at least one of each type

    println!("{}", String::from_utf8(password)?);
    Ok(())
}
