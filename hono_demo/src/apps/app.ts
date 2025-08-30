import { Hono } from 'hono'
import { hc } from 'hono/client'
import { createMiddleware } from 'hono/factory'
import { logger } from 'hono/logger'
import { requestId } from 'hono/request-id'

const demoMiddleware = createMiddleware(async (c, next) => {
	console.log('called sub1')
  console.log('sub1 middleware:', c.req.path)
	await next()
})

const aLoggerHono = () => {
  return createMiddleware(async (c, next) => {
    const reqHeader = c.req.header()
    console.log(reqHeader)
    // const reqContentType = c.req.header('content-type') || ''
    // let reqBody: any = null
    // // 根据 content-type 解析请求体
    // // 注意：如果是 multipart/form-data，则需要使用 formData() 方法
    // // 这里为了演示，假设只处理 json、x-www-form-urlencoded 和 text/plain
    // switch (true) {
    //   case reqContentType.includes('application/json'):
    //     reqBody = await c.req.json()
    //     break
    //   case reqContentType.includes('application/x-www-form-urlencoded'):
    //     reqBody = await c.req.formData()
    //     break
    //   case reqContentType.includes('text/'):
    //     reqBody = await c.req.text()
    //     break
    //   default:
    //     // multipart/form-data 或 流 类型，不解析
    //     console.log('Skipping logging body for content-type:', reqContentType)
    // }
    // console.log(JSON.stringify(reqBody, null, 2))

    await next()

    console.log(c.res.headers)
  })
}

const subRouter1 = new Hono().basePath('/sub1')
.get('', (c) => c.text('Hello from sub1 router!'))
subRouter1.use(demoMiddleware)
subRouter1.get('/2', (c) => c.text('Hello from sub1 router!'))

const subRouter2 = new Hono()
.get('', (c) => c.text('Hello from sub2 router!'))


const app = new Hono()
.use(requestId())
.use(logger())
.use(aLoggerHono())
.get('/main', (c) => c.text('Hello from main!'))
.get('/401', (c) => {
  return c.json({ message: 'Unauthorized' }, 401)
})
.post('/401', async (c) => {
  const body = await c.req.json()
  console.log('Received body:', body)
  return c.json({ message: 'Unauthorized' }, 401)
})
.use(demoMiddleware)
.get('/main2', (c) => c.text('Hello from main!'))
.get('/err', (c) => {
  throw new Error('This is an error from main route')
})
.route('', subRouter1) // 由于 Hono 的路由匹配是从上到下的，因此这里的空字符串会匹配到所有路径，会经过 subRouter1 的中间件
.route('/sub2', subRouter2);

export default app

