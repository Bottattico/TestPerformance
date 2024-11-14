# Test Results

This directory contains the results of load tests conducted with the `hey` tool to evaluate the server's performance under different request loads.

## Folder Structure

- **Folder `1000`**: Contains the results of the load test with 1000 requests.
- **Folder `10000`**: Contains the results of the load test with 10000 requests.

## Test Information

The tests were executed using the following `hey` command, adjusting the number of requests (`-n`) to achieve the desired results:

```sh
hey -n <number_of_requests> http://localhost:8080
```
