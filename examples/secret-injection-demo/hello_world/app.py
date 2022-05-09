import os
import json


def lambda_handler(event, context):

    return {
        "statusCode": 200,
        "body": json.dumps({
            "DB_USERNAME": os.getenv("DB_USERNAME"),
            "DB_PASSWORD": os.getenv("DB_PASSWORD"),
        }),
    }

