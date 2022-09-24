import { QueryClient } from '@tanstack/solid-query';
import { FetchTransport, createClient } from '@rspc/client';
import { createSolidQueryHooks } from '@rspc/solid';

import type { Operations } from "./bindings"; // These were the bindings exported from your Rust code!

// You must provide the generated types as a generic and create a transport (in this example we are using HTTP Fetch) so that the client knows how to communicate with your API.
export const rspcClient = createClient<Operations>({
  // Refer to the integration your using for the correct transport.
  transport: new FetchTransport("http://localhost:4001/rspc"),
});

export const queryClient = new QueryClient();
export const rspc = createSolidQueryHooks<Operations>();
