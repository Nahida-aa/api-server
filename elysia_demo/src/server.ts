import { swagger } from '@elysiajs/swagger';
import { Elysia } from 'elysia';
import { communityApp } from './api/community/router';
import { wsApp } from './api/ws/router';

const app = new Elysia()
  .use(swagger())
  .get('/chat', () => Bun.file('src/public/chat.html'))
  .use(wsApp)
  .use(communityApp)
  .listen(3000)

console.log(
  `Elysia is running at http://${app.server?.hostname}:${app.server?.port}`
);

export type App = typeof app 