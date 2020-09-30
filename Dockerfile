FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.1

RUN apt-get update && apt-get -y install libsdl2-dev libsdl2-ttf-dev
