# covid19-mt-bot

A telegram bot that sends an automated message for COVID19 Malta's statistics.

## Setup
1. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:abcabcabc`
2. Get the chat/channel id from [here](https://codesandbox.io/s/get-telegram-chat-id-q3qkk)
3. Paste the bot token in ``TELOXIDE_TOKEN`` and also paste in the ``CHANNEL_ID`` in the ``.env-example``
4. Rename ``.env-example`` to ``.env``
5. Run ``cargo build --release`` (if not deploying via docker)
6. Run the executable in ``target/release/covid19-mt-bot`` (if not deploying via docker)

### Deploy via Docker
7.1. After you have set up your enviroment, build the docker image by running ``docker build -t covid19-mt-bot .``
7.2. Once you have built the docker image, create a new container by running ``docker run -d covid19-mt-bot``
