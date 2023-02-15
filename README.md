# 🚀 Sessionizer - Brute force secret keys for sessions 

Sessionizer is a fast and efficient tool written in Rust that allows you to brute-force weak secret keys for sessions. With Sessionizer, you can easily recover the secret keys for your Flask, JwT, and soon more frameworks and libraries.

## Features

- Fast and efficient secret key brute-forcing algorithm
- Easy-to-use command-line interface
- Supports multiple web frameworks/libraries (Flask, JwT, soon others)
- Cross-platform compatibility (works on Windows, macOS, and Linux)

## How it works

Sessionizer works by generating all possible combinations of characters for a given key length and using them to try to decrypt the encrypted session data. This approach is known as a brute-force attack and can be very effective against weak secret keys. The algorithm used by Sessionizer is optimized for speed and can quickly generate and test thousands of keys per second, making it a very fast and efficient tool.

## Installation

Sessionizer is easy to install and use. Simply download the appropriate binary from the [Releases](https://github.com/DEMON1A/Sessionizer/releases) page for your operating system, and you're ready to go!

> NOTE: if you don't find the required binary for your system in the releases just clone sessionizer and compile it locally using rust 

## Usage

Using Sessionizer is easy. Simply run the `sessionizer` binary with the appropriate command-line arguments:

```sh
$ sessionizer -s session-data -w wordlist.txt -f flask
```

- `-s`, `--session` specifies the encrypted session data
- `-w`, `--wordlist` specifies the path to the wordlist file containing the keys to try
- `-f`, `--framework` specifies the web framework being used (currently only Flask and JwT is supported)
- `-v`, `--verbose` enables verbose mode that shows the process details
- `--silent` disables printing the banner and anything unrelated to the results

Sessionizer will start the brute-force attack and display the secret key if it is found. If the key is not found, Sessionizer will display a message indicating that the key was not found and suggest trying a different wordlist or checking the framework.

> NOTE: Using verbose mode disallows sessionizer from using threads because of the implementation of the progress bar, that might affect your program performance based on the size of the wordlist, while using large wordlists it's not recommended to use the verbose mode at all 

## Contributing

We welcome contributions from the community! If you have a bug report, feature request, or would like to contribute code, please submit an issue or pull request on the [GitHub repository](https://github.com/DEMON1A/Sessionizer).

## License

Sessionizer is released under the [MIT License](https://opensource.org/licenses/MIT). See the LICENSE file for more details.

---

Thank you for checking out Sessionizer! We hope it helps you recover your lost session keys and makes your web development work a little bit easier.



