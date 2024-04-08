/// Returns the thumbnail URL of the image mapped to provided ID.
///
/// * `cdn_server_url` - The URL of the CDN server.
/// * `id` - The public key of the image.
pub fn get_thumbnail_url(cdn_server_url: &str, id: &str) -> String {
    if cfg!(test) {
        // Return the ID during tests.
        id.to_string()
    } else {
        format!("{cdn_server_url}/thumb/{id}")
    }
}
