# AWS Lambda Secrets Injector

A simple tool to inject secrets from AWS Secrets Manager into Lambda functions as environment variables

## Features

- retrieve secrets from AWS Secrets Manager
- save the secrets to a file in /tmp directory
- expose the secrets as environment variables using a wrapper script and delete the tmp file

## Usage

1. configure lambda environment variables with following syntax: 
   - ENV_VAR = "{{inject:secretsmanager:secret-id:SecretString:json-key}}"
   - *secret-id*: the ARN or name of the secret
   - *json-key*: The key name of the key-value pair whose value you want to retrieve
2. add the injector and a wrapper script to the lambda function as a layer
3. give the lambda function permission to read the secrets from AWS Secrets Manager

## Example

Checkout the simple python lambda example [here](examples/secret-injection-demo). 

## Status

This project to a demo. Do **NOT** use for production environment. It has a lot of room for improvement.