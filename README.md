# zyAnswers

This program scrapes the correct answers for participation activity questions in a zyBooks section.

The types of questions currently supported are:
- Multiple Choice
- Short Answer


## Compilation
1. Make sure cargo and rust are installed, then clone this repository.
2. Run `cargo build --release` to compile the project. The binary will be in ./target/release/

If you're on Arch Linux, check the releases section for a PKGBUILD file.

## How to use
1. Compile the program for your device
2. Go to zyBooks and get the following information:
  - your zyBook code (Ex: WebProgrammingR37)
  - the chapter and section you want the answers for (Ex: 7.11 Using third-party web APIs (JavaScript); i.e. chapter 7, section 11)
  - your zyBooks auth token (See [Getting your zyBooks auth token](#getting-your-zybooks-auth-token))

3. Run the binary from the terminal
Ex:
```console
$ zyanswers --auth-token <your token> --chapter <chapter> --section <section> --zybook-code <your zybook code>
```

## Getting your zyBooks auth token

> [!CAUTION]
> DO NOT GIVE ANYBODY YOUR AUTH TOKEN as it is essentially the same as giving them your zyBooks *username* **and** *password*.
> This program only uses your token to send a GET request to `zyserver.zybooks.com` to access the specific chapter and section of your zybook that you requested.
> See [Why no pre-compiled binaries?](#why-no-precompiled-binaries) for more information.

### On Firefox
1. Go to zybooks.com, log-in, and inspect element.
2. Go to the `Storage` tab
3. Navigate into `Local Storage > https://learn.zybooks.com`
4. Click on the cookie that has the key  `ember_simple_auth-session-5`
5. On the right where the data should've appeared, navigate into `authenticated > session > auth_token`
6. The value between the quotes is your auth token.

### On other browsers
1. Look up how to view cookies in depth (the actual keys and values)
2. Navigate to the same cookie described in the Firefox section


## Why no pre-compiled binaries?
You should never trust random programs on the internet with your log-in tokens. It's always a good idea to check the source code and compile yourself to make sure the program does what you expect with your personal details.
By forcing you to compile the program yourself, you're making sure that the binary you get actually came from the unmodified source code, without any malicious code added in.

It's also just a pain in the ass to have to compile each version of the program for every major platform.
