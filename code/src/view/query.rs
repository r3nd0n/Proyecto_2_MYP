#[derive(Clone, Debug)]
pub struct UsrQuery {
    pub raw: String,
}

/// Genera una estructura UsrQuery a partir de un string ingresado por le usuario.
/// Los bloques se separan con "&" (como AND) o "," (como OR) 
/// implementado en /model/interpreter.rs.
pub fn query_generator(search: &str) -> UsrQuery {
    UsrQuery {
        raw: search.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_generator_keeps_empty_input() {
        let query = query_generator("");
        assert_eq!(query.raw, "");
    }

    #[test]
    fn query_generator_preserves_raw_expression() {
        let input = "ar: red hot chili peppers & al: californication, ca: scar tissue";
        let query = query_generator(input);

        assert_eq!(query.raw, input);
    }
}