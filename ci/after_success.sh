set -ex

if [ "$TRAVIS_PULL_REQUEST" = 'false' ]; then
    echo "User-agent: *\nDisallow: /" > book/robots.txt
    ghp book
    set +x
    git push -fq "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages && echo OK
    set -x
fi
