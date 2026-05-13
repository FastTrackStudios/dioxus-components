The Avatar component is used to display a user's profile picture or an icon representing the user. It handles the loading state of the image and can display a fallback if the image fails to load.

## Component Structure

```rust
Avatar {
    src: "https://example.com/avatar.png",
    alt: "Jane Doe",
    on_state_change: |state: AvatarState| { /* image is loading/loaded/failed */ },
    "JD"
}
```
