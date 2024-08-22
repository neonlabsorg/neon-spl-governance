#!/bin/bash
set -euo pipefail

docker images
while getopts u:p:b:t: option
do
  case "${option}" in
    u) user=${OPTARG};;
    p) password=${OPTARG};;
    b) branch=${OPTARG};;
    t) tag=${OPTARG};;
    *) echo -e "usage: $0 \n [-u] docker hub user \n [-p] docker hub password \n [-b] current branch \n [-t] governance-cli tag" >&2
       exit 1 ;;
  esac
done

if [[ $branch == "main" ]]; then
    TAG="stable"
else
    TAG=$branch
fi

docker login -u=$user -p=$password
docker push neonlabsorg/neon-governance:$tag

if [[ $TAG == "stable" ]] || [[ $TAG == ci-* ]] || [[ $TAG == v*.*.* ]]; then
    docker tag neonlabsorg/neon-governance:$tag neonlabsorg/neon-governance:${TAG}
    docker push neonlabsorg/neon-governance:${TAG}
fi

exit 0