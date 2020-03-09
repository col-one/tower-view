#!/usr/bin/env bash

BASEDIR=$(dirname "$0")
mkdir /opt/TowerView
echo coping towerview bin...
cp $BASEDIR/../towerview /opt/TowerView
echo coping config...
cp -r $BASEDIR/../config /opt/TowerView
echo coping towerview.desktop file...
cp $BASEDIR/towerview.desktop /usr/share/applications

