# Ferris-Bot

---

A Discord bot that could be coming to a server near you

## How to run Ferris-Bot

### ENV Variables
- DISCORD_TOKEN (How the bot authenticates to discord)
- DEBUG (Only set to FALSE in production)
- CHANNEL (In production the only channel the bot will respond in)

I recommend you put your ENV variables in a file called .env

### Docker

In the root of the project run
```
docker build -t ferris-bot .
```

then 

```
docker run --env-file .env ferris-bot
```
