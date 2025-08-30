import { hc } from 'hono/client'
import { Hono } from 'hono'
import type app from './app'

// 😃
const client = hc<typeof app>('http://localhost') // Typed correctly

const res = await client.main.$get()

res.status