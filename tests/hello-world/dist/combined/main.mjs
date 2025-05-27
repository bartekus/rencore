// This file was bundled by Encore v0.0.0-develop+74f891c2c940382008e854d72405de10296d00f6-modified
//
// https://encore.dev

// tests/hello-world/.encore/build/combined/combined/main.mjs
import { registerGateways, registerHandlers, run } from "encore.dev/internal/codegen/appinit";
import { api } from "encore.dev/api";
import { Service } from "encore.dev/service";
var get = api(
  { expose: true, method: "GET", path: "/hello/:name" },
  async ({ name }) => {
    const msg = `Hello ${name}!`;
    return { message: msg };
  }
);
var encore_service_default = new Service("hello");
var gateways = [];
var handlers = [
  {
    apiRoute: {
      service: "hello",
      name: "get",
      handler: get,
      raw: false,
      streamingRequest: false,
      streamingResponse: false
    },
    endpointOptions: { "expose": true, "auth": false, "isRaw": false, "isStream": false, "tags": [] },
    middlewares: encore_service_default.cfg.middlewares || []
  }
];
registerGateways(gateways);
registerHandlers(handlers);
await run(import.meta.url);
//# sourceMappingURL=main.mjs.map
