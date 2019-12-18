#!/usr/bin/env bash

BASEDIR=$(dirname "$0")
mkdir /opt/tower
echo coping tower bin...
cp $BASEDIR/../tower /opt/tower
echo coping config...
cp -r $BASEDIR/../config /opt/tower
echo coping tower.desktop file...
cp $BASEDIR/tower.desktop /usr/share/applications

