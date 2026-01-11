# zyAnswers

This program scrapes the correct answers for participation activity questions in a zyBooks section.

The types of questions currently supported are:
- Multiple Choice
- Short Answer

## How it works
When you load a zybooks section, zybooks makes a get request to one of its APIs which contains every single question and answer on the page in a single JSON object. This program simply makes that get request for itself and filters out the answers from the JSON data.

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


## Binaries
I only provide pre-compiled binaries for GNU/Linux distributions because I want Windows users to get a taste of having to compile programs while other platforms get binaries for free. I might change my mind in the future if this program gets advanced enough
