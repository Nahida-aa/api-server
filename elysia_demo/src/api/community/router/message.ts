import { Elysia, t } from 'elysia';

const messageBody = t.Object({
  channelId: t.String(),
  content: t.String(),
  contentType: t.String(),
  replyToId: t.Optional(t.String()),
  attachments: t.Optional(t.Array(t.Object({
    id: t.String(),
    name: t.String(),
    url: t.String(),
    type: t.String(),
    size: t.Number()
  })))
})
export const communityChannelMessageApp = new Elysia()
  .post('/community/message', ({ body }) => { }, {
    // validate incoming message
    body: messageBody
  })