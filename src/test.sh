#!/bin/bash

set -o pipefail  # Exit on pipe command errors

# Function to update .env file with .env.sample
update_env() {
    local env_path=$1
    local env_sample_path=$2

    cp $env_path "${env_path}.bak"  # backup original .env file

    while IFS= read -r line || [[ -n "$line" ]]
    do
        if [[ $line =~ ^#.*$ || $line == "" ]]; then  # if comment or empty line
            echo "$line" >> "${env_path}.new"
        else
            key=$(echo "$line" | cut -d '=' -f 1)  # extract the key
            # search for the key in .env file
            value=$(grep "^$key=" "${env_path}.bak")
            if [[ $value ]]; then
                echo "$value" >> "${env_path}.new"  # use value from .env if key exists
            else
                echo "$line" >> "${env_path}.new"  # use default value from .env.sample
            fi
        fi
    done < "$env_sample_path"

    mv "${env_path}.new" "$env_path"  # replace original .env file with the new one
}

# Usage
update_env ".env" ".env.sample"
