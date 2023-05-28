#!/usr/bin/env bash

export REPO=ll_stack

set -Eeuo pipefail

# ./build.sh checks the code and exectures cargo doc
./build.sh
# remove the old docs and replace by new docs
git rm -r docs || echo "Apparently first run .. ignoring error"
mv target/doc docs
echo '<meta http-equiv="refresh" content="0; url=$REPO/index.html">' > docs/index.html
echo "" > docs/.nojekyll
cat >  docs/_config.yml <<EOF
title: Cayman theme
description: Cayman is a clean, responsive theme for GitHub Pages.
show_downloads: true
google_analytics:
theme: jekyll-theme-cayman
EOF
mkdir -p docs/assets/css
cat > docs/assets/css/style.scss <<EOF
---
---

@import 'jekyll-theme-cayman';
EOF
git add docs
git add _config.yml
git add docs/assets/css/style.scss

cat <<EOF
Please commit and push the changes by executing:
git commit -m "Updated documents"
git push
EOF
