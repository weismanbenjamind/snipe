![Version](https://img.shields.io/badge/version-0.1.0-blue)

# Snipe

Snipe is a lightweight, fast, precise CLI HTTP client. The idea of the tool is to configure HTTP requests in a `.toml` file then use the CLI tool to make requests and tweak output/formatting options.

## Quickstart

By default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example configuration file is shows below:

```toml
[vars]  # Reusable variables
github_base_url = "https://api.github.com"

[targets.create-gist]  # Create an API request with id 'create-gist'
name = "Github Gist"
method = "POST"
url = "${VARS.github_base_url}/gists"  # Reuse the variable github_base_url from [vars]

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
# Outputs: create-gist
snipe list-targets
```

To make the `create-gist` request run:
```sh
# Outputs the response body
snipe shoot --target create-gist
```


## Example Usage

Below are some more detailed examples of `snipe` usage. Note - help for these setting can seen by simply using the `--help` command on the `snipe` CLI. For example:

```sh
snipe --help  # Help for snipe
snipe list-targets --help  # Help for the `list-targets` command
snipe shoot --help  # Help for `shoot` command - Note this is the command used for actually making HTTP requests
```

### Sending Requests and Formatting Output

By default snipe will display the response body to `stdout`. However, flags can be passed for status code, body, headers, full, and int status code to customize the output. There are some constraints with which combos can be passed together which are shown below.

#### Grabbing Specific Response Components

`snipe` allows for grabbing specific components from a response. Some response components cannot be grabbed at the same time. Examples below.

```sh
# Valid args
snipe shoot --target request-id-from-cfg --status-code  # Only status code (e.g. 200 OK)
snipe shoot --target request-id-from-cfg --headers  # Only headers
snipe shoot --target request-id-from-cfg --body  # Only body (default)
snipe shoot --target request-id-from-cfg --int-status-code  # Status code integer. (e.g just 200)
snipe shoot --target request-id-from-cfg --full  # Status code (e.g. 200 OK), headers, body
snipe shoot --target request-id-from-cfg --status-code --headers  # Status code (e.g. 200 OK) and headers
snipe shoot --target request-id-from-cfg --status-code --body  # Status code (e.g. 200 OK) and body
snipe shoot --target request-id-from-cfg --headers --body  # Headers and body

# Invalid args
snipe shoot --target request-id-from-cfg --status-code --full  # ERROR! => --full cannot be passed with any other flags
snipe shoot --target request-id-from-cfg --status-code --int-status-code  # ERROR! => --int-status-code cannot be passed with any other flags
```


#### Tweaking response format

The `--format` argument allows for changing response output format. Currently there are two options `http` (default) and `json`. The `http` return the output as an `http` string in the format:

```sh
status_code
headers

body
```

Combos will follow a similar format. For example `snipe shoot --target request-id-from-cfg --status-code --body` will return:

```sh
status_code

body
```

While the combo `snipe shoot --target request-id-from-cfg --status-code --headers` will return:

```sh
status_code
headers
```

***Note - if the the response cannot be parsed into a string the snipe will emit an error.***

The `json` format will attempt to parse the response into a json string. The output is formatted something like:

```json
{
  "status_code": "<response_status_code>",
  "headers": "<response_headers>",
  "body": "<response_body>",
}
```

Similarly - combos can be used. For example `snipe shoot --target request-id-from-cfg --status-code --body --format json` will return:

```json
{
  "status_code": "<response_status_code>",
  "body": "<response_body>",
}
```

While the combo `snipe shoot --target request-id-from-cfg --status-code --headers --format json` will return:

```json
{
  "status_code": "<response_status_code>",
  "headers": "<response_headers>",
}
```

***Note - if the the response cannot be parsed into a json string snipe will emit an error.***

Parsing to json also has a `--pretty` flag which can be used to make the output more readable. `--pretty` is not valid with `--format http` and if this combination is passed snipe will emit an error. An examples of using `--pretty` are shown below:

```sh
# Valid args
snipe shoot --target request-id-from-cfg --full --format json --pretty

# Invalid args
snipe shoot --target request-id-from-cfg --full --format http --pretty # ERROR! => Can't pass --format http with --pretty
snipe shoot --target request-id-from-cfg --full --pretty # ERROR! => By default snipe uses --format http which is invalid with --pretty
```

Lastly, `snipe` can output a response to a file using the `--output` argument. An example of writing a full response in pretty printed json to a file is shown below:

```sh
snipe shoot --target request-id-from-cfg --full --format json --pretty --output-file full-response.json
```

### Changing the Path to the Configuration File

Use the the `--config` (`-c`) argument to change the path of the configuration file. As stated above by default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example of using a different config looks something like the following:

```sh
snipe --config ~/.config/snipe/snipe_targets.toml shoot --target request-id-from-cfg
```

If the config cannot be found, snipe will fall back to attempting to read the `SNIPE_TARGETS` environment variable which houses the path to the config. The environment variable to look for can be tweaked with the `--cfg-env` (`-e`) arg. Searching for the environment variable can be skipped entirely by passing `skip` for this argument. For example:

```sh
snipe --cfg-env SNIPE_CONFIG shoot --target request-id-from-cfg  # If can't find the config at ./.snipe_targets.toml use the value at the environment variable SNIPE_CONFIG
snipe --config ~/.config/snipe/snipe_targets.toml --cfg-env SNIPE_CONFIG shoot --target request-id-from-cfg  # If can't find the config at ~/.config/snipe/snipe_targets.toml use the value at the environment variable SNIPE_CONFIG
snipe --cfg-env skip shoot --target request-id-from-cfg  # Skip an environment variable to identify the config
```

### Verbosity

Use `--verbose` (`-v`) for verbose mode. This mode will set the log level to info. Use `-vv` to set the log level to debug. Note, any number of `v` arguments can be passed. However, two or more arguments will simply set the log level to debug. For example:

```sh
snipe --verbose shoot --target request-id-from-cfg  # Log level at info
snipe -v shoot --target request-id-from-cfg  # Log level at info
snipe -vv shoot --target request-id-from-cfg  # Log level at debug
snipe -vvv shoot --target request-id-from-cfg  # Log level at debug
snipe -vvvvv shoot --target request-id-from-cfg  # Log level at debug
```
