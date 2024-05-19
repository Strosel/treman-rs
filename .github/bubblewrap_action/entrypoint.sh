#!/bin/sh -l

echo "========================= Changing directory to $1 ========================="
cd $1
echo "========================= Building APK, it may take 3-4 minutes or more ========================="
( sleep 5 && while [ 1 ]; do sleep 5; echo y; done ) | bubblewrap build --skipSigning
cp ./*.apk ..
echo "========================= APK building finished ========================="
