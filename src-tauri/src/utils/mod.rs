use regex::Regex;

pub fn is_valid_phone(phone: &str) -> bool {
    if phone.len() == 0 {
        return true;
    }
    /*
     Matches formats:
     123-456-7890
     (123) 456-7890
     123.456.7890
     123 456 7890
     (123)-456-7890
     1234567890
     +1 (123) 456-7890
     +1-123-456-7890
     1-123-456-7890
    */

    let phone_regex =
        Regex::new(r"^(?:\+?1[-. ]?)?\(?([0-9]{3})\)?[-. ]?([0-9]{3})[-. ]?([0-9]{4})$").unwrap();
    phone.is_empty() || phone_regex.is_match(phone)
}

pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email.is_empty() || email_regex.is_match(email)
}

pub fn is_valid_state(state: &str) -> bool {
    let valid_states = [
        "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA",
        "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ",
        "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VT",
        "VA", "WA", "WV", "WI", "WY",
    ];

    state.is_empty() || valid_states.contains(&state.to_uppercase().as_str())
}

/*pub fn is_valid_image(extension: &str) -> bool {
    let valid_extensions = ["png", "jpg", "jpeg", "gif", "webp", "svg"];
    valid_extensions.contains(&extension)
}*/
