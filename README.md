# Snipe

Snipe is a lightweight, fast, precise CLI HTTP client. The idea of the tool is to configure HTTP requests in a `.toml` file then use the CLI tool to make requests and tweak output/formatting options.

## Quickstart

By default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example of this file might look something like the following:

```toml
[vars]  # Reusable variables
github_base_url = "https://api.github.com"

[targets.create-gist]  # Create an API request with id "create-gist"
name = "Github Gist"
method = "POST"
url = "${VARS.GITHUB_BASE_URL}/gists"  # Reuse the variable github_base_url from [vars]

[targets.create-gist.headers]  # Headers for 'create-gist' request
User-Agent = "snipe"

[targets.create-gist.auth]  # Auth for 'create-gist' request
scheme = "bearer"
token = "${ENV.GITHUB_PAT}"  # Use the value stored at the GITHUB_PAT env var for the API token

[targets.create-gist.payload]  # Payload for 'create-gist` request
description = "Test gist"
public = false
files = {"test.txt" = {content = "Testing Gist"}}
```

Once the configuration file is created in your present working directory the following command will show the potential HTTP requests to make:

```sh
# Outputs:
# create-gist
snipe list-targets
```

To make the Create Gist request run:
```sh
# Outputs:
# Response body
snipe shoot --target create-gist
```


## Example Usage

Below are some more detailed examples of `snipe` usage. These examples assume we are using the `.toml` configuration file example in the `Quickstart` section. Note - a deeper dive into these setting can seen by simply using the `--help` command on the `snipe` CLI. For example:

```sh
snipe --help  # Help for snipe
snipe list-targets --help  # Help for list-targets command
snipe shoot --help  # Help for shoot command - note this is the command used for actually making HTTP requests
```

### Changing the Path to the Configuration File

Use the the `--config` (`-c`) argument to change the path of the configuration file. As stated above by default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example of using a different config might look like the following:

```sh
snipe --config ~/.config/snipe/snipe_targets.toml shoot --target create-gist
```

If the config cannot be found, snipe will fall back to attempting to read a `SNIPE_TARGETS` environment variable which houses the path to the config. The environment variable to look for can be tweaked with the `--cfg-env` (`-e`) flag. Searching for the environment variable can be skipped entirely by passing `skip` for this argument. For example:

```sh
snipe --cfg-env SNIPE_CONFIG shoot --target create-gist  # If can't find the config at the value specified by --config use the value at the environment variable SNIPE_CONFIG
snipe --cfg-env skip shoot --target create-gist  # Skip using environment variables to identify the config
```

### Verbosity

Use `--verbose` (`-v`) for verbose mode. This mode will set the log level to info. Use `-vv` to set the log level to debug. Note, any number of `v` arguments can be passed. However, two or more arguments will simply set the log level to debug. For example:

```sh
snipe --verbose shoot --target create-gist  # Log level at info
snipe --v shoot --target create-gist  # Log level at info
snipe --vv shoot --target create-gist  # Log level at debug
snipe --vvv shoot --target create-gist  # Log level at debug
snipe --vvvvv shoot --target create-gist  # Log level at debug
```

### Formatting output

By default snipe will display the response body to `stdout`. However, flags can be passed for status code, body, headers, full, and int status code to customize the output. There are some constraints and which combos can be passed together.

#### Grabbing Specific Response Components

`snipe` allows for grabbing specific components of a response. Some response components cannot be grabbed at the same time. Examples below.

```sh
snipe shoot --target create-gist --status-code  # Only status code (e.g. 200 OK)
snipe shoot --target create-gist --headers  # Only headers
snipe shoot --target create-gist --body  # Only body (default)
snipe shoot --target create-gist --int-status-code  # Status code (e.g. 200 OK) integer. E.g. just 200
snipe shoot --target create-gist --full  # Status code (e.g. 200 OK), headers, body
snipe shoot --target create-gist --status-code --headers  # Status code (e.g. 200 OK) and headers
snipe shoot --target create-gist --status-code --body  # Status code (e.g. 200 OK) and body
snipe shoot --target create-gist --headers --body  # Headers and body
snipe shoot --target create-gist --status-code --full  # ERROR! => --full cannot be passed with any other flags
snipe shoot --target create-gist --status-code --int-status-code  # ERROR! => --int-status-code cannot be passed with any other flags
```


#### Tweaking response format

The `--format` argument allows for changing response output format. Currently there are two options `http` (default) and `json`. `http` return the output as an `http` string in the format:

```sh
status_code
headers

body
```

Combos will follow a similar format. For example `snipe shoot --target create-gist --status-code --body` will return:

```sh
status_code

body
```

While the combo `snipe shoot --target create-gist --status-code --headers` will return:

```sh
status_code
headers
```

***Note - if the the response component requested cannot be parsed a string the snipe will emit an error***

The `json` format will attempt to parse the response into a json string. The output is formatted something like:
```json
{
  "status_code": "<response_status_code>",
  "headers": "<response_headers>",
  "body": "<response_body>",
}
```

Simlarly - combos can be use. For example `snipe shoot --target create-gist --status-code --body --format json` will return:
```json
{
  "status_code": "<response_status_code>",
  "body": "<response_body>",
}
```

While the combo `snipe shoot --target create-gist --status-code --headers --format json` will return:
```json
{
  "status_code": "<response_status_code>",
  "headers": "<response_headers>",
}
```

***Note - if the the response component requested cannot be parsed into json the snipe will emit an error***

Parsing to json also has a `--pretty` flag which can be used to make the output more readable. __PICK UP HERE__
