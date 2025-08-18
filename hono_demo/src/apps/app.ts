import openapi from './openapi/router'

const routers = [
  openapi,
] as const;

import { OpenAPIHono, createRoute, z } from '@hono/zod-openapi'

const app = new OpenAPIHono()

app.openapi(createRoute({
  method:'get', path:'/', 
  responses: {
    200: { 
      content: {'text/plain': { schema: z.string() }},
      description: 'Root path response'
    },
  }
}), 
  ({req, text}) => text(req.path))


export app