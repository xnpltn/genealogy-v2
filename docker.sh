#!/bin/bash

sudo docker run  -v ~/.bun/bin:/root/.bun/bin -v $(pwd):/gene -v ~/.docker-debian:/root/.cargo/registry -it deb-env:rs bash


## export PATH=$PATH:/root/.bun/bin:/root/.cargo/bin:/root/.nvm/bin
