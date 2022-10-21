#!/bin/bash

echo "starting services ..."

echo "starting consumers..."
/opt/ideas/bin/qmailer &

echo "starting api..."
/opt/ideas/bin/api

#wait -n
#exit $?

