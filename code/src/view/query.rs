use crate::view::view::AlbumViewData;

#[derive(Clone, Debug)]
pub struct usr_query {
    pub query : Vec<String>,
} 

/// Genera una estructura usr_query a partir de un strig.
/// separa la cadena si se encuentran los caracteres "&"
/// o ",".
/// 
/// #Arguments
/// search: string que se usara para crear la estructura
/// usr_query.
pub fn query_generator(search: &str) -> usr_query {
    let tokens = search
        .split(|c| c == '&' || c == ',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    
    usr_query { query: tokens }
}
