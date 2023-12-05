import { HttpRequest, HttpResponse, Kv, Router } from "@fermyon/spin-sdk"

let router = Router();

router.post("/data", async (_req, extra) => {
  let key = "advent";
  let store = Kv.openDefault();

  store.set(key, extra.body);

  return { status: 201 }
});

router.get("/data", async (_req, _extra) => {
  let key = "advent";
  let store = Kv.openDefault();

  let data = store.get(key);

  return {
    status: 200,
    headers: { "content-type": "application/json" },
    body: data
  }
});

export async function handleRequest(req: HttpRequest): Promise<HttpResponse> {
  return router.handleRequest(req, { body: req.body });
}

