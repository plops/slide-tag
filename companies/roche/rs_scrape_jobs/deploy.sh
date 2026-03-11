#!/bin/bash
# Optimize for Hetzner server
# Usage: ./build.sh 

set -e
./build_release.sh
echo "Deploying $@..."
rsync -avz target/release/rs-scrape rhetzneruso:/var/www/jobs.rocketrecap.com/
echo "Restarting service..."
ssh rhetzneruso "systemctl restart jobs-rocketrecap.service"
