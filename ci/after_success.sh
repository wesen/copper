set -ex

. $(dirname $0)/env.sh

if [[ $LINUX && $TRAVIS_PULL_REQUEST == false ]]; then
    ./ghp-import/ghp_import.py book
    set +x
    git push -fq https://$GH_TOKEN@github.com/$TRAVIS_REPO_SLUG.git gh-pages && echo OK
    set -x
fi
