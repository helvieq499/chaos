// ==UserScript==
// @name        Chaos through Discord
// @match       https://helvieq499.github.io/*
// @match       https://discord.com/stop-blocking-on-origin
// @grant       none
// @version     1.0
// @run-at      document-start
// ==/UserScript==

// Userscripts can be dangerous
//   so this file is to remain documented
//   such that anyone could understand what it is doing

// This section is to correct broken URLs because of a static deployment

window.addEventListener("load", () => {
  if (document.location.host == "helvieq499.github.io") {
    // If you are not on the root (/chaos) then move to it
    if (document.location.pathname != "/chaos/") {
      document.location = "https://helvieq499.github.io/chaos/";
    }
  }
});

// This section is to proxy our requests through a Discord tab

// BUG: this is required or it breaks
window.addEventListener("load", () => {
  if (document.location.href == "https://helvieq499.github.io/chaos/") {
    // Constant for Chaos to detect
    window.CHAOS_THROUGH_DISCORD_VERSION = 1;

    // Create a map of requests that have yet to get a response
    const requests = {};

    // Listen to responses from the subtab (Discord)
    window.addEventListener("message", (event) => {
      if (event.origin != "https://discord.com") return;

      // Retrieve the resolve and reject methods to end the original Promise
      const [res, rej] = requests[event.data[1]];
      if (event.data[0] == "resolve") {
        const data = event.data[2];

        // Reconstruct the response
        let response = new Response(data.body, {
          status: data.status,
          statusText: data.statusText,
          headers: new Headers(data.headers),
        });

        // Hack to bypass wasm-bindgen dying on an empty string
        response.__defineGetter__("url", () => data.url);

        res(response);
      } else rej(event.data[2]);
    });

    // Open the sub-tab (Discord) to run the rest of the script
    const subtab = window.open("https://discord.com/stop-blocking-on-origin");

    // Hook into `fetch` to proxy Discord requests
    ((original) =>
      (window.fetch = function (req, options) {
        // Chaos seems to only send Requests and not strings
        if (req instanceof Request && req.url.includes("discord.com")) {
          const id = (Math.random() * Number.MAX_SAFE_INTEGER) >>> 0;

          // Serialize the request
          req.arrayBuffer().then((buffer) => {
            subtab.postMessage(
              {
                id,
                url: req.url,
                method: req.method,
                headers: [...req.headers.entries()],
                body: buffer,
                keepalive: req.keepalive,
              },
              "https://discord.com"
            );
          });

          // Return a Promise that will be resolved when we complete a round-trip with the subtab
          return new Promise((res, rej) => (requests[id] = [res, rej]));
        } else original.apply(this, arguments);
      }))(window.fetch);
  } else if (document.location.host == "discord.com") {
    if (!window.opener) return;

    // Clear the page to put our message on it
    document.close();
    try {
      document.write(`
        <title>Chaos through Discord</title>
        <style>body { background-color: #35393e; color: white; }</style>
        <h1>Please keep this tab open.</h1>
        <h2>This tab is being used to make Chaos work</h2>
        <h3>It will close when it is no longer needed</h3>
        <h4>This is required because Discord intentionally prevents iframes and cross-origin requests from working</h4>
      `);
      document.close();
    } catch {
      // TODO: Firefox prevents document.open, an alternative way to show the message is needed
    }

    // Close this sub-tab (Discord) if the parent (Chaos) is closed
    setInterval(() => {
      if (window.opener == null || window.opener.closed) window.close();
    }, 200);

    window.addEventListener("message", (event) => {
      if (event.origin != "https://helvieq499.github.io") return;

      // Remove the body if the request cannot support it
      let body = event.data.body;
      if (event.data.method == "GET" || event.data.method == "HEAD") {
        body = undefined;
      }

      // Perform the original (Chaos) request in the subtab (Discord)
      fetch(event.data.url, {
        method: event.data.method,
        headers: new Headers(event.data.headers),
        body,
        keepalive: event.data.keepalive,
      })
        // Send the result back to Chaos
        .then((resp) =>
          resp.arrayBuffer().then((buffer) => {
            window.opener.postMessage(
              [
                "resolve",
                event.data.id,
                {
                  url: resp.url,
                  body: buffer,
                  status: resp.status,
                  statusText: resp.statusText,
                  headers: [...resp.headers.entries()],
                },
              ],
              "https://helvieq499.github.io"
            );
          })
        )
        // Send the error back to Chaos
        .catch((err) =>
          window.opener.postMessage(
            ["reject", event.data.id, err],
            "https://helvieq499.github.io"
          )
        );
    });

    // This should never happen but might as well have a check for it
  } else
    console.error(
      `Script ran on the wrong location: ${document.location.href}`
    );
});
