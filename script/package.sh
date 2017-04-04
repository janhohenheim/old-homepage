#!/bin/sh
PWD=$(pwd)

rpmbuild -ba package.spec \
  --define "_sourcedir $PWD" \
  --define "_rpmdir $PWD/rpm"

