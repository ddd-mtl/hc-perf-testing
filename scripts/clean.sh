#!/bin/bash

# TOP LEVEL
rm -rf .hc*
rm -rf target
# WEBCOMPONENTS
rm -rf webcomponents/dist
rm -rf webcomponents/src/generated
rm webcomponents/tsconfig.tsbuildinfo
# WEBAPP
rm -rf webapp/dist/
rm -rf webapp/out-tsc/
rm webapp/tsconfig.tsbuildinfo
