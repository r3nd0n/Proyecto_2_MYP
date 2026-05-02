#[derive(Clone, Debug)]
pub struct UsrQuery {
    pub raw: String,
}

/// Genera una estructura UsrQuery a partir de un string ingresado por le usuario.
/// Los bloques se separan con "&" (como AND) o "," (como OR) 
/// implementado en /model/interpreter.rs.
///
/// #Arguments
/// search: string que se usara para crear la estructura UsrQuery.
pub fn query_generator(search: &str) -> UsrQuery {
    UsrQuery {
        raw: search.to_string(),
    }
}