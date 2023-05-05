#!/bin/sh

# Script for updating version number across the code base

# Check pre-conditions
if [ $# != 1 ]; then
  echo 1>&2 "$0: Aborting. Missing argument: new version number"
  exit 2
fi


# Change webcomponents/package.json
OLD_VER=`awk -F ":" '/"version"/ {print $2}' ./webcomponents/package.json | sed 's/"//g' | sed 's/,//g' | sed 's/ //g'`
echo "./webcomponents/package.json $OLD_VER -> $1"
sed -i "s/\"version\": \"$OLD_VER\"/\"version\": \"$1\"/" ./webcomponents/package.json

# Change webapp/package.json
OLD_VER=`awk -F ":" '/"version"/ {print $2}' ./webapp/package.json | sed 's/"//g' | sed 's/,//g' | sed 's/ //g'`
echo "./webapp/package.json $OLD_VER -> $1"
sed -i "s/\"version\": \"$OLD_VER\"/\"version\": \"$1\"/" ./webapp/package.json
