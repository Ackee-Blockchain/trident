#!/bin/bash

if [[ -z "${DOMAIN_ZONE_ID}" ]]; then
    echo -e "\033[0;31mERROR: Variable DOMAIN_ZONE_ID is missing, set it up in CI settings.\033[0m" >&2
    exit 1
fi

if [[ -z "${DOMAIN_TOKEN}" ]]; then
    echo -e "\033[0;31mERROR: Variable DOMAIN_KEY and DOMAIN_TOKEN are missing, set one of it in CI settings.\033[0m" >&2
    exit 1
fi

SITEMAP_URL="https://ackee.xyz/trident/docs/latest/sitemap.xml"
PURGE_URLS=$(curl -s $SITEMAP_URL | grep -Eo '<loc>[^<]*' | cut -d '>' -f 2)

PURGE_URLS+=" https://ackee.xyz/trident/docs/index.html"
PURGE_URLS+=" https://ackee.xyz/trident/docs/latest"
PURGE_URLS+=" https://ackee.xyz/trident/docs/versions.json"
PURGE_URLS+=" https://ackee.xyz/trident/docs/latest/search/search_index.json"

for url in $PURGE_URLS
do
    echo "Purgin cache for ${url}"
    RESPONSE=$(curl -X POST "https://api.cloudflare.com/client/v4/zones/${DOMAIN_ZONE_ID}/purge_cache" \
         -H "Authorization: Bearer ${DOMAIN_TOKEN}" \
         --data "{\"files\":[\"${url}\"]}")
    SUCCESS=$(echo $RESPONSE | jq -r '.success')
    if [[ "$SUCCESS" != "true" ]]; then
        echo "Failed to purge $url. Response: $RESPONSE"
        exit 1
    fi
done
