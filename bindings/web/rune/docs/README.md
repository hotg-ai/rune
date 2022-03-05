@hotg-ai/rune / [Exports](modules.md)

# Rune Web Bindings

A package that lets you run Runes in the browser.

## Getting Started

The easiest way to get started is by following [*Lesson 4: Integrating With The
Browser*][lesson-4] from our tutorial series.

This will walk you through creating a React application which initializes the
Rune runtime and executes it every time a button is pressed.

## Contributing

See [the Rune project's `CONTRIBUTING.md` file][contributing] for tips on how
to start hacking on Rune.

### Releasing

Use the `yarn release` script when publishing this package to NPM.

This script will copy `package.json` into the `dist/` folder and makes sure to
run `yarn publish` from there. By doing this, we avoid mentioning `dist/` in the
import path (i.e. you import from `@hotg-ai/rune/builtins` instead of
`@hotg-ai/rune/dist/builtins`).

As a precaution, the `package.json` in this folder sets `"private": true` to
make sure you don't accidentally run `yarn publish` 

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.md) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT.md) or
   http://opensource.org/licenses/MIT)

at your option.

[lesson-4]: https://hotg.dev/docs/tutorials/lesson-4/README
[contributing]: https://github.com/hotg-ai/rune/blob/master/CONTRIBUTING.md
