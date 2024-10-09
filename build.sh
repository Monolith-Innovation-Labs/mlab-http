#!/bin/bash

stalker="/media/data/anomaly/Anomaly-1.5.3-Full.2"

echo "" > $stalker/gamedata/configs/crc_input.txt
echo "" > $stalker/gamedata/configs/crc_output.txt

mkdir -p $stalker/gamedata/scripts
cp -rf ./gamedata/scripts/* $stalker/gamedata/scripts

(cd app ; cargo build)

cp ./app/target/debug/mlab_client $stalker/mlab_client
chmod +x $stalker/mlab_client
(cd $stalker ; ./mlab_client)
