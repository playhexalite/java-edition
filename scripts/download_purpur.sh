#!/bin/bash

DIRNAME=$(dirname "$(realpath "$0")")
cd "$DIRNAME/../run"
rm -f purpur.jar
curl https://api.purpurmc.org/v2/purpur/1.19/latest/download -o ./purpur.jar
