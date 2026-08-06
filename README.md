
> [!NOTE]
> This repository was branched out of [storiny/web](https://github.com/storiny/web) which was originally a monolith. If you're looking for the original git history before the split, you can find it over on the original repository.

# Storiny OG (Opengraph)

A fast, lightweight microservice responsible for dynamically generating open graph images for Storiny stories and tags on-the-fly.

## Specimen

| Story | Tag |
| -- | -- |
| <img width="640" alt="Dynamic story opengraph image" src="https://github.com/user-attachments/assets/f85bbfae-2027-48d9-808a-c1acd806e1f7" /> | <img width="640" alt="Dynamic tag opengraph image" src="https://github.com/user-attachments/assets/a52d0cfe-0365-4600-80f2-5d8293fb06cf" /> |

---

## Tech stack

- **Framework**: [Actix Web](https://actix.rs/) serves our HTTP endpoints
- **RPC**: [Tonic](https://github.com/hyperium/tonic) (gRPC) is used to fetch live story and tag metadata directly from the [`storiny/api`](https://github.com/storiny/api) service
- **Templating**: [Sailfish](https://github.com/Kogia-sima/sailfish) provides fast HTML/SVG macro-based templating
- **Rasterization**: [resvg](https://github.com/RazrFalcon/resvg) and [tiny-skia](https://github.com/RazrFalcon/tiny-skia) parse and render the SVG into a PNG
- **Caching**: Redis handles rate limiting

---

## Under the hood

1. **gRPC data fetching**: When a request arrives for a story or tag, the service dispatches a gRPC request to the API microservice
2. **Dynamic text**: Text wrapping and truncation are calculated dynamically using `textwrap` and custom font metric utils to ensures titles and descriptions fit perfectly within the card bounds without bleeding over
3. **SVG templating**: The resolved metadata (cover image, author avatar, title...) is injected into an SVG template using Sailfish
4. **Rasterization**: Finally, `resvg` parses the populated SVG and rasterizes it into a PNG. This raster image is returned to the client with suitable caching with the help of Cloudflare.

---

## API endpoints

- `GET /stories/{id}`: Fetches story metadata via gRPC, renders the story template, and returns a dynamic PNG opengraph image
- `GET /tags/{id}`: Fetches tag metadata via gRPC, renders the tag template, and returns a dynamic PNG opengraph image
- `GET /health`: Health check endpoint. Returns `OK`.

---

## Running locally

Make sure you have Rust installed. You will also need the main `storiny-api` running locally for the gRPC calls to succeed.

```bash
# clone
git clone https://github.com/storiny/og.git
cd og

# install deps and run the service
cargo run
```
