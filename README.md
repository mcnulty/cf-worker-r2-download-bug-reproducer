This repository contains the source for a Rust Cloudflare worker that
implements an API to download a large (~200 MB) file from R2 that results
in the worker exceeding its CPU time limit. The same worker implemented in
TypeScript/JavaScript does not exceed the CPU time limit.

## Reproducing

The following steps can be used to reproduce the bug. They assume that you
have already set up your environment with `npm`, `wrangler`, and `rclone`,
and the corresponding set up in your Cloudflare account.

- Create a R2 bucket in your Cloudflare account called `r2-download-bug`
- Initialize the worker
  - `npm install`
- Generate the test file using `npm run gen-test-file`
- Upload the test file to the bucket
  - `rclone copy --progress bucket-src/200MB-file r2:r2-download-bug/`
- Deploy the worker
  - `npm run deploy`
- Open an additional terminal to tail the worker logs:
  - `npx wrangler tail`
- In another terminal, download the file with curl:
  - `curl -v https://r2-download-bug.$DOMAIN.workers.dev/200MB-file > ./200MB-file`

After this last step, you should see the following:
- The downloaded file is not the correct size. It will be less than expected due to the
  CPU limit being exceeded.
- The wrangler logs will say something like:

```
GET https://r2-download-bug.$DOMAIN.workers.dev/200MB-file - Exceeded CPU Limit @ 9/20/2023, 10:40:04 AM
✘ [ERROR]   Error: Worker exceeded CPU time limit.
```

## Working Locally

The `wrangler.toml` is set up to work locally, but first, the local R2 bucket needs
to be initialized. To do that, run the following:

```
npm run gen-test-file
npm run setup-local-r2
```
