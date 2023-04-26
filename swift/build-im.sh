#!/bin/bash

while getopts "u:" opt; do
  case $opt in
    u)
      username=$OPTARG
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
    :)
      echo "Option -$OPTARG requires an argument." >&2
      exit 1
      ;;
  esac
done

rm -rf .build
rm -rf /Users/$(id -un)/Library/Input\ Methods/KhiinIM.app
rm -rf /Users/$(id -un)/Library/Containers/com.edwardgreve.inputmethod.KhiinIM
rm -rf /Users/$(id -un)/Library/Developer/Xcode/DerivedData/Khiin-*/

xcodebuild -scheme KhiinIM build CONFIGURATION_BUILD_DIR=$(pwd)/.build

if [ -z "$username" ]; then
  echo "For local testing, it is recommended to create a separate"
  echo "user account and pass in the \"-u [username]\" flag to move the"
  echo "IME application to that user's home folder for testing."
  echo "Otherwise, you will have to log out / log in every time you"
  echo "make a change."
  exit 0
fi

sudo rm -rf /Users/$username/Library/Input\ Methods/KhiinIM.app
sudo rm -rf /Users/$username/Library/Containers/com.edwardgreve.inputmethod.KhiinIM
sudo cp -R .build/KhiinIM.app /Users/$username/Library/Input\ Methods/
sudo chown -R $username /Users/$username/Library/Input\ Methods/KhiinIM.app

echo "Copied IME to \"/Users/$username/Library/Input Methods\""
