#!/bin/bash

# Navigate to the directory this script is contained within, in case it is called from elsewhere.
cd $(dirname $0)

# Clone the docs branch into ./xrb-docs
git clone --branch docs https://github.com/XdotRS/xrb ./xrb-docs

# Regenerate documentation.
rm -r ./target/doc
cargo doc --no-deps --document-private-items

# Replace the `doc` folder with the newly generated docs
rm -r ./xrb-docs/doc
cp -r ./target/doc ./xrb-docs/

# Commit and push the newly generated docs to the docos branch
cd ./xrb-docs
git add -A
git commit --allow-empty --quiet -m "regenerated documentation"
git push origin docs
cd ..

# Remove the cloned docs branch repo
rm -rf ./xrb-docs
