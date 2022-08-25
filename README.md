# spazio-grigio-bot

<p align="center">~ Gli ultimi aggiornamenti di spazio grigio in ogni momento ~</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.2.1 (25/08/2022)</p>

<p align="center">
  <a href="https://opensource.org/licenses/Unlicense"
    ><img
      src="https://img.shields.io/badge/License-Unlicense-teal.svg"
      alt="License-Unlicense"
  /></a>
  <a href="https://github.com/veeso/spazio-grigio-bot/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/spazio-grigio-bot.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/spazio-grigio-bot"
    ><img
      src="https://img.shields.io/crates/d/spazio-grigio-bot.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/spazio-grigio-bot"
    ><img
      src="https://img.shields.io/crates/v/spazio-grigio-bot.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/spazio-grigio-bot/actions"
    ><img
      src="https://github.com/veeso/spazio-grigio-bot/workflows/Build/badge.svg"
      alt="Build CI"
  /></a>
</p>

---

- [spazio-grigio-bot](#spazio-grigio-bot)
  - [About spazio-grigio-bot 📰](#about-spazio-grigio-bot-)
  - [Command API 🐚](#command-api-)
  - [Get started 🏁](#get-started-)
    - [Users](#users)
    - [Developers](#developers)
      - [Deploy with heroku](#deploy-with-heroku)
  - [Support the developer ☕](#support-the-developer-)
  - [Powered by 💪](#powered-by-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About spazio-grigio-bot 📰

spazio-grigio-bot is a Telegram bot to get the latest news from Irina from Spazio Grigio.

---

## Command API 🐚

- `/ciaoirina`

    Subscribe to the irina's newsletter

- `/sialconsumismo`

    Unsubscribe from newsletter

- `/buongiornoirina`

    Get a morning routine video

- `/postminimalista`

    Get latest instagram post from Irina

- `/videominimalista`

    Get latest video from Irina

- `/seratasenzatv`

    Get latest videos from Irina

- `/help`

    Show help

---

## Get started 🏁

### Users

Scan this QR code or go to this URL <https://t.me/spaziogrigio_bot> to start a chat with Spazio grigio bot, then add it to any group or chat directly with him.

![telegram-qr](/docs/images/qr-code.webp)

### Developers

If you want to develop on this bot, you can follow these simple steps:

1. Clone this repository `git clone git@github.com:veeso/spazio-grigio-bot.git`
2. Create your bot with the [Botfather](https://t.me/botfather)
3. Get your API key
4. Set your API key in your environment using the variable `TELOXIDE_TOKEN`
5. Set your database path in your environment using the variable `DATABASE_URI`
6. Touch the database file `touch $DATABASE_URI`
7. Set your email account details in the environment `IMAP_SERVER`, `IMAP_PORT`, `EMAIL_ADDRESS`, `EMAIL_PASSWORD`
8. Set redis url in the environment `REDIS_URL`
9. Set rsshub in the environment `RSSHUB_URL`
10. Run the spazio-grigio bot

#### Deploy with heroku

You can then deploy your own version of the spazio-grigio bot using `heroku`, with these simple steps:

1. Create your heroku app `heroku create --buildpack emk/rust`
2. configure the Telegram API key with `heroku config:set TELOXIDE_TOKEN=<YOUR_API_KEY>`
3. git push heroku main

---

## Support the developer ☕

If you like spazio-grigio-bot and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)
[![bitcoin](https://img.shields.io/badge/Bitcoin-ff9416?style=for-the-badge&logo=bitcoin&logoColor=white)](https://btc.com/bc1qvlmykjn7htz0vuprmjrlkwtv9m9pan6kylsr8w)
[![litecoin](https://img.shields.io/badge/Litecoin-345d9d?style=for-the-badge&logo=Litecoin&logoColor=white)](https://blockchair.com/litecoin/address/ltc1q89a7f859gt7nuekvnuuc25wapkq2f8ny78mp8l)
[![ethereum](https://img.shields.io/badge/Ethereum-3C3C3D?style=for-the-badge&logo=Ethereum&logoColor=white)](https://etherscan.io/address/0xE57E761Aa806c9afe7e06Fb0601B17beC310f9c4)

---

## Powered by 💪

- [feed-rs](https://github.com/feed-rs/feed-rs)
- [teloxide](https://github.com/teloxide/teloxide)
- [tokio](https://tokio.rs/)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve spazio-grigio-bot, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View spazio-grigio-bot's changelog [HERE](CHANGELOG.md)

---

## License 📃

spazio-grigio-bot is licensed under the Unlicense license.

You can read the entire license [HERE](LICENSE)
