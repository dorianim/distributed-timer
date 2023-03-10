<h1 align="center">
    distributed-timer
</h1>

A simple, distributed timer, originally designed for bouldering for competitions.

# Features

- Create timers with custom sequences
- Indication sound at one minute and a countdown at five seconds
- Show the timer on many devices
- Time is synced accurately even when the clock of the devices is out of sync
- set custom background colors
- change color or labels without restarting the timer
- start the timer at a scheduled point of time in the future

## Screenshots

<table align="center">
    <tr>
        <td align="center">
            <a href="https://raw.githubusercontent.com/dorianim/distributed-timer/main/.github/media/screenshot-1.png">
                <img src="https://raw.githubusercontent.com/dorianim/distributed-timer/main/.github/media/screenshot-1.png" width="500px" />
            </a>
        </td>
        <td align="center">
            <a href="https://raw.githubusercontent.com/dorianim/distributed-timer/main/.github/media/screenshot-2.png">
                <img src="https://raw.githubusercontent.com/dorianim/distributed-timer/main/.github/media/screenshot-2.png" width="500px" />
            </a>
        </td>
    </tr>
    <tr>
        <td align="center">
            <a href="https://raw.githubusercontent.com/dorianim/distributed-timer/main/.github/media/screenshot-3.png">
                <img src="https://raw.githubusercontent.com/dorianim/distributed-timer/main/.github/media/screenshot-3.png" width="500px" />
            </a>
        </td>
        <td align="center">
            <a href="https://raw.githubusercontent.com/dorianim/distributed-timer/main/.github/media/screenshot-4.png">
                <img src="https://raw.githubusercontent.com/dorianim/distributed-timer/main/.github/media/screenshot-4.png" width="500px" />
            </a>
        </td>
    </tr>
</table>

# Usage

You can use our publicly hosted instance free of charge: **[timer.itsblue.de](https://timer.itsblue.de)**

If you'd like to self-host, you can also use docker:

- 1. install docker and docker-compose on your system (follow the [official guide](https://docs.docker.com/engine/install/#server))
- 2. create the folder `/opt/distributed-timer`
- 3. create the file `/opt/distributed-timer/docker-compose.yml` with this content:

  ```yaml
  services:
    timer:
      image: ghcr.io/dorianim/distributed-timer:latest
      environment:
        JWT_KEY: some-random-string
        REDIS_STRING: "redis://:@redis/0"
      ports:
        - 3000:3000
      depends_on:
        - redis

    redis:
      image: docker.io/redis
      volumes:
        - ./redis-data:/data
  ```

  Make sure, to replace the JWT_KEY.

- 4. start the container: `docker-compose up -d`

# Build

## binary

To build a binary, you need:

- cargo >= 1.67.1
- npm >= 8.19.2

To build, run `cargo build --release` in the repo.

## docker image

To build the docker image, simply run `docker build . -t boulder-timer` in the repo.
