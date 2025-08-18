import {swagger} from '@elysiajs/swagger';
import { Elysia, t } from 'elysia'
import { noteApp } from './modules/note/router';

const app = new Elysia()
  .use(swagger())
  .get('/', ({ path }) => path) 
  .post('/hello', 'Do you miss me?')
  .get('null', ({status}) => status(204))
  .use(noteApp)
  .listen(3000)

console.log(
  `Elysia is running at http://${app.server?.hostname}:${app.server?.port}`
);
