# Tinkering
A Repository containing Rust projects.

# Projects

### Suggestinator
Discord Bot to pin (save) movie suggestions from server members.
If the name requested by the user is not enough to single out an existing movie, the Bot shows a list of movies for the user to select the correct movie.
- Uses imdb's API to search, and save a movie id.
- Uses MongoDB to save the movie id, the member that suggested it, the amount of upvotes that this suggestion received from the server members, and the date to be viewed.

### Minimal OS
Very minimalistic kernel built with Rust.
Uses BootImage Create as the tool to create the boot image disk that can then be booted up using KVM or other virtual machine engine.
