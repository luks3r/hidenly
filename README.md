# Hidenly

Hide your secrets within your messages. The algorithm converts any text into invisible Unicode characters and hides it inside any message. It is also capable of hiding images!


## Features

- Encode text and images inside any string
- Decode the string into a hidden message or an image
- Share encoded messages by any modern messenger
- Open-source (yaaaay!)
- Uses WASM to provide the best performance

## What it currently lacks?

- **The current implementation only allows images under 10-20 kilobytes.** Multiple megabytes of image raw binary is hard to hide within simple messages like "hello" and results in a few thousand character string "hello" (invisible Unicode...).
- **Some messengers might still not allow or support a few Unicode characters.** There are a limited amount of invisible Unicode characters available and it is hard to make every messenger support it.

## Building

To create a production release, build with:
```bash
npm run build
```

For development use following commands:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Notes

Thanks to [Steganographr](https://github.com/neatnik/steganographr) project for the inspiration for the project
## License

Licensed under [MIT License](LICENSE)
